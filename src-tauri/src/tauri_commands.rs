use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
};

use cpal::traits::DeviceTrait;
use tauri::State;

use crate::{audio::AudioCommand, audio_backend::audio_device_manager::AudioDeviceManager};

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
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_input_device(new_device);

    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");

    Ok(())
}

#[tauri::command]
pub fn set_output_device(
    state: State<Arc<Mutex<AudioDeviceManager>>>,
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_output_device(new_device);

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
