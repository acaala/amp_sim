use std::collections::HashMap;

pub trait Processor: Send + Sync {
    fn process(&self, input: f32) -> f32;
    fn update_values(&mut self, hash_map_values: HashMap<String, f32>);
    fn get_name(&self) -> &'static str;
    fn name() -> &'static str
    where
        Self: Sized;
}
