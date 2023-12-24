use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::audio_backend::{
    audio_device_manager::AudioDeviceManager, audio_pipeline::AudioPipeline,
    audio_stream_manager::AudioStreamManager,
};

pub enum AudioCommand {
    Start,
    Stop,
}

fn audio_thread(
    rx: Receiver<AudioCommand>,
    device_manager: Arc<Mutex<AudioDeviceManager>>,
    audio_pipeline: Arc<Mutex<AudioPipeline>>,
) {
    let mut stream_manager = AudioStreamManager::new();

    for command in rx {
        match command {
            AudioCommand::Start => match device_manager.lock() {
                Ok(guard) => {
                    println!("Starting stream...");

                    stream_manager
                        .run(&guard, audio_pipeline.clone())
                        .expect("Failed to start streams");
                }
                Err(poisoned) => {
                    println!("{:#?}", poisoned)
                }
            },
            AudioCommand::Stop => {
                println!("Stopping stream...");
                stream_manager.stop().expect("to stop streams");
            }
        }
    }
}

pub fn start_audio_thread(
    device_manager: Arc<Mutex<AudioDeviceManager>>,
    audio_pipeline: Arc<Mutex<AudioPipeline>>,
) -> Sender<AudioCommand> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        audio_thread(rx, device_manager, audio_pipeline);
    });

    tx
}

pub fn get_processor_impl_names() -> Vec<&'static str> {
    // Has to be a better way.
    vec!["amplifier", "screamer"]
}
