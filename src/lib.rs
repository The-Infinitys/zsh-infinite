#![feature(trait_alias)]
mod modules;
use crate::modules::zsh::theme::{self, prompt_theme::PromptTheme};
pub use modules::*;
use once_cell::sync::Lazy;

static PROMPT_THEME: Lazy<PromptTheme> = Lazy::new(theme::manager::load_theme);

fn prompt_theme() -> &'static PromptTheme {
    &PROMPT_THEME
}

#[cfg(feature = "zsh-module")]
mod zmod;
#[cfg(feature = "zsh-module")]
use zmod::setup;
#[cfg(feature = "zsh-module")]
zsh_module::export_module!(zsh_infinite, setup);
