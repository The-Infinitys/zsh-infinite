use zsh_seq::ZshPromptBuilder;

use crate::zsh::{
    prompt::{PromptConnection, PromptCurveLine},
    theme_manager,
};
pub fn left() {
    let curved_lines = PromptCurveLine::default();
    let l = PromptConnection::Line.to_string();
    let theme = theme_manager::load_theme();
    let prompt = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(&curved_lines.top_left)
        .str(&l)
        .end_color()
        .newline()
        .color(theme.color.sc)
        .str(&curved_lines.bottom_left)
        .str(&l)
        .str(" ")
        .end_color();
    print!("{}", prompt.build());
}
