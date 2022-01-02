use std::collections::HashMap;

use serde_json::Value;
use skia_safe::{Canvas, Color, Paint, Rect};

use crate::generators::types::dimensions::InputExtractor;
use crate::util::colors;

use crate::generators::types::{
    self,
    dimensions::{GeneratorDimensionInfo, IntegerInput},
};

const NAME_NUM_ITEMS: &str = "test_palette";

const INPUT_NUM_ITEMS: IntegerInput = IntegerInput {
    min: 0,
    max: 20,
    default: 15,
};

pub fn get_generator() -> types::GeneratorInfo {
    types::GeneratorInfo {
        name: NAME_NUM_ITEMS,
        description: "this is just for testing",
        author: "Sam Vasta",
        created_at: "Dec 2021",
        dimensions: vec![GeneratorDimensionInfo {
            name: "num_items",
            description: "blah",
            data_info: Box::new(INPUT_NUM_ITEMS),
        }],
        generate: generate,
    }
}

fn generate(
    canvas: &mut Canvas,
    width: i32,
    height: i32,
    seed: u64,
    dimensions: HashMap<String, Value>,
) {
    let rand = fastrand::Rng::with_seed(seed);
    let num_items = INPUT_NUM_ITEMS.from_dimensions(NAME_NUM_ITEMS, dimensions);

    canvas.save();
    canvas
        .translate((
            128.0 + rand.f32() * 20.0 - 10.0,
            128.0 + rand.f32() * 20.0 - 10.0,
        ))
        .rotate(360.0 * rand.f32(), None);
    let rect = Rect::from_point_and_size((-90.5, -90.5), (181.0, 181.0));
    let mut paint = Paint::default();
    paint.set_color(Color::BLUE);
    canvas.draw_rect(rect, &paint);
    canvas.restore();

    println!(
        "generating test_generator @{}x{}, seed={}, num_items={}",
        width, height, seed, num_items
    )
}
