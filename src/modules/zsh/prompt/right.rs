use zsh_seq::ZshPromptBuilder;

use crate::zsh::{
    prompt::{PromptConnection, PromptCurveLine},
    theme_manager,
};

pub async fn right() {
    let curved_lines = PromptCurveLine::default();
    let l = PromptConnection::Line.to_string();
    let theme = theme_manager::load_theme();
    let prompt = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(&l)
        .str(curved_lines.bottom_right.as_str())
        .end_color();
    print!("{}", prompt.build());
}
