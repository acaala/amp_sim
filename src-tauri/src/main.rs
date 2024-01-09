// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use amp_sim::{
    audio::start_audio_thread,
    audio_backend::{audio_device_manager::AudioDeviceManager, audio_pipeline::AudioPipeline},
    config::Config,
    tauri_commands::{
        __cmd__add_processor_to_pipeline, __cmd__get_active_processors, __cmd__get_devices,
        __cmd__get_openai_api_key, __cmd__get_processors, __cmd__remove_processor,
        __cmd__set_input_device, __cmd__set_openai_api_key, __cmd__set_output_device,
        __cmd__start_audio, __cmd__stop_audio, __cmd__submit_user_prompt_to_ai,
        __cmd__update_processor_values, add_processor_to_pipeline, get_active_processors,
        get_devices, get_openai_api_key, get_processors, remove_processor, set_input_device,
        set_openai_api_key, set_output_device, start_audio, stop_audio, submit_user_prompt_to_ai,
        update_processor_values,
    },
};

fn main() {
    let config = Arc::new(Mutex::new(Config::retrieve()));
    let audio_device_manager = Arc::new(Mutex::new(AudioDeviceManager::new()));
    let audio_pipeline = Arc::new(Mutex::new(AudioPipeline::new()));

    let audio_tx = start_audio_thread(audio_device_manager.clone(), audio_pipeline.clone());

    if let Some(input_device) = &config.lock().unwrap().previous_input_device {
        let _ = audio_device_manager
            .lock()
            .unwrap()
            .set_input_device(input_device.to_string());
    }

    if let Some(output_device) = &config.lock().unwrap().previous_output_device {
        let _ = audio_device_manager
            .lock()
            .unwrap()
            .set_output_device(output_device.to_string());
    }

    tauri::Builder::default()
        .manage(audio_device_manager)
        .manage(audio_tx)
        .manage(audio_pipeline)
        .manage(config)
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
            remove_processor,
            submit_user_prompt_to_ai,
            set_openai_api_key,
            get_openai_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
