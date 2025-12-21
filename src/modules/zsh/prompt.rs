mod left;
mod right;
mod transient;
pub use left::left;
pub use right::right;
use serde::{Deserialize, Serialize};
use std::fmt;
pub use transient::transient;
use zsh_seq::ZshPromptBuilder;

use crate::zsh::theme::prompt_theme::PromptContents;

impl Prompt {
    fn left_separation(&self) -> usize {
        if self.left.is_empty() {
            0
        } else {
            self.left.len() + 1
        }
    }
    fn right_separation(&self) -> usize {
        if self.right.is_empty() {
            0
        } else {
            self.right.len() + 1
        }
    }
    fn total_separation(&self) -> usize {
        self.left_separation() + self.right_separation()
    }
    pub fn add_left(&mut self, content: &str) {
        self.left.push(content.to_string());
    }
    pub fn add_right(&mut self, content: &str) {
        self.right.push(content.to_string());
    }
    fn render_left_fg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        let color_scheme = &prompt_contents.color;
        let left_separators = &prompt_contents.left_segment_separators;
        let start_sep_color = color_scheme.accent.get(0.0);
        let bg_color = color_scheme.bg;
        let end_sep_color = color_scheme
            .accent
            .get(self.left_separation() as f32 / (self.total_separation() + 1) as f32);

