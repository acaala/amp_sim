use std::collections::HashMap;

use crate::audio_backend::processor_trait::{Processor, ProcessorHashMapValue};
pub struct Amplifier {
    pub preamp_gain: f32,
    pub distortion_gain: f32,
    pub tone: f32,
    pub volume: f32,
}

impl Processor for Amplifier {
    fn process(&self, sample: f32) -> f32 {
        // Apply preamp gain
        let preamped_sample = sample * self.preamp_gain;

        // Apply distortion
        let distorted_sample = self.apply_distortion(preamped_sample);

        // Apply a simple low-pass filter to control high-frequency content
        let filtered_sample = self.apply_low_pass_filter(distorted_sample);

        // Adjust overall volume
        let output_sample = filtered_sample * self.volume;

        output_sample
    }

    fn update_values(&mut self, hash_map_values: HashMap<String, String>) {
        self.volume = hash_map_values
            .get("volume")
            .unwrap()
            .parse::<f32>()
            .unwrap();
        println!(
            "Set volume to: {:#?}",
            hash_map_values.get("volume").unwrap()
        );

        self.preamp_gain = hash_map_values
            .get("preamp_gain")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        println!(
            "Set preamp_gain to: {:#?}",
            hash_map_values.get("preamp_gain").unwrap()
        );
        self.distortion_gain = hash_map_values
            .get("distortion_gain")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        println!(
            "Set distortion_gain to: {:#?}",
            hash_map_values.get("distortion_gain").unwrap()
        );
        self.tone = hash_map_values.get("tone").unwrap().parse::<f32>().unwrap();

        println!("Set tone to: {:#?}", hash_map_values.get("tone").unwrap());
    }

    fn name() -> &'static str {
        "Amplifier"
    }

    fn get_name(&self) -> &'static str {
        "amplifier"
    }

    fn to_hash_map(&self) -> HashMap<String, ProcessorHashMapValue> {
        let mut processor_hash_map = HashMap::new();
        let mut processor_details = HashMap::new();

        processor_details.insert("preamp_gain".to_string(), self.preamp_gain);
        processor_details.insert("distortion_gain".to_string(), self.distortion_gain);
        processor_details.insert("tone".to_string(), self.tone);
        processor_details.insert("volume".to_string(), self.volume);

        processor_hash_map.insert(
            "name".to_string(),
            ProcessorHashMapValue::Str(self.get_name().to_string()),
        );
        processor_hash_map.insert(
            "details".to_string(),
            ProcessorHashMapValue::Map(processor_details),
        );

        processor_hash_map
    }
}

impl Amplifier {
    // Create a new Amplifier instance with default parameters
    pub fn new() -> Self {
        Amplifier {
            preamp_gain: 40.0,     // Gain before distortion stage
            distortion_gain: 60.0, // Gain applied during distortion stage
            tone: 400.0,           // Adjust the tone for controlling high-frequency content
            volume: 0.0,           // Adjust the overall output volume
        }
    }
    // Apply distortion to a sample
    fn apply_distortion(&self, sample: f32) -> f32 {
        // Apply clipping with a tanh function
        let clipped_sample = (sample * self.distortion_gain).tanh();
        clipped_sample
    }

    // Apply a simple low-pass filter to control high-frequency content
    fn apply_low_pass_filter(&self, sample: f32) -> f32 {
        // You can adjust the filter parameters based on your preference
        let dt = 1.0 / 44_100.0; // Sample rate assumed to be 44.1 kHz
        let cutoff_frequency = self.tone;
        let alpha = cutoff_frequency * dt / (1.0 + cutoff_frequency * dt);

        // Simple one-pole low-pass filter
        let filtered_sample = alpha * sample + (1.0 - alpha) * 0.0;

        filtered_sample
    }
}
