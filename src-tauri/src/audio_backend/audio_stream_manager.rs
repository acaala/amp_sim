use std::{
    mem::MaybeUninit,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, Stream, StreamConfig,
};
use ringbuf::{Consumer, HeapRb, Producer, SharedRb};

use super::{audio_device_manager::AudioDeviceManager, audio_pipeline::AudioPipeline};

pub struct AudioStreamManager {
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,
}

impl AudioStreamManager {
    pub fn new() -> Self {
        AudioStreamManager {
            input_stream: None,
            output_stream: None,
        }
    }

    pub fn run(
        &mut self,
        audio_device_manager: &AudioDeviceManager,
        audio_pipeline: Arc<Mutex<AudioPipeline>>,
    ) -> Result<(), Error> {
        let input_device = &audio_device_manager.input_device;
        let output_device = &audio_device_manager.output_device;

        println!("Input Device: {:#?}", input_device.name().unwrap());
        println!("Output Device: {:#?}", output_device.name().unwrap());

        let config: StreamConfig = input_device.default_input_config().unwrap().config();

        // Create a delay in case the input and output devices aren't synced.
        let latency_frames = (300.0 / 1_000.0) * config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * config.channels as usize;

        // The buffer to share samples
        let ring = HeapRb::<f32>::new(latency_samples * 2);
        let (mut producer, consumer) = ring.split();

        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }

        let input_stream =
            Self::get_input_stream(&input_device, producer).expect("Failed to get input stream");

        let output_stream = Self::get_output_stream(&output_device, consumer, audio_pipeline)
            .expect("Failed to get output stream");

        input_stream.play().expect("to play input stream");
        output_stream.play().expect("to play input stream");

        self.input_stream = Some(input_stream);
        self.output_stream = Some(output_stream);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.input_stream = None;
        self.output_stream = None;

        println!("Stopped stream");
        Ok(())
    }

    fn get_input_stream(
        input_device: &Device,
        mut producer: Producer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>,
    ) -> Result<Stream, anyhow::Error> {
        let config: StreamConfig = input_device.default_input_config().unwrap().into();
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                // eprintln!("output stream fell behind: try increasing latency");
            }
        };
        Ok(input_device.build_input_stream(&config, input_data_fn, Self::err_fn, None)?)
    }

    fn get_output_stream(
        output_device: &Device,
        mut consumer: Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>,
        audio_pipeline: Arc<Mutex<AudioPipeline>>,
    ) -> Result<Stream, anyhow::Error> {
        let config = output_device.default_output_config().unwrap().config();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => audio_pipeline.lock().unwrap().process_sample(s),

                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                // eprintln!("input stream fell behind: try increasing latency");
            }
        };

        Ok(output_device.build_output_stream(&config, output_data_fn, Self::err_fn, None)?)
    }

    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
}
