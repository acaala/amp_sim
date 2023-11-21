// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::Mutex};

use amp_sim::audio_backend::audio_device_manager::AudioDeviceManager;
use cpal::traits::DeviceTrait;

#[tauri::command]
fn get_devices() -> Result<HashMap<String, Vec<String>>, String> {
    let input_devices = AudioDeviceManager::get_input_devices().expect("To get input devices");
    let output_devices = AudioDeviceManager::get_output_devices().expect("To get output devices");
    Ok(HashMap::from([
        ("inputs".to_string(), input_devices),
        ("outputs".to_string(), output_devices),
    ]))
}

#[tauri::command]
fn set_input_device(
    state: tauri::State<'_, Mutex<AudioDeviceManager>>,
    new_device: String,
) -> Result<(), String> {
    state.lock().unwrap().set_input_device(new_device);
    println!("{:#?}", state.lock().unwrap().input_device.name());
    Ok(())
}

#[tauri::command]
fn set_output_device(
    state: tauri::State<Mutex<AudioDeviceManager>>,
    new_device: String,
) -> Result<(), String> {
    state.lock().unwrap().set_output_device(new_device);
    println!("{:#?}", state.lock().unwrap().output_device.name());
    Ok(())
}

// #[tauri::command]
// fn start_audio(state: tauri::State<'_, Mutex<AudioDeviceManager>>) -> Result<(), String> {
//     state.lock().unwrap().start_stream();
//     Ok(())
// }

fn main() {
    let audio_device_manager = AudioDeviceManager::new();

    tauri::Builder::default()
        .manage(Mutex::new(audio_device_manager))
        .invoke_handler(tauri::generate_handler![
            set_input_device,
            set_output_device,
            get_devices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
