use palette::convert::IntoColorUnclamped;

use super::ColorPalette;

pub struct WeightedColor<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch>
        + palette::convert::IntoColorUnclamped<palette::Srgb>
        + Copy
        + palette::RelativeContrast,
{
    color: T,
    weight: f64,
}

pub struct WeightedPalette<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch>
        + palette::convert::IntoColorUnclamped<palette::Srgb>
        + Copy
        + palette::RelativeContrast,
{
    colors: Vec<Box<WeightedColor<T>>>,
    total_weight: f64,
}

impl<T> WeightedPalette<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch>
        + palette::convert::IntoColorUnclamped<palette::Srgb>
        + Copy
        + palette::RelativeContrast,
{
    fn add_color(&mut self, color: &T, weight: f64) {
        self.colors.push(Box::new(WeightedColor {
            color: *color,
            weight: weight,
        }));
        self.total_weight += weight;
    }

    fn color_at(&self, value: f64) -> &T {
        if self.colors.is_empty() {
            panic!("Cannot retrieve a color from an empty palette!")
        }

        let mut accumulator = value;

        for color in self.colors.iter() {
            if accumulator > color.weight {
                accumulator -= color.weight
            } else {
                return &color.color;
            }
        }

        &self.colors.last().unwrap().color
    }

    fn color_at_percent(&self, percent: f64) -> &T {
        let value = percent / self.total_weight;

        self.color_at(value)
    }

    fn new(colors: Vec<T>) -> WeightedPalette<T> {
        WeightedPalette {
            colors: colors
                .iter()
                .map(|x| {
                    Box::new(WeightedColor {
                        color: *x,
                        weight: 1.0,
                    })
                })
                .collect(),
            total_weight: colors.len() as f64,
        }
    }
}

impl WeightedPalette<palette::Lch> {
    fn new_from_ramp(
        start: palette::Lch,
        end: palette::Lch,
        num_steps: u32,
    ) -> WeightedPalette<palette::Lch> {
        let mut l = start.l;
        let mut c = start.chroma;
        let mut h = start.hue;

        let mut vec: Vec<Box<WeightedColor<palette::Lch>>> = Vec::new();

        vec.push(Box::new(WeightedColor {
            color: palette::Lch::new(l, c, h).into_color_unclamped(),
            weight: 1.0,
        }));

        let dl = (end.l - start.l) / num_steps as f32;
        let dc = (end.chroma - start.chroma) / num_steps as f32;
        let dh = (end.hue - start.hue).to_positive_degrees() / num_steps as f32;

        for _ in 0..num_steps {
            l += dl;
            c += dc;
            h += dh;
            vec.push(Box::new(WeightedColor {
                color: palette::Lch::new(l, c, h).into_color_unclamped(),
                weight: 1.0,
            }));
        }

        WeightedPalette {
            colors: vec,
            total_weight: num_steps as f64,
        }
    }
}

impl<T> ColorPalette<T> for WeightedPalette<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch>
        + palette::convert::IntoColorUnclamped<palette::Srgb>
        + Copy
        + palette::RelativeContrast,
{
    fn all_colors(&self) -> Vec<T> {
        self.colors
            .iter()
            .map(|weighted_color| weighted_color.color)
            .collect()
    }

    fn random_color(&self, random: &fastrand::Rng) -> &T {
        let value = random.f64() * self.total_weight;

        self.color_at(value)
    }
}
