use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Error};
use cpal::traits::DeviceTrait;
use tauri::{State, Window};

use crate::{
    assistant::Assistant,
    audio::{get_processor_impl_names, AudioCommand},
    audio_backend::{
        audio_device_manager::AudioDeviceManager,
        audio_pipeline::AudioPipeline,
        processor_trait::{Processor, ProcessorHashMapValue},
        processors::{amplifier::Amplifier, screamer::ScreamerPedal},
    },
    config::{assistant_config::AssistantConfig, audio_config::AudioConfig, config::Config},
    events::emit_pipeline_updated_event,
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
    audio_config: State<Arc<Mutex<AudioConfig>>>,
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_input_device(new_device.clone());

    let mut config_guard = audio_config.lock().unwrap();
    config_guard.previous_input_device = Some(new_device);
    let _ = config_guard.save();

    tx.send(AudioCommand::Start)
        .expect("Failed to send start command");

    Ok(())
}

#[tauri::command]
pub fn set_output_device(
    state: State<Arc<Mutex<AudioDeviceManager>>>,
    audio_config: State<Arc<Mutex<AudioConfig>>>,
    tx: State<Sender<AudioCommand>>,
    new_device: String,
) -> Result<(), String> {
    tx.send(AudioCommand::Stop)
        .expect("Failed to send stop command");

    let _ = state.lock().unwrap().set_output_device(new_device.clone());

    let mut config_guard = audio_config.lock().unwrap();
    config_guard.previous_output_device = Some(new_device);
    let _ = config_guard.save();

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
pub fn add_processor_to_pipeline(
    audio_pipeline: State<Arc<Mutex<AudioPipeline>>>,
    window: Window,
    name: String,
) {
    let processor = init_processor(&name, None);

    if let Ok(proc) = processor {
        let mut audio_pipeline_guard = audio_pipeline.lock().unwrap();
        audio_pipeline_guard.add_processor(proc);
        println!("Added processor: {:#?}", name);

        emit_pipeline_updated_event(window, audio_pipeline_guard)
    }
}

fn init_processor(
    name: &String,
    values: Option<HashMap<String, String>>,
) -> Result<Box<dyn Processor>, Error> {
    match name.to_lowercase().as_str() {
        "amplifier" => {
            let mut amplifier = Box::new(Amplifier::new());
            if let Some(hashmap_values) = values {
                amplifier.update_values(hashmap_values);
            }
            Ok(amplifier)
        }
        "screamer" => Ok(Box::new(ScreamerPedal::new())),
        _ => {
            println!("Failed to find processor");
            Err(anyhow!("Processor not found"))
        }
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
pub fn remove_processor(
    pipeline: State<Arc<Mutex<AudioPipeline>>>,
    window: Window,
    processor_name: String,
) {
    let mut pipeline_guard = pipeline.lock().unwrap();

    pipeline_guard.remove_processor(processor_name);

    emit_pipeline_updated_event(window, pipeline_guard);
}

#[tauri::command]
pub async fn set_openai_api_key(
    assistant: State<'_, Arc<tokio::sync::Mutex<AssistantConfig>>>,
    key: String,
) -> Result<(), String> {
    let mut assistant_guard = assistant.lock().await;

    assistant_guard.api_key = Some(key);
    let _ = assistant_guard.save();

    // TODO: Handle failure.
    Ok(())
}

#[tauri::command]
pub async fn get_openai_api_key(
    assistant: State<'_, Arc<tokio::sync::Mutex<AssistantConfig>>>,
) -> Result<Option<String>, String> {
    let assistant_guard = assistant.lock().await;

    Ok(assistant_guard.api_key.clone())
}

#[tauri::command]
pub fn init_assistant(
    assistant: State<'_, Arc<tokio::sync::Mutex<Assistant>>>,
    assistant_config: State<'_, Arc<tokio::sync::Mutex<AssistantConfig>>>,
) -> Result<(), String> {
    println!("Init assistant");
    let assistant_config_clone = Arc::clone(&assistant_config);
    let assistant_clone = Arc::clone(&assistant);

    tokio::spawn(async move {
        let mut assistant_config_guard = assistant_config_clone.lock().await;
        let mut assistant_guard = assistant_clone.lock().await;

        if let Some(api_key) = &assistant_config_guard.api_key {
            assistant_guard.api_key = Some(api_key.clone());
        }

        if let Some(thread_id) = &mut assistant_config_guard.thread_id {
            assistant_guard.thread_id = Some(thread_id.clone());
            println!("{:?}", thread_id);
        } else {
            match assistant_guard.init_thread().await {
                Ok(thread_id) => {
                    assistant_config_guard.thread_id = Some(thread_id);
                    let _ = assistant_config_guard.save();
                }
                Err(err) => {
                    eprintln!("Error initializing thread: {:?}", err);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn submit_user_prompt(
    assistant: State<Arc<tokio::sync::Mutex<Assistant>>>,
    audio_pipeline: State<Arc<Mutex<AudioPipeline>>>,
    window: Window,
    prompt: String,
) -> Result<(), String> {
    let assistant_clone = Arc::clone(&assistant);
    let audio_pipeline_clone = Arc::clone(&audio_pipeline);

    tokio::spawn(async move {
        let assistant_guard = assistant_clone.lock().await;
        let _ = assistant_guard.send_message(prompt).await;
        let run_id = assistant_guard.run_assistant().await.unwrap();
        let mut run_status = String::new();

        while run_status != "completed" {
            println!("Processing..");
            // Do an event to the front end here.
            thread::sleep(Duration::from_secs(2));
            if let Ok(status) = assistant_guard.check_run_status(run_id.clone()).await {
                run_status = status
            }
        }
        println!("Is Complete");

        let response = assistant_guard.retrieve_messages().await.unwrap();

        let assistant_response = assistant_guard
            .get_parsed_assistant_response(response)
            .unwrap();

        for processor_map in &assistant_response.processors {
            for (processor_name, settings) in processor_map {
                let mut new_settings_map: HashMap<String, String> = HashMap::new();

                for (setting_name, setting_value) in settings {
                    new_settings_map.insert(setting_name.to_owned(), setting_value.to_string());
                }

                let processor = init_processor(processor_name, Some(new_settings_map));
                if let Ok(proc) = processor {
                    audio_pipeline_clone.lock().unwrap().add_processor(proc);
                    println!("Added processor: {:#?}", processor_name)
                }
            }
        }

        let audio_pipeline_guard = audio_pipeline_clone.lock().unwrap();
        emit_pipeline_updated_event(window, audio_pipeline_guard);
    });

    Ok(())
}
