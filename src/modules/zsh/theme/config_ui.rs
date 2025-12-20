use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::fmt;
use zsh_seq::NamedColor;

use super::color_scheme::AccentColor;
use super::gradient::create_default_rainbow_gradient;
use super::named_color_serde;
use super::prompt_theme::PromptTheme; // PromptThemeの定義があるファイルを指定
use crate::zsh::prompt::{PromptConnection, PromptSeparation}; // crateルートからのパス

// DisplayNamedColor
struct DisplayNamedColor<'a>(&'a NamedColor);
impl<'a> fmt::Display for DisplayNamedColor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // named_color_serde内のデシリアライザが期待する形式に合わせる
        let s = match self.0 {
            NamedColor::Code256(c) => format!("Code256({})", c),
            NamedColor::FullColor((r, g, b)) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            _ => format!("{:?}", self.0),
        };
        write!(f, "{}", s)
    }
}

pub fn prompt_for_named_color(prompt_text: &str, default_color: &NamedColor) -> NamedColor {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .default(DisplayNamedColor(default_color).to_string())
        .interact_text()
        .map(|s| named_color_serde::deserialize_from_str(&s).unwrap_or(*default_color))
        .unwrap_or_else(|_| *default_color)
}

// 新しい関数: フルカラーのRGB値をプロンプトで取得
pub fn prompt_for_rgb_color(prompt_text: &str, default_rgb: (u8, u8, u8)) -> (u8, u8, u8) {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .default(format!(
            "#{:02X}{:02X}{:02X}",
            default_rgb.0, default_rgb.1, default_rgb.2
        ))
        .interact_text()
        .map(|s| {
            if s.starts_with('#')
                && s.len() == 7
                && let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&s[1..3], 16),
                    u8::from_str_radix(&s[3..5], 16),
                    u8::from_str_radix(&s[5..7], 16),
                )
            {
                return (r, g, b);
            }
            default_rgb
        })
        .unwrap_or(default_rgb)
}

pub fn configure_colors(theme: &mut PromptTheme) {
    println!("\n--- Configure Colors ---");

    theme.color.bg = prompt_for_named_color("Background color", &theme.color.bg);
    theme.color.fg = prompt_for_named_color("Foreground color", &theme.color.fg);
    theme.color.pc = prompt_for_named_color("Primary color (pc)", &theme.color.pc);
    theme.color.sc = prompt_for_named_color("Secondary color (sc)", &theme.color.sc);

    let options = [
        "Single Color",
        "Rainbow",
        "Default Rainbow Gradient",
        "Custom Gradient",
    ];
    let default_selection = match &theme.color.accent {
        AccentColor::Single(_) => 0,
        AccentColor::Rainbow(_) => 1,
        // Default Rainbow Gradient と Custom Gradient を区別するために、既存のグラデーションが
        // デフォルトの虹色グラデーションと一致するかどうかを簡易的に判定するか、
        // または単に Custom Gradient にフォールバックさせるかを検討。
        // ここでは簡単に Custom Gradient にフォールバックさせます。
        AccentColor::Gradient(_) => 3, // Custom Gradient に対応
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose separation color type")
        .items(options)
        .default(default_selection)
        .interact()
        .unwrap();

    theme.color.accent = match selection {
        0 => AccentColor::Single(prompt_for_named_color("Color", &NamedColor::LightBlack)),
        1 => {
            let color = prompt_for_named_color(
                "Rainbow Start Color (Hex)",
                &NamedColor::FullColor((255, 0, 0)),
            );
            AccentColor::Rainbow(color)
        }
        2 => {
            // Default Rainbow Gradient
            AccentColor::Gradient(create_default_rainbow_gradient())
        }
        3 => {
            // Custom Gradient (Existing 2-point gradient)
            let c1_rgb = prompt_for_rgb_color("Gradient Start Color (Hex)", (0, 255, 255)); // Cyan
            let c2_rgb = prompt_for_rgb_color("Gradient End Color (Hex)", (0, 0, 255)); // Blue
            AccentColor::Gradient(vec![(c1_rgb, 0.0), (c2_rgb, 1.0)])
        }
        _ => unreachable!(),
    };
}

pub fn configure_connection(theme: &mut PromptTheme) {
    println!("\n--- Configure Connection ---");
    let options = [
        PromptConnection::None,
        PromptConnection::Line,
        PromptConnection::Double,
        PromptConnection::Bold,
        PromptConnection::Dashed,
        PromptConnection::Dotted,
        PromptConnection::Dot,
        PromptConnection::Bullet,
        PromptConnection::Wave,
        PromptConnection::ZigZag,
        PromptConnection::Bar,
        PromptConnection::Gradient,
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose style")
        .items(
            options
                .iter()
                .map(|o| format!("{:?}", o))
                .collect::<Vec<_>>(),
        )
        .default(
            options
                .iter()
                .position(|&p| p == theme.connection)
                .unwrap_or(0),
        )
        .interact()
        .unwrap();
    theme.connection = options[selection];
}

pub fn configure_separation(theme: &mut PromptTheme) {
    println!("\n--- Configure Separators ---");
    let options = [
        PromptSeparation::Block,
        PromptSeparation::Sharp,
        PromptSeparation::Slash,
        PromptSeparation::BackSlash,
        PromptSeparation::Round,
        PromptSeparation::Blur,
        PromptSeparation::Flame,
        PromptSeparation::Pixel,
        PromptSeparation::Wave,
        PromptSeparation::Lego,
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose style")
        .items(
            options
                .iter()
                .map(|o| format!("{:?}", o))
                .collect::<Vec<_>>(),
        )
        .default(
            options
                .iter()
                .position(|&p| p == theme.separation)
                .unwrap_or(0),
        )
        .interact()
        .unwrap();
    theme.separation = options[selection];
}
