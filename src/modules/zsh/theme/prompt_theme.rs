use serde::{Deserialize, Serialize};
use tokio::process::Command;

use super::color_scheme::PromptColorScheme;
use crate::zsh::prompt::{PromptConnection, PromptSeparation};

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy)]
pub enum AccentWhich {
    #[default]
    ForeGround,
    BackGround,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub prompt_contents_list: Vec<PromptContents>,
    pub transient_color: PromptColorScheme,
}
impl Default for PromptTheme {
    fn default() -> Self {
        Self {
            prompt_contents_list: vec![PromptContents::default()],
            transient_color: PromptColorScheme::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PromptSegmentSeparators {
    pub start_separator: PromptSeparation,
    pub mid_separator: PromptSeparation,
    pub end_separator: PromptSeparation,
}

impl Default for PromptSegmentSeparators {
    fn default() -> Self {
        Self {
            start_separator: PromptSeparation::Sharp,
            mid_separator: PromptSeparation::Sharp,
            end_separator: PromptSeparation::Sharp,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContents {
    pub left: Vec<PromptContent>,
    pub right: Vec<PromptContent>,
    pub color: super::color_scheme::PromptColorScheme,
    pub connection: PromptConnection,
    pub left_segment_separators: PromptSegmentSeparators,
    pub right_segment_separators: PromptSegmentSeparators,
    pub accent_which: AccentWhich,
}

impl Default for PromptContents {
    fn default() -> Self {
        Self {
            left: vec![
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "whoami".to_string(),
                ]),
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "hostname".to_string(),
                ]),
            ],
            right: vec![
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "echo ${PWD/#$HOME/\\~}".to_string(),
                ]),
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "echo $?".to_string(),
                ]),
            ],
            color: super::color_scheme::PromptColorScheme::default(),
            connection: PromptConnection::default(),
            left_segment_separators: PromptSegmentSeparators::default(),
            right_segment_separators: PromptSegmentSeparators::default(),
            accent_which: AccentWhich::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContent {
    shell: Vec<String>,
}

impl PromptContent {
    pub fn new(shell: Vec<String>) -> Self {
        Self { shell }
    }
    pub async fn content(&self) -> Option<String> {
        if self.shell.is_empty() {
            return None;
        }
        let output = Command::new(&self.shell[0])
            .args(&self.shell[1..])
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                None
            } else {
                Some(stdout)
            }
        } else {
            None
        }
    }
}
