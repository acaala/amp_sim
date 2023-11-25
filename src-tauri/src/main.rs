// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use amp_sim::{
    audio::start_audio_thread,
    audio_backend::{audio_device_manager::AudioDeviceManager, audio_pipeline::AudioPipeline},
    tauri_commands::{
        __cmd__add_processor_to_pipeline, __cmd__get_active_processors, __cmd__get_devices,
        __cmd__get_processors, __cmd__remove_processor, __cmd__set_input_device,
        __cmd__set_output_device, __cmd__start_audio, __cmd__stop_audio,
        __cmd__update_processor_values, add_processor_to_pipeline, get_active_processors,
        get_devices, get_processors, remove_processor, set_input_device, set_output_device,
        start_audio, stop_audio, update_processor_values,
    },
};

fn main() {
    let audio_device_manager = Arc::new(Mutex::new(AudioDeviceManager::new()));
    let audio_pipeline = Arc::new(Mutex::new(AudioPipeline::new()));

    let audio_tx = start_audio_thread(audio_device_manager.clone(), audio_pipeline.clone());

    tauri::Builder::default()
        .manage(audio_device_manager)
        .manage(audio_tx)
        .manage(audio_pipeline)
        .invoke_handler(tauri::generate_handler![
            set_input_device,
            set_output_device,
            get_devices,
            start_audio,
            stop_audio,
            get_processors,
            add_processor_to_pipeline,
            update_processor_values,
            get_active_processors,
            remove_processor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
