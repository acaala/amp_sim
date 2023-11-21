use crate::audio_backend::processor_trait::Processor;
pub struct Amplifier {
    preamp_gain: f32,
    distortion_gain: f32,
    tone: f32,
    volume: f32,
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
}

impl Amplifier {
    // Create a new Amplifier instance with default parameters
    pub fn new() -> Self {
        Amplifier {
            preamp_gain: 40.0,     // Gain before distortion stage
            distortion_gain: 60.0, // Gain applied during distortion stage
            tone: 400.0,           // Adjust the tone for controlling high-frequency content
            volume: 1.0,           // Adjust the overall output volume
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
