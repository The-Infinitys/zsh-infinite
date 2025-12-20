use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::zsh::prompt::{PromptConnection, PromptSeparation};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub color: super::color_scheme::PromptColorScheme,
    pub connection: PromptConnection,
    pub separation: PromptSeparation,
    pub prompt_contents_list: Vec<PromptContents>,
}
impl Default for PromptTheme {
    fn default() -> Self {
        Self {
            color: super::color_scheme::PromptColorScheme::default(),
            connection: PromptConnection::default(),
            separation: PromptSeparation::default(),
            prompt_contents_list: vec![PromptContents::default()],
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContents {
    pub left: Vec<PromptContent>,
    pub right: Vec<PromptContent>,
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
