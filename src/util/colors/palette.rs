use std::collections::HashMap;

use palette::{convert::IntoColorUnclamped, rgb};

pub trait PickRandomColor<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    fn random_color(&self, random: &fastrand::Rng) -> &'a T;
}

pub struct WeightedColor<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    color: &'a T,
    weight: f64,
}

pub struct Palette<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    colors: Vec<WeightedColor<'a, T>>,
}

impl<'a, T> Palette<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    fn color_at(&self, percent: f64) -> &'a T {
        if self.colors.is_empty() {
            panic!("Cannot retrieve a color from an empty palette!")
        }

        let mut accumulator = percent;

        for color in self.colors.iter() {
            if accumulator > color.weight {
                accumulator -= color.weight
            } else {
                return color.color;
            }
        }

        self.colors.last().unwrap().color
    }
}

impl<'a, T> PickRandomColor<'a, T> for Palette<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    fn random_color(&self, random: &fastrand::Rng) -> &'a T {
        let percent = random.f64();

        self.color_at(percent)
    }
}

pub struct NamedPalette<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    color_vec: Vec<&'a T>,
    colors: HashMap<String, &'a T>,
}

impl<'a, T> PickRandomColor<'a, T> for NamedPalette<'a, T>
where
    T: IntoColorUnclamped<rgb::Rgba>,
{
    fn random_color(&self, random: &fastrand::Rng) -> &'a T {
        let index = random.usize(..self.colors.len());

        &self.color_vec[index]
    }
}
