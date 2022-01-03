mod palette;

use super::types;

pub fn build_generators() -> types::GeneratorGroupBuilder {
    types::GeneratorGroupBuilder::new("util_generators").add_generator(palette::get_generator())
}
