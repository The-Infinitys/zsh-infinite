use zsh_seq::{ZshPromptBuilder, ZshSequence};

use crate::zsh::{
    prompt::{PromptConnection, PromptCurveLine},
    theme_manager,
};
pub fn right() {
    let curved_lines = PromptCurveLine::default();
    let l = PromptConnection::Line.to_string();
    let theme = theme_manager::load_theme();
    let prompt = ZshPromptBuilder::new()
        .add_sequence(ZshSequence::ForegroundColor(theme.color.sc))
        .add_sequence(ZshSequence::Literal(l))
        .add_sequence(ZshSequence::Literal(curved_lines.bottom_right.to_string()))
        .add_sequence(ZshSequence::ForegroundColorEnd);
    print!("{}", prompt.build());
}
