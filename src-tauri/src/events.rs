use std::{collections::HashMap, sync::MutexGuard};

use tauri::Window;

use crate::audio_backend::{audio_pipeline::AudioPipeline, processor_trait::ProcessorHashMapValue};

pub fn emit_pipeline_updated_event(window: Window, pipeline: MutexGuard<AudioPipeline>) {
    let active_processors: Vec<HashMap<String, ProcessorHashMapValue>> = pipeline
        .processors
        .iter()
        .filter_map(|proc| Some(proc.to_hash_map()))
        .collect();

    let _ = window.emit("pipeline_updated", active_processors);
}
