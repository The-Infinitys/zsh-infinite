mod left;
mod right;
mod transient;
use crate::{args::PromptType, zsh::theme::PromptTheme};
pub use left::left;
pub use right::right;
use serde::{Deserialize, Serialize};
pub use transient::transient;

impl Prompt {
    pub fn add_left(&mut self, content: &str) {
        self.left.push(content.to_string());
    }
    pub fn add_right(&mut self, content: &str) {
        self.right.push(content.to_string());
    }
    pub fn render_left() -> String {
        "".to_string()
    }
    pub fn render_right() -> String {
        "".to_string()
    }
}

#[derive(Clone)]
pub struct Prompt {
    pub left: Vec<String>,
    pub right: Vec<String>,
    pub theme: PromptTheme,
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptConnection {
    None,
    #[default]
    Line,
    Dot,
}
impl ToString for PromptConnection {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::None => " ",
            Self::Line => "─",
            Self::Dot => "·",
        })
    }
}
struct PromptCurveLine {
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
}
impl Default for PromptCurveLine {
    fn default() -> Self {
        let top_left = "╭".to_string();
        let top_right = "╮".to_string();
        let bottom_left = "╰".to_string();
        let bottom_right = "╯".to_string();
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptSeparation {
    Block,
    #[default]
    Sharp,
    Slash,
    Round,
    Blur,
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
            Self::Slash => PromptSeparationBox::new("", ""),
            Self::Block => PromptSeparationBox::new(" ", " "),
            Self::Sharp => PromptSeparationBox::new("", ""),
            Self::Round => PromptSeparationBox::new("", ""),
            Self::Blur => PromptSeparationBox::new("▓▒░", "░▒▓"),
        }
    }
    pub fn sep_line(&self) -> PromptSeparationLine {
        match self {
            Self::Slash => PromptSeparationLine::new("╱", "╱"),
            Self::Block => PromptSeparationLine::new("|", "|"),
            Self::Sharp => PromptSeparationLine::new("", ""),
            Self::Round => PromptSeparationLine::new("", ""),
            Self::Blur => PromptSeparationLine::new("▓▒░", "░▒▓"),
        }
    }
}
