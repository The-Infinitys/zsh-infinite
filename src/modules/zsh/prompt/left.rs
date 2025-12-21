use crate::zsh::{
    prompt::{Prompt, PromptCurveLine},
    theme_manager,
};
use crossterm::terminal;
use futures::future::join_all;
use unicode_width::UnicodeWidthStr;
use zsh_seq::ZshPromptBuilder;

pub async fn left() {
    let theme = theme_manager::load_theme();

    // 1. リストが空の場合の早期リターン（またはデフォルト表示）
    if theme.prompt_contents_list.is_empty() {
        // デフォルトのPromptContentsから設定を取得
        let default_prompt_contents = crate::zsh::theme::prompt_theme::PromptContents::default();
        let curved_lines = PromptCurveLine::from(default_prompt_contents.connection);
        let h = &curved_lines.horizontal;

        let mut start_builder = ZshPromptBuilder::new().color(default_prompt_contents.color.sc);
        if default_prompt_contents.left_cap_enabled {
            start_builder = start_builder.str(&curved_lines.top_left);
        }
        start_builder = start_builder.str(h).str(h);
        if default_prompt_contents.right_cap_enabled {
            start_builder = start_builder.str(&curved_lines.top_right);
        }
        start_builder = start_builder.end_color();
        println!("{}", start_builder.build());

        let mut end_builder = ZshPromptBuilder::new().color(default_prompt_contents.color.sc);
        if default_prompt_contents.left_cap_enabled {
            end_builder = end_builder.str(&curved_lines.bottom_left);
        }
        end_builder = end_builder.str(h).str(" ").end_color();
        print!("{}", end_builder.build());
        return;
    }

    // 2. リストがある場合のメインループ
    for (i, prompt_contents) in theme.prompt_contents_list.iter().enumerate() {
        let mut prompt = Prompt::default();
        let curved_lines = PromptCurveLine::from(prompt_contents.connection);
        let h = &curved_lines.horizontal;

        // (非同期取得部分は変更なし)
        let left_futures: Vec<_> = prompt_contents
            .left
            .iter()
            .map(|c| async move { c.content().await })
            .collect();
        let right_futures: Vec<_> = prompt_contents
            .right
            .iter()
            .map(|c| async move { c.content().await })
            .collect();
        let (left_results, right_results) =
            tokio::join!(join_all(left_futures), join_all(right_futures));

        for content in left_results.into_iter().flatten() {
            prompt.add_left(&content);
        }
        for content in right_results.into_iter().flatten() {
            prompt.add_right(&content);
        }

        let left_content = prompt.render_left(prompt_contents);
        let right_content = prompt.render_right(prompt_contents);
        let terminal_width = terminal::size().map(|(w, _)| w).unwrap_or(80) as usize;
        let left_width = left_content.len();
        let right_width = right_content.len();
        let conn_line_width =
            UnicodeWidthStr::width(prompt_contents.connection.to_string().as_str());

        // キャップが有効な場合にのみサイドデコレーションの幅を考慮
        let mut side_decor_width = 0;
        if prompt_contents.left_cap_enabled {
            side_decor_width += UnicodeWidthStr::width(curved_lines.top_left.as_str());
        }
        if prompt_contents.right_cap_enabled {
            side_decor_width += UnicodeWidthStr::width(curved_lines.top_right.as_str());
        }
        side_decor_width += conn_line_width; // Horizontal line segment between cap and content

        let connection_len =
            terminal_width.saturating_sub(left_width + right_width + side_decor_width);

        let connection_str = prompt_contents
            .connection
            .to_string()
            .repeat(connection_len / conn_line_width);

        eprintln!("left: {}", left_width);
        eprintln!("right {}", right_width);
        eprintln!("{}", connection_len / conn_line_width);

        let mut row_builder = ZshPromptBuilder::new();
        row_builder = row_builder.color(prompt_contents.color.sc);

        // 左キャップの描画
        if prompt_contents.left_cap_enabled {
            if i == 0 {
                row_builder = row_builder.str(&curved_lines.top_left);
            } else {
                row_builder = row_builder.str(&curved_lines.cross_left);
            }
        }
        row_builder = row_builder.str(h); // キャップの有無にかかわらず水平線は描画

        let mut final_prompt = row_builder
            .end_color()
            .connect(left_content)
            .color(prompt_contents.color.pc)
            .str(&connection_str)
            .end_color()
            .connect(right_content)
            .color(prompt_contents.color.sc);

        final_prompt = final_prompt.str(h); // キャップの有無にかかわらず水平線は描画
        // 右キャップの描画
        if prompt_contents.right_cap_enabled {
            final_prompt = final_prompt.str(if i == 0 {
                &curved_lines.top_right
            } else {
                &curved_lines.cross_right
            });
        }
        final_prompt = final_prompt.end_color();

        println!("{}", final_prompt.build());
    }

    // 最終行の描画
    // prompt_contents_listが空でない場合、最後のPromptContentsの設定を使用
    let last_prompt_contents = theme
        .prompt_contents_list
        .last()
        .cloned()
        .unwrap_or_else(crate::zsh::theme::prompt_theme::PromptContents::default);
    let curved_lines = PromptCurveLine::from(last_prompt_contents.connection);
    let h = &curved_lines.horizontal;

    let mut end_builder = ZshPromptBuilder::new().color(last_prompt_contents.color.sc);
    if last_prompt_contents.left_cap_enabled {
        end_builder = end_builder.str(&curved_lines.bottom_left);
    }
    end_builder = end_builder.str(h).str(" ").end_color();
    print!("{}", end_builder.build());
}
