use std::collections::HashMap;

use colors::hsv::Hsv;
use colors::lch::Lch;
use colors::palette::weighted_palette::WeightedPalette;
use colors::{ToLch, ToSkia};
use serde_json::Value;
use skia_safe::{Canvas, Paint, Rect};

use crate::generators::types::dimensions::{InputExtractor, OptionsInput};

use crate::generators::types::{
    self,
    dimensions::{GeneratorDimensionInfo, IntegerInput},
};

const GENERATOR_NAME: &str = "palette";
const NAME_NUM_COLORS: &str = "numColors";
const NAME_PALETTE_TYPE: &str = "paletteType";
const LCH: &str = "lch";
const HSV: &str = "hsv";

const INPUT_NUM_COLORS: IntegerInput = IntegerInput {
    min: 3,
    max: 30,
    default: 5,
};
fn palette_type_input() -> OptionsInput {
    OptionsInput {
        options: vec![String::from(LCH), String::from(HSV)],
        default: String::from(LCH),
    }
}

pub fn get_generator() -> types::GeneratorInfo {
    types::GeneratorInfo {
        name: GENERATOR_NAME,
        description: "this is just for testing",
        author: "Sam Vasta",
        created_at: "Dec 2021",
        dimensions: vec![
            GeneratorDimensionInfo {
                name: NAME_NUM_COLORS,
                description: "blah",
                data_info: Box::new(INPUT_NUM_COLORS),
            },
            GeneratorDimensionInfo {
                name: NAME_PALETTE_TYPE,
                description: "blah",
                data_info: Box::new(palette_type_input()),
            },
        ],
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
    println!("{:?}", dimensions);
    let num_colors = INPUT_NUM_COLORS.from_dimensions(NAME_NUM_COLORS, &dimensions);
    let palette_type = palette_type_input().from_dimensions(NAME_PALETTE_TYPE, &dimensions);

    let (colors, total_weight): (Vec<(skia_safe::Color, f64)>, f64) = if palette_type == LCH {
        let palette = WeightedPalette::new_from_ramp(
            Lch::new(80.0, 100.0, 0.0),
            Lch::new(80.0, 100.0, 360.0),
            num_colors as u32,
        );
        let total_weight = palette.total_weight;
        let colors = palette
            .colors_and_weights()
            .iter()
            .map(|x| (x.color.to_skia(), x.weight))
            .collect();

        (colors, total_weight)
    } else {
        let palette = WeightedPalette::new_from_ramp(
            Hsv::new(0.0, 1.0, 0.5),
            Hsv::new(360.0, 1.0, 0.5),
            num_colors as u32,
        );

        let total_weight = palette.total_weight;
        let colors = palette
            .colors_and_weights()
            .iter()
            .map(|x| (x.color.to_skia(), x.weight))
            .collect();
        (colors, total_weight)
    };

    canvas.save();

    let mut x = 0;

    for i in colors {
        let rect_width = (width as f64 * i.1 / total_weight).round() as i32;
        let rect = Rect::from_point_and_size((x, 0), (rect_width, height));

        let mut paint = Paint::default();

        paint.set_color(i.0);
        canvas.draw_rect(rect, &paint);

        x += rect_width;
    }

    canvas.restore();

    println!(
        "generating palette @{}x{}, seed={}, num_items={}",
        width, height, seed, num_colors
    )
}