        let start_cap = if prompt_contents.left_cap_enabled {
            let mut builder = ZshPromptBuilder::new()
                .end_color_bg()
                .color(start_sep_color);
            if left_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&left_separators.start_separator.sep_box().right);
            if left_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder = builder
                .end_color()
                .color_bg(start_sep_color)
                .color(bg_color);
            if left_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&left_separators.start_separator.sep_box().right);
            if left_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color_bg()
        } else {
            ZshPromptBuilder::new()
        };

        let end_cap = if prompt_contents.left_cap_enabled {
            let mut builder = ZshPromptBuilder::new()
                .end_color_bg()
                .end_color()
                .color_bg(end_sep_color)
                .color(bg_color);
            if left_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&left_separators.end_separator.sep_box().left);
            if left_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder = builder.end_color_bg().color(end_sep_color);
            if left_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&left_separators.end_separator.sep_box().left);
            if left_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color()
        } else {
            ZshPromptBuilder::new()
        };

        let mut builder = ZshPromptBuilder::new().connect(start_cap);
        builder = self
            .left
            .iter()
            .enumerate()
            .fold(builder, |mut b, (i, content)| {
                b = b.end_color().color_bg(bg_color).str(content).end_color();
                if i == self.left.len() - 1 {
                    b
                } else {
                    let mut mid_sep_builder = ZshPromptBuilder::new().color(
                        color_scheme
                            .accent
                            .get((i + 1) as f32 / (self.total_separation() + 1) as f32),
                    );
                    if left_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.bold();
                    }
                    mid_sep_builder =
                        mid_sep_builder.str(&left_separators.mid_separator.sep_line().left);
                    if left_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.end_bold();
                    }
                    b.connect(mid_sep_builder)
                }
            });
        builder = builder.connect(end_cap);
        builder
    }
    fn render_left_bg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.left.is_empty() {
            return ZshPromptBuilder::new();
        }
        let color_scheme = &prompt_contents.color;
        let left_separators = &prompt_contents.left_segment_separators;

        let start_cap = if prompt_contents.left_cap_enabled {
            let mut builder = ZshPromptBuilder::new().color(color_scheme.accent.get(0.0));
            if left_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&left_separators.start_separator.sep_box().right);
            if left_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color()
        } else {
            ZshPromptBuilder::new()
        };

        let content_len = self.left.len();
        let mut builder = ZshPromptBuilder::new().connect(start_cap);
        builder = self
            .left
            .iter()
            .enumerate()
            .fold(builder, |mut b, (i, content)| {
                let mut content_builder = ZshPromptBuilder::new()
                    .end_color()
                    .color_bg(color_scheme.accent.get(i as f32 / content_len as f32));
                if left_separators.separator_bold {
                    // content自体は太字にしないが、セパレータと一貫性を持たせるため
                    content_builder = content_builder.bold();
                }
                content_builder = content_builder.str(content);
                if left_separators.separator_bold {
                    content_builder = content_builder.end_bold();
                }
                b = b.connect(content_builder.end_color_bg());

                if i == content_len - 1 {
                    if prompt_contents.left_cap_enabled {
                        let mut end_cap_builder = ZshPromptBuilder::new()
                            .color(color_scheme.accent.get(i as f32 / content_len as f32));
                        if left_separators.separator_bold {
                            end_cap_builder = end_cap_builder.bold();
                        }
                        end_cap_builder =
                            end_cap_builder.str(&left_separators.end_separator.sep_box().left);
                        if left_separators.separator_bold {
                            end_cap_builder = end_cap_builder.end_bold();
                        }
                        b.connect(end_cap_builder.end_color())
                    } else {
                        b
                    }
                } else {
                    let mut mid_sep_builder = ZshPromptBuilder::new()
                        .color(color_scheme.accent.get(i as f32 / content_len as f32))
                        .color_bg(color_scheme.accent.get((i + 1) as f32 / content_len as f32));
                    if left_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.bold();
                    }
                    mid_sep_builder =
                        mid_sep_builder.str(&left_separators.mid_separator.sep_box().left);
                    if left_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.end_bold();
                    }
                    b.connect(mid_sep_builder.end_color().end_color_bg())
                }
            });
        builder
    }
    pub fn render_left(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        match prompt_contents.accent_which {
            crate::zsh::theme::prompt_theme::AccentWhich::ForeGround => {
                self.render_left_fg(prompt_contents)
            }
            crate::zsh::theme::prompt_theme::AccentWhich::BackGround => {
                self.render_left_bg(prompt_contents)
            }
        }
    }
    pub fn render_right_fg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.right.is_empty() {
            return ZshPromptBuilder::new();
        }
        let color_scheme = &prompt_contents.color;
        let right_separators = &prompt_contents.right_segment_separators;

        let bg_color = color_scheme.bg;
        // 右側の開始地点（左端）のセパレーター色
        let start_sep_color = color_scheme
            .accent
            .get(1.0 - self.right_separation() as f32 / (self.total_separation() + 1) as f32);
        // 右側の終了地点（右端）のセパレーター色
        let end_sep_color = color_scheme
            .accent
            .get(1.0 - 1.0 / (self.total_separation() + 1) as f32);

        // 右プロンプトの開始キャップ（左側の境界）
        let start_cap = if prompt_contents.right_cap_enabled {
            let mut builder = ZshPromptBuilder::new().color(start_sep_color);
            if right_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&right_separators.start_separator.sep_box().right);
            if right_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder = builder
                .end_color()
                .color_bg(start_sep_color)
                .color(bg_color);
            if right_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&right_separators.start_separator.sep_box().right);
            if right_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color()
        } else {
            ZshPromptBuilder::new()
        };

        // 右プロンプトの終了キャップ（右端の境界）
        let end_cap = if prompt_contents.right_cap_enabled {
            let mut builder = ZshPromptBuilder::new()
                .end_color_bg()
                .color_bg(end_sep_color)
                .color(bg_color);
            if right_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&right_separators.end_separator.sep_box().left);
            if right_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder = builder.end_color_bg().end_color().color(end_sep_color);
            if right_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&right_separators.end_separator.sep_box().left);
            if right_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color()
        } else {
            ZshPromptBuilder::new()
        };

        let mut builder = ZshPromptBuilder::new().connect(start_cap);

        // fold を使用して右側の要素を結合
        builder = self
            .right
            .iter()
            .enumerate()
            .fold(builder, |mut b, (i, content)| {
                b = b.color_bg(bg_color).str(content);

                // 最後の要素でなければセパレーターを追加
                if i == self.right.len() - 1 {
                    b
                } else {
                    // 色の計算位置を右側のオフセットに合わせる
                    let color_pos = (self.left_separation() + i + 2) as f32
                        / (self.total_separation() + 1) as f32;
                    let mut mid_sep_builder =
                        ZshPromptBuilder::new().color(color_scheme.accent.get(color_pos));
                    if right_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.bold();
                    }
                    mid_sep_builder =
                        mid_sep_builder.str(&right_separators.mid_separator.sep_line().right);
                    if right_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.end_bold();
                    }
                    b.connect(mid_sep_builder.end_color())
                }
                .end_color_bg()
            });

        builder.connect(end_cap)
    }
    pub fn render_right_bg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.right.is_empty() {
            return ZshPromptBuilder::new();
        }
        let color_scheme = &prompt_contents.color;
        let right_separators = &prompt_contents.right_segment_separators;

        let content_len = self.left.len() + self.right.len();
        let start_cap = if prompt_contents.right_cap_enabled {
            let mut builder = ZshPromptBuilder::new().color(
                color_scheme
                    .accent
                    .get((self.left.len() + 1) as f32 / content_len as f32),
            );
            if right_separators.separator_bold {
                builder = builder.bold();
            }
            builder = builder.str(&right_separators.start_separator.sep_box().right);
            if right_separators.separator_bold {
                builder = builder.end_bold();
            }
            builder.end_color()
        } else {
            ZshPromptBuilder::new()
        };

        let mut builder = ZshPromptBuilder::new().connect(start_cap);
        builder = self
            .right
            .iter()
            .enumerate()
            .fold(builder, |mut b, (i, content)| {
                let i_global = i + self.left.len() + 1; // 全体でのインデックス
                let mut content_builder = ZshPromptBuilder::new().end_color().color_bg(
                    color_scheme
                        .accent
                        .get(i_global as f32 / content_len as f32),
                );
                if right_separators.separator_bold {
                    content_builder = content_builder.bold();
                }
                content_builder = content_builder.str(content);
                if right_separators.separator_bold {
                    content_builder = content_builder.end_bold();
                }
                b = b.connect(content_builder.end_color_bg());

                if i_global == content_len {
                    if prompt_contents.right_cap_enabled {
                        let mut end_cap_builder = ZshPromptBuilder::new().color(
                            color_scheme
                                .accent
                                .get(i_global as f32 / content_len as f32),
                        );
                        if right_separators.separator_bold {
                            end_cap_builder = end_cap_builder.bold();
                        }
                        end_cap_builder =
                            end_cap_builder.str(&right_separators.end_separator.sep_box().left);
                        if right_separators.separator_bold {
                            end_cap_builder = end_cap_builder.end_bold();
                        }
                        b.connect(end_cap_builder.end_color())
                    } else {
                        b
                    }
                } else {
                    let mut mid_sep_builder = ZshPromptBuilder::new()
                        .color(
                            color_scheme
                                .accent
                                .get(i_global as f32 / content_len as f32),
                        )
                        .color_bg(
                            color_scheme
                                .accent
                                .get((i_global + 1) as f32 / content_len as f32),
                        );
                    if right_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.bold();
                    }
                    mid_sep_builder =
                        mid_sep_builder.str(&right_separators.mid_separator.sep_box().left);
                    if right_separators.separator_bold {
                        mid_sep_builder = mid_sep_builder.end_bold();
                    }
                    b.connect(mid_sep_builder.end_color().end_color_bg())
                }
            });
        builder
    }
    pub fn render_right(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        match prompt_contents.accent_which {
            crate::zsh::theme::prompt_theme::AccentWhich::ForeGround => {
                self.render_right_fg(prompt_contents)
            }
            crate::zsh::theme::prompt_theme::AccentWhich::BackGround => {
                self.render_right_bg(prompt_contents)
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Prompt {
    left: Vec<String>,
    right: Vec<String>,
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptConnection {
    #[default]
    None, // 空白
    Line,     // 標準の細線 (─)
    Double,   // 二重線 (═)
    Bold,     // 太線 (━)
    Dashed,   // 破線 (╌)
    Dotted,   // 点線 (┄)
    Dot,      // 中点 (·)
    Bullet,   // 弾丸 (•)
    Wave,     // 波線 (〜)
    ZigZag,   // ギザギザ (≈)
    Bar,      // 太いバー (█)
    Gradient, // グラデーション (░▒▓)
}

impl fmt::Display for PromptConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::None => " ",
            Self::Line => "─",
            Self::Double => "═",
            Self::Bold => "━",
            Self::Dashed => "╌",
            Self::Dotted => "┄",
            Self::Dot => "·",
            Self::Bullet => "•",
            Self::Wave => "~",
            Self::ZigZag => "≈",
            Self::Bar => "█",
            Self::Gradient => "▒",
        };
        write!(f, "{}", s)
    }
}
struct PromptCurveLine {
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
    horizontal: String, // 横線 ─
    #[allow(unused)]
    vertical: String, // 縦線 │
    cross_left: String, // 縦線から右に枝分かれ ├
    cross_right: String,
}
impl Default for PromptCurveLine {
    fn default() -> Self {
        Self {
            top_left: "╭".to_string(),
            top_right: "╮".to_string(),
            bottom_left: "╰".to_string(),
            bottom_right: "╯".to_string(),
            horizontal: "─".to_string(),
            vertical: "│".to_string(),
            cross_left: "├".to_string(),
            cross_right: "┤".to_string(),
        }
    }
}
impl From<PromptConnection> for PromptCurveLine {
    fn from(conn: PromptConnection) -> Self {
        match conn {
            // 二重線
            PromptConnection::Double => Self {
                top_left: "╔".to_string(),
                top_right: "╗".to_string(),
                bottom_left: "╚".to_string(),
                bottom_right: "╝".to_string(),
                horizontal: "═".to_string(),
                vertical: "║".to_string(),
                cross_left: "╠".to_string(),
                cross_right: "╣".to_string(),
            },
            // 太線
            PromptConnection::Bold | PromptConnection::Bar => Self {
                top_left: "┏".to_string(),
                top_right: "┓".to_string(),
                bottom_left: "┗".to_string(),
                bottom_right: "┛".to_string(),
                horizontal: "━".to_string(),
                vertical: "┃".to_string(),
                cross_left: "┣".to_string(),
                cross_right: "┫".to_string(),
            },
            // 標準の直角
            PromptConnection::Line | PromptConnection::Dashed | PromptConnection::Dotted => Self {
                top_left: "┌".to_string(),
                top_right: "┐".to_string(),
                bottom_left: "└".to_string(),
                bottom_right: "┘".to_string(),
                horizontal: "─".to_string(),
                vertical: "│".to_string(),
                cross_left: "├".to_string(),
                cross_right: "┤".to_string(),
            },
            // 丸角（デフォルト）
            _ => Self {
                top_left: "╭".to_string(),
                top_right: "╮".to_string(),
                bottom_left: "╰".to_string(),
                bottom_right: "╯".to_string(),
                horizontal: conn.to_string(),
                vertical: "│".to_string(),
                cross_left: "├".to_string(),
                cross_right: "┤".to_string(),
            },
        }
    }
}

#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptSeparation {
    Block,
    #[default]
    Sharp, // 三角形 (Powerline Default)
    Slash,     // 斜線
    BackSlash, // 逆斜線
    Round,     // 半円
    Blur,      // グラデーション
    Flame,     // 炎
    Pixel,     // ドット/ピクセル
    Wave,      // 波形
    Lego,      // レゴブロック風
}
pub struct PromptSeparationBox {
    pub left: String,
    pub right: String,
}
pub struct PromptSeparationLine {
    pub left: String,
    pub right: String,
}
impl PromptSeparationBox {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}
impl PromptSeparationLine {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}
impl From<PromptSeparation> for PromptSeparationBox {
    fn from(value: PromptSeparation) -> Self {
        value.sep_box()
    }
}
impl From<PromptSeparation> for PromptSeparationLine {
    fn from(value: PromptSeparation) -> Self {
        value.sep_line()
    }
}
impl PromptSeparation {
    pub fn sep_box(&self) -> PromptSeparationBox {
        match self {
            Self::Block => PromptSeparationBox::new(" ", " "),
            Self::Sharp => PromptSeparationBox::new("", ""), // Powerline三角形
            Self::Slash => PromptSeparationBox::new("", ""), // 斜線
            Self::BackSlash => PromptSeparationBox::new("", ""), // 逆斜線
            Self::Round => PromptSeparationBox::new("", ""), // 半円
            Self::Blur => PromptSeparationBox::new("▓▒░", "░▒▓"), // グラデ
            Self::Flame => PromptSeparationBox::new("", ""), // 炎
            Self::Pixel => PromptSeparationBox::new("", ""), // ピクセル
            Self::Wave => PromptSeparationBox::new("", ""),  // 波
            Self::Lego => PromptSeparationBox::new("", ""),  // (代替)
        }
    }
    pub fn sep_line(&self) -> PromptSeparationLine {
        match self {
            Self::Block => PromptSeparationLine::new("|", "|"),
            Self::Sharp => PromptSeparationLine::new("", ""), // 細い三角形
            Self::Slash => PromptSeparationLine::new("╱", "╱"), // 細い斜線
            Self::BackSlash => PromptSeparationLine::new("╲", "╲"), // 細い逆斜線
            Self::Round => PromptSeparationLine::new("", ""), // 細い半円
            Self::Blur => PromptSeparationLine::new("░", "░"),  // 薄い網掛け
            Self::Flame => PromptSeparationLine::new("", ""), // 細い炎
            Self::Pixel => PromptSeparationLine::new("", ""), // 細いピクセル
            Self::Wave => PromptSeparationLine::new("", ""),  // 細い波
            Self::Lego => PromptSeparationLine::new("", ""),  // (代替)
        }
    }
}
