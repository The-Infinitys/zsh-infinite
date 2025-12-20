use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

pub type GradientPart = Vec<((u8, u8, u8), f32)>;

// --- Gradient用のカスタムシリアライズ/デシリアライズ ---

pub fn serialize_gradient<S>(stops: &GradientPart, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(stops.len()))?;
    for (rgb, pos) in stops {
        // フルカラー形式をHex文字列に変換して保存
        seq.serialize_element(&format!("#{:02X}{:02X}{:02X}:{}", rgb.0, rgb.1, rgb.2, pos))?;
    }
    seq.end()
}

pub fn deserialize_gradient<'de, D>(deserializer: D) -> Result<GradientPart, D::Error>
where
    D: Deserializer<'de>,
{
    struct GradientVisitor;
    impl<'de> Visitor<'de> for GradientVisitor {
        type Value = GradientPart;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of '#RRGGBB:stop' strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut stops = Vec::new();
            while let Some(s) = seq.next_element::<String>()? {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 && parts[0].starts_with('#') && parts[0].len() == 7 {
                    let r = u8::from_str_radix(&parts[0][1..3], 16).map_err(de::Error::custom)?;
                    let g = u8::from_str_radix(&parts[0][3..5], 16).map_err(de::Error::custom)?;
                    let b = u8::from_str_radix(&parts[0][5..7], 16).map_err(de::Error::custom)?;
                    let pos = parts[1].parse::<f32>().map_err(de::Error::custom)?;
                    stops.push(((r, g, b), pos));
                } else {
                    return Err(de::Error::custom(format!(
                        "Invalid gradient stop format: {}",
                        s
                    )));
                }
            }
            Ok(stops)
        }
    }
    deserializer.deserialize_seq(GradientVisitor)
}

// --- 色計算ロジック ---

pub fn lerp_rgb_color(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let r = (rgb1.0 as f32 + (rgb2.0 as f32 - rgb1.0 as f32) * t) as u8;
    let g = (rgb1.1 as f32 + (rgb2.1 as f32 - rgb1.1 as f32) * t) as u8;
    let b = (rgb1.2 as f32 + (rgb2.2 as f32 - rgb1.2 as f32) * t) as u8;
    (r, g, b)
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r_f = r as f32 / 255.0;
    let g_f = g as f32 / 255.0;
    let b_f = b as f32 / 255.0;

    let max = r_f.max(g_f).max(b_f);
    let min = r_f.min(g_f).min(b_f);
    let delta = max - min;

    let mut h = 0.0;
    let s = if max == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * ((max + min) / 2.0) - 1.0).abs())
    };
    let l = (max + min) / 2.0;

    if delta != 0.0 {
        h = if max == r_f {
            (g_f - b_f) / delta % 6.0
        } else if max == g_f {
            (b_f - r_f) / delta + 2.0
        } else {
            (r_f - g_f) / delta + 4.0
        };
        h *= 60.0;
        if h < 0.0 {
            h += 360.0;
        }
    }
    (h, s, l)
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r_p, g_p, b_p) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((r_p + m) * 255.0) as u8,
        ((g_p + m) * 255.0) as u8,
        ((b_p + m) * 255.0) as u8,
    )
}

pub fn create_default_rainbow_gradient() -> GradientPart {
    vec![
        ((255, 0, 0), 0.0),    // Red
        ((255, 127, 0), 0.16), // Orange
        ((255, 255, 0), 0.32), // Yellow
        ((0, 255, 0), 0.48),   // Green
        ((0, 0, 255), 0.64),   // Blue
        ((75, 0, 130), 0.80),  // Indigo
        ((148, 0, 211), 1.0),  // Violet
    ]
}
