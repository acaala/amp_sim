use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
};

use anyhow::{anyhow, Error};
use cpal::traits::DeviceTrait;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;
use tauri::State;

use crate::{
    audio::{get_processor_impl_names, AudioCommand},
    audio_backend::{
        audio_device_manager::AudioDeviceManager,
        audio_pipeline::AudioPipeline,
        processor_trait::{Processor, ProcessorHashMapValue},
        processors::{amplifier::Amplifier, screamer::ScreamerPedal},
    },
    config::Config,
};

#[tauri::command]
pub fn start_audio(
    tx: tauri::State<Sender<AudioCommand>>,
    audio_device_manager: State<Arc<Mutex<AudioDeviceManager>>>,
) -> Result<HashMap<String, String>, String> {
    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");

    let device_manager = audio_device_manager
        .lock()
        .expect("To get default devices on load");

    Ok(HashMap::from([
        (
            "input".to_string(),
            device_manager.input_device.name().unwrap(),
        ),
        (
            "output".to_string(),
            device_manager.output_device.name().unwrap(),
        ),
    ]))
}

#[tauri::command]
pub fn stop_audio(tx: State<Sender<AudioCommand>>) {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");
}

#[tauri::command]
pub fn set_input_device(
    state: State<Arc<Mutex<AudioDeviceManager>>>,
    config: State<Arc<Mutex<Config>>>,
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_input_device(new_device.clone());

    let mut config_guard = config.lock().unwrap();
    config_guard.previous_input_device = Some(new_device);
    config_guard.save();

    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");

    Ok(())
}

#[tauri::command]
pub fn set_output_device(
    state: State<Arc<Mutex<AudioDeviceManager>>>,
    config: State<Arc<Mutex<Config>>>,
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_output_device(new_device.clone());

    let mut config_guard = config.lock().unwrap();
    config_guard.previous_output_device = Some(new_device);
    config_guard.save();

    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");

    Ok(())
}

#[tauri::command]
pub fn get_devices() -> Result<HashMap<String, Vec<String>>, String> {
    let input_devices = AudioDeviceManager::get_input_devices().expect("To get input devices");
    let output_devices = AudioDeviceManager::get_output_devices().expect("To get output devices");
    Ok(HashMap::from([
        ("inputs".to_string(), input_devices),
        ("outputs".to_string(), output_devices),
    ]))
}

#[tauri::command]
pub fn get_processors() -> Vec<&'static str> {
    get_processor_impl_names()
}

#[tauri::command]
pub fn get_active_processors(
    audio_pipeline: State<Arc<Mutex<AudioPipeline>>>,
) -> Vec<HashMap<String, ProcessorHashMapValue>> {
    let active_processors = audio_pipeline.lock().unwrap();

    let active_processors = active_processors
        .processors
        .iter()
        .filter_map(|proc| Some(proc.to_hash_map()))
        .collect();

    active_processors
}

#[tauri::command]
pub fn add_processor_to_pipeline(audio_pipeline: State<Arc<Mutex<AudioPipeline>>>, name: String) {
    let processor: Result<Box<dyn Processor>, Error> = match name.as_str() {
        "amplifier" => {
            let amplifier = Box::new(Amplifier::new());
            Ok(amplifier)
        }
        "screamer" => Ok(Box::new(ScreamerPedal::new())),
        _ => {
            println!("Failed to find processor");
            Err(anyhow!("Processor not found"))
        }
    };

    if let Ok(proc) = processor {
        audio_pipeline.lock().unwrap().add_processor(proc);
        println!("Added processor: {:#?}", name)
    }
}

#[tauri::command]
pub fn update_processor_values(
    pipeline: State<Arc<Mutex<AudioPipeline>>>,
    processor_name: String,
    values: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut pipeline_guard = pipeline.lock().unwrap();
    let processors = &mut pipeline_guard.processors;
    if let Some(proc) = processors
        .iter_mut()
        .find(|proc| proc.get_name() == processor_name)
    {
        proc.update_values(values.clone());
    }

    values
}

#[tauri::command]
pub fn remove_processor(pipeline: State<Arc<Mutex<AudioPipeline>>>, processor_name: String) {
    let mut pipeline_guard = pipeline.lock().unwrap();

    pipeline_guard.remove_processor(processor_name)
}

#[tauri::command]
pub fn set_openai_api_key(config: State<Arc<Mutex<Config>>>, key: String) -> Result<(), String> {
    let mut config_guard = config.lock().unwrap();

    config_guard.openai_api_key = Some(key);
    let _ = config_guard.save();

    // TODO: Handle failure.
    Ok(())
}

#[tauri::command]
pub fn get_openai_api_key(config: State<Arc<Mutex<Config>>>) -> Option<String> {
    let config_guard = config.lock().unwrap();

    config_guard.openai_api_key.clone()
}
