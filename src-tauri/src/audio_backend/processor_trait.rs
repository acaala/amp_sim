pub trait Processor: Send + Sync {
    fn process(&self, input: f32) -> f32;
    fn name() -> &'static str
    where
        Self: Sized;
}
