use crate::audio_backend::processor_trait::Processor;

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

    fn name() -> &'static str {
        "Screamer"
    }
}

impl ScreamerPedal {
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
