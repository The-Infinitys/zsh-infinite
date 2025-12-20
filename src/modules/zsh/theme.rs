pub mod named_color_serde; // 既存のファイルをそのまま使用

pub mod prompt_theme;
pub mod color_scheme;
pub mod gradient;
pub mod config_ui;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::zsh::theme_manager;

pub async fn main() {
    let mut current_theme = theme_manager::load_theme();
    loop {
        println!("\n--- Zsh Infinite Theme Configuration ---");
        let options = [
            "Configure Colors",
            "Configure Connection",
            "Configure Separators",
            "Save and Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main Menu")
            .items(options)
            .interact()
            .unwrap();

        match selection {
            0 => config_ui::configure_colors(&mut current_theme),
            1 => config_ui::configure_connection(&mut current_theme),
            2 => config_ui::configure_separation(&mut current_theme),
            3 => {
                let _ = theme_manager::save_theme(&current_theme);
                break;
            }
            _ => unreachable!(),
        }
    }
}