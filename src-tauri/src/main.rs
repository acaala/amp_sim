// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use amp_sim::audio_backend::{
    audio_device_manager::AudioDeviceManager, audio_stream_manager::AudioStreamManager,
};
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
    state: tauri::State<Arc<Mutex<AudioDeviceManager>>>,
    tx: tauri::State<mpsc::Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    println!("{:#?}", state.lock().unwrap().input_device.name());
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");
    let _ = state.lock().unwrap().set_input_device(new_device);
    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");
    Ok(())
}

#[tauri::command]
fn set_output_device(
    state: tauri::State<Arc<Mutex<AudioDeviceManager>>>,
    tx: tauri::State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    println!("{:#?}", state.lock().unwrap().output_device.name());

    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_output_device(new_device);

    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");
    Ok(())
}

#[tauri::command]
fn start_audio(tx: tauri::State<mpsc::Sender<AudioCommand>>) {
    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");
}

#[tauri::command]
fn stop_audio(tx: tauri::State<mpsc::Sender<AudioCommand>>) {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");
}

enum AudioCommand {
    Start,
    Stop,
}

fn audio_thread(rx: mpsc::Receiver<AudioCommand>, device_manager: Arc<Mutex<AudioDeviceManager>>) {
    let mut stream_manager = AudioStreamManager::new();

    for command in rx {
        match command {
            AudioCommand::Start => match device_manager.lock() {
                Ok(guard) => {
                    println!("Starting stream");
                    stream_manager.run(&guard).expect("Failed to start streams");
                }
                Err(poisoned) => {
                    println!("{:#?}", poisoned)
                }
            },
            AudioCommand::Stop => {
                println!("stopping");
                stream_manager.stop().expect("to stop streams");
            }
        }
    }
}

fn start_audio_thread(
    device_manager: Arc<std::sync::Mutex<AudioDeviceManager>>,
) -> mpsc::Sender<AudioCommand> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        audio_thread(rx, device_manager);
    });

    tx
}
fn main() {
    let audio_device_manager = Arc::new(Mutex::new(AudioDeviceManager::new()));
    let audio_device_manager_clone = audio_device_manager.clone();
    let audio_tx = start_audio_thread(audio_device_manager_clone);
    tauri::Builder::default()
        .manage(audio_device_manager)
        .manage(audio_tx)
        .invoke_handler(tauri::generate_handler![
            set_input_device,
            set_output_device,
            get_devices,
            start_audio,
            stop_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
