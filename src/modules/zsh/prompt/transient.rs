use crate::zsh::theme_manager;
use zsh_seq::{ZshPromptBuilder, ZshSequence, NamedColor};

pub fn transient(exit_code: Option<i32>) {
    let theme = theme_manager::load_theme();
    let transient_str = "❯ ";

    let color = match exit_code {
        Some(0) => theme.color.pc,
        _ => NamedColor::Red,
    };

    let prompt = ZshPromptBuilder::new()
        .add_sequence(ZshSequence::ForegroundColor(color))
        .add_sequence(ZshSequence::Literal(transient_str.to_string()))
        .add_sequence(ZshSequence::ForegroundColorEnd)
        .add_sequence(ZshSequence::ResetStyles);

    print!("{}", prompt.build());
}
