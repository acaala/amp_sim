use std::collections::HashMap;

use crate::audio_backend::processor_trait::{Processor, ProcessorHashMapValue};

pub struct ScreamerPedal {
    pub overdrive: f32, // Controls the amount of overdrive
    pub tone: f32,      // Tone control, typically affects mid frequencies
    pub level: f32,     // Output level of the pedal
}

impl Processor for ScreamerPedal {
    fn process(&self, input: f32) -> f32 {
        // Processing logic to simulate the Screamer pedal's effect:
        // 1. Soft clipping to produce overdrive
        // 2. Tone control to shape the sound
        // 3. Adjust the output level

        let clipped = self.soft_clipping(input * self.overdrive);
        let shaped_tone = self.apply_tone(clipped);
        shaped_tone * self.level
    }
    fn update_values(&mut self, hash_map_values: HashMap<String, String>) {
        self.overdrive = hash_map_values
            .get("overdrive")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        println!("Set overdrive to: {:#?}", self.overdrive);

        self.tone = hash_map_values.get("tone").unwrap().parse::<f32>().unwrap();
        println!("Set tone to: {:#?}", self.tone);

        self.level = hash_map_values
            .get("level")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        println!("Set level to: {:#?}", self.level);
    }

    fn get_name(&self) -> &'static str {
        "Screamer"
    }

    fn to_hash_map(&self) -> std::collections::HashMap<String, ProcessorHashMapValue> {
        let mut processor_hash_map = HashMap::new();
        let mut processor_details = HashMap::new();

        processor_details.insert("overdrive".to_string(), self.overdrive);
        processor_details.insert("tone".to_string(), self.tone);
        processor_details.insert("level".to_string(), self.level);

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

impl ScreamerPedal {
    pub fn new() -> Self {
        ScreamerPedal {
            overdrive: 1.0,
            tone: 1.0,
            level: 1.0,
        }
    }

    fn soft_clipping(&self, input: f32) -> f32 {
        // Simple soft clipping: a basic way to simulate overdrive
        // Real pedals use diodes for clipping; this is a basic approximation
        input.tanh()
    }

    fn apply_tone(&self, input: f32) -> f32 {
        // Tone shaping logic here
        // Placeholder for simplicity
        input * self.tone
    }
}
