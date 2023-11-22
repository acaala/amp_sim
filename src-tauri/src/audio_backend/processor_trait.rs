pub trait Processor: Send + Sync {
    fn process(&self, input: f32) -> f32;
}
