use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod dimensions;

use serde_json::Value;
use skia_safe::Canvas;


pub type Generator =
    fn(canvas: &mut Canvas, width: i32, height: i32, seed: u64, dimensions: HashMap<String, Value>);


#[derive(Serialize)]
pub struct GeneratorInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub created_at: &'static str,
    pub dimensions: Value,

    #[serde(skip)]
    pub generate: Generator,
}

#[derive(Serialize)]
pub struct GeneratorGroup {
    pub group_name: &'static str,
    pub generators: Vec<GeneratorInfo>,
    pub sub_groups: Vec<GeneratorGroup>,
}

#[derive(Deserialize)]
pub struct GenerationRequest {
    pub generator_type: &'static str,
    pub seed: Option<u64>,
    pub width: u32,
    pub height: u32,
    pub dimension_values: HashMap<String, String>,
}
