use std::collections::HashMap;

use serde_json::Value;
use skia_safe::{EncodedImageFormat, Surface};

mod test_generator;
pub mod types;

pub fn generator_list() -> Vec<types::GeneratorInfo> {
    vec![test_generator::get_generator()]
}

pub fn generator_map() -> HashMap<String, types::GeneratorInfo> {
    let mut generators: HashMap<String, types::GeneratorInfo> = HashMap::new();

    for generator in generator_list() {
        generators.insert(String::from(generator.name), generator);
    }

    generators
}

#[derive(Debug, Clone)]
pub struct GeneratorError;

pub struct GenerateArgs {
    pub width: i32,
    pub height: i32,
    pub generator_type: String,
    pub seed: u64,
    pub dimensions: HashMap<String, Value>,
}

pub fn generate_one(args: GenerateArgs) -> Result<Vec<u8>, GeneratorError> {
    let map = generator_map();
    let generator = map.get(&args.generator_type);

    if let Some(gen) = generator {
        let mut surface =
            Surface::new_raster_n32_premul((args.width * 2, args.height * 2)).unwrap();
        let canvas = surface.canvas();
        (gen.generate)(canvas, args.width, args.height, args.seed, args.dimensions);

        let image = surface.image_snapshot();
        let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

        Ok(data.as_bytes().to_vec())
    } else {
        Err(GeneratorError)
    }
}
