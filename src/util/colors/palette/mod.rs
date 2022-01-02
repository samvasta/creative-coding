use palette::RelativeContrast;

pub mod weighted_palette;

pub struct TextColorPair<T> {
    pub light: T,
    pub dark: T,
}

impl<T> TextColorPair<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch> + Copy + palette::RelativeContrast,
{
    pub fn new(color1: T, color2: T) -> TextColorPair<T> {
        let lch1: palette::Lch = color1.into_color_unclamped();
        let lch2: palette::Lch = color2.into_color_unclamped();

        if lch1.l > lch2.l {
            TextColorPair {
                light: color1,
                dark: color2,
            }
        } else {
            TextColorPair {
                light: color2,
                dark: color1,
            }
        }
    }
}

pub trait ColorPalette<T>
where
    T: palette::convert::IntoColorUnclamped<palette::Lch>
        + palette::convert::IntoColorUnclamped<palette::Srgb>
        + Copy
        + palette::RelativeContrast,
{
    fn all_colors(&self) -> Vec<T>;

    fn random_color(&self, random: &fastrand::Rng) -> &T;

    fn lightest_color(&self) -> Option<T> {
        let mut lightest_so_far: Option<T> = None;
        let mut lightest_so_far_value: f32 = 0.0;

        for color in self.all_colors().iter() {
            let lch: palette::Lch = (*color).into_color_unclamped();

            if lch.l > lightest_so_far_value {
                lightest_so_far_value = lch.l;
                lightest_so_far = Some(color.to_owned())
            }
        }

        lightest_so_far
    }
    fn darkest_color(&self) -> Option<T> {
        let mut darkest_so_far: Option<T> = None;
        let mut darkest_so_far_value: f32 = 100.0;

        for color in self.all_colors().iter() {
            let lch: palette::Lch = (*color).into_color_unclamped();

            if lch.l < darkest_so_far_value {
                darkest_so_far_value = lch.l;
                darkest_so_far = Some(color.to_owned())
            }
        }

        darkest_so_far
    }

    fn biggest_contrast(&self) -> Option<TextColorPair<T>> {
        let all_colors = self.all_colors();
        if all_colors.len() < 2 {
            return None;
        }

        let mut c1 = all_colors[0];
        let mut c2 = all_colors[1];

        let mut largest_contrast_so_far: <T as RelativeContrast>::Scalar =
            c1.get_contrast_ratio(&c2);

        // have to permute over all pairs
        for outer_idx in 0..all_colors.len() - 1 {
            for inner_idx in outer_idx + 1..all_colors.len() {
                let contrast = all_colors[outer_idx].get_contrast_ratio(&all_colors[inner_idx]);

                if contrast > largest_contrast_so_far {
                    largest_contrast_so_far = contrast;
                    c1 = all_colors[outer_idx];
                    c2 = all_colors[inner_idx];
                }
            }
        }

        Some(TextColorPair::new(c1, c2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestColorPalette {
        colors: Vec<palette::Hsv>,
    }

    impl ColorPalette<palette::Hsv> for TestColorPalette {
        fn all_colors(&self) -> Vec<palette::Hsv> {
            self.colors.to_vec()
        }

        fn random_color(&self, _: &fastrand::Rng) -> &palette::Hsv {
            &self.colors[0]
        }
    }

    #[test]
    fn test_get_lightest() {
        let palette = TestColorPalette {
            colors: vec![
                palette::Hsv::new(0_f32, 0.5_f32, 0.5_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.3_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.7_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.1_f32),
            ],
        };

        let lightest = palette.lightest_color();

        assert!(lightest.is_some());
        assert_eq!(lightest.unwrap(), palette.colors[2]);
    }

    #[test]
    fn test_get_darkest() {
        let palette = TestColorPalette {
            colors: vec![
                palette::Hsv::new(0_f32, 0.5_f32, 0.5_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.3_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.7_f32),
                palette::Hsv::new(0_f32, 0.5_f32, 0.1_f32),
            ],
        };

        let darkest = palette.darkest_color();

        assert!(darkest.is_some());
        assert_eq!(darkest.unwrap(), palette.colors[3]);
    }

    #[test]
    fn test_biggest_contrast() {
        let palette = TestColorPalette {
            colors: vec![
                palette::Hsv::new(0_f32, 0.0_f32, 0.9_f32), //white
                palette::Hsv::new(0_f32, 0.0_f32, 0.1_f32), //black
                palette::Hsv::new(0_f32, 0.0_f32, 0.5_f32), //gray
            ],
        };

        let maybe_biggest_contrast = palette.biggest_contrast();

        assert!(maybe_biggest_contrast.is_some());
        let biggest_contrast = maybe_biggest_contrast.unwrap();
        assert_eq!(biggest_contrast.light, palette.colors[0]);
        assert_eq!(biggest_contrast.dark, palette.colors[1]);
    }
}
