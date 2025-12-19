use zsh_seq::NamedColor;

use crate::zsh::prompt::{PromptConnection, PromptSeparation};
#[derive(Clone, Default)]
pub struct PromptTheme {
    pub color: PromptColorScheme,
    pub connection: PromptConnection,
    pub separation: PromptSeparation,
}
#[derive(Clone)]
pub struct PromptColorScheme {
    pub bg: NamedColor,
    pub fg: NamedColor,
    pub separation: SeparationColor,
}
#[derive(Clone)]
pub enum SeparationColor {
    Single(NamedColor),
    Rainbow(f32),
}
impl SeparationColor {
    fn get(&self, progress: f32) -> NamedColor {
        match self {
            Self::Single(color) => color.to_owned(),
            Self::Rainbow(start_hue) => {
                // progress (0.0~1.0) に応じて色相を 0~360度変化させる
                let hue = (start_hue + progress * 360.0) % 360.0;
                // 彩度 1.0, 輝度 0.5 (最も鮮やかな状態) でRGBに変換
                let rgb = hsl_to_rgb(hue, 1.0, 0.5);
                NamedColor::FullColor(rgb)
            }
        }
    }
}
/// HSLからRGB(u8, u8, u8)へ変換する補助関数
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0).round() as u8,
        ((g_prime + m) * 255.0).round() as u8,
        ((b_prime + m) * 255.0).round() as u8,
    )
}

impl Default for PromptColorScheme {
    fn default() -> Self {
        Self {
            bg: NamedColor::Black,
            fg: NamedColor::White,
            separation: SeparationColor::Single(NamedColor::LightBlack),
        }
    }
}
