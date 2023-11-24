use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub enum ProcessorHashMapValue {
    Str(String),
    Map(HashMap<String, f32>),
}
pub trait Processor: Send + Sync {
    fn process(&self, input: f32) -> f32;
    fn update_values(&mut self, hash_map_values: HashMap<String, String>);
    fn get_name(&self) -> &'static str;
    fn to_hash_map(&self) -> HashMap<String, ProcessorHashMapValue>;

    fn name() -> &'static str
    where
        Self: Sized;
}
