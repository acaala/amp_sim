pub trait Processor: Send {
    fn process(&self, input: f32) -> f32;
}