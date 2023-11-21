use crate::audio_backend::audio_pipeline::AudioPipeline;
use crate::audio_backend::processors::amplifier::Amplifier;
use crate::audio_backend::processors::screamer::ScreamerPedal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, DevicesError, Host, Stream, StreamConfig};
use dialoguer::Select;
use ringbuf::{Consumer, HeapRb, Producer, SharedRb};
use std::mem::MaybeUninit;
use std::sync::Arc;

pub struct AudioProcessor {
    // input_device: Arc<Device>,
    // output_device: Device,
    input_stream: Stream,
    output_stream: Stream,
}

impl AudioProcessor {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        // let input_device = host.default_input_device().unwrap();
        // let output_device = host.default_output_device().unwrap();
        let input_device = Self::get_input_device(&host).expect("Failed to get input device");
        let output_device = Self::get_output_device(&host).expect("Failed to get output device");

        let config: StreamConfig = input_device.default_input_config().unwrap().config();

        // Create a delay in case the input and output devices aren't synced.
        let latency_frames = (150.0 / 1_000.0) * config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * config.channels as usize;

        // The buffer to share samples
        let ring = HeapRb::<f32>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }

        let mut audio_pipeline = AudioPipeline::new();
        // audio_pipeline.add_processor(Box::new(Amplifier::new()));

        // let screamer_pedal = ScreamerPedal {
        //     overdrive: 2.0,
        //     tone: 0.8,
        //     level: 0.7,
        // };

        // audio_pipeline.add_processor(Box::new(screamer_pedal));

        let input_stream =
            Self::get_input_stream(&input_device, producer).expect("Failed to get input stream");
        let output_stream = Self::get_output_stream(&output_device, consumer, audio_pipeline)
            .expect("Failed to get output stream");

        println!("{:#?}", input_device.name());

        Ok(AudioProcessor {
            // input_device,
            // output_device,
            input_stream,
            output_stream,
        })
    }

    fn start_streams(&self) -> Result<(), anyhow::Error> {
        println!("Staring streams...");
        self.input_stream.play()?;
        self.output_stream.play()?;

        Ok(())
    }

    pub fn run(&self, duration_seconds: u64) {
        // Run for the specified duration.
        self.start_streams().expect("Failed to start streams");
        println!("Running for {}s", duration_seconds);
        // std::thread::sleep(std::time::Duration::from_secs(duration_seconds));
    }

    fn get_device<T, F>(host: &Host, get_devices: F, prompt: &str) -> Option<Device>
    where
        T: Iterator<Item = Device>,
        F: Fn(&Host) -> Result<T, DevicesError>,
    {
        let devices = match get_devices(host) {
            Ok(devices) => devices.collect::<Vec<_>>(), // Store devices in a Vec
            Err(e) => {
                println!("Error getting input devices: {:?}", e);
                return None;
            }
        };

        let device_names: Vec<String> = devices.iter().filter_map(|dev| dev.name().ok()).collect();

        let selection = Select::new()
            .with_prompt(prompt)
            .default(0)
            .items(&device_names)
            .interact()
            .unwrap();

        // Use the stored devices to find the selected device
        devices.into_iter().find(|device| match device.name() {
            Ok(name) => name == device_names[selection],
            Err(_) => {
                println!("Error getting device name");
                false
            }
        })
    }

    pub fn get_input_device(host: &Host) -> Option<Device> {
        Self::get_device(host, Host::input_devices, "Select the input device")
    }

    fn get_output_device(host: &Host) -> Option<Device> {
        Self::get_device(host, Host::output_devices, "Select the output device")
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
                eprintln!("output stream fell behind: try increasing latency");
            }
        };
        Ok(input_device.build_input_stream(&config, input_data_fn, Self::err_fn, None)?)
    }

    fn get_output_stream(
        output_device: &Device,
        mut consumer: Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>,
        audio_pipeline: AudioPipeline,
    ) -> Result<Stream, anyhow::Error> {
        let config = output_device.default_output_config().unwrap().config();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => audio_pipeline.process_sample(s),

                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };

        Ok(output_device.build_output_stream(&config, output_data_fn, Self::err_fn, None)?)
    }

    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
}
