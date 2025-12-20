use zsh_seq::ZshPromptBuilder;

use crate::zsh::{prompt::PromptCurveLine, theme_manager};

pub async fn right() {
    let theme = theme_manager::load_theme();
    // PromptContentsのリストの最後の要素からテーマ情報を取得
    // リストが空の場合はデフォルトを使用
    let prompt_contents = theme
        .prompt_contents_list
        .last()
        .cloned()
        .unwrap_or_else(|| crate::zsh::theme::prompt_theme::PromptContents::default());

    let curved_lines = PromptCurveLine::from(prompt_contents.connection);
    let h = &curved_lines.horizontal;
    let b = &curved_lines.bottom_right;
    let builder = ZshPromptBuilder::new()
        .color(prompt_contents.color.sc)
        .str(h)
        .str(b)
        .end_color();

    println!("{}", builder.build());
}
