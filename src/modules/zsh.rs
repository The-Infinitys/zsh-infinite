use crate::args::{PromptType, ZshCommands};

mod prompt;
pub mod theme; // Make theme public
mod theme_manager;
pub use theme_manager::{load_theme, save_theme};

pub fn main(command: ZshCommands) {
    match command {
        ZshCommands::Prompt { side } => match side {
            PromptType::Left => prompt::left(),
            PromptType::Right => prompt::right(),
            PromptType::Transient { exit_code } => prompt::transient(exit_code),
        },
    }
}
