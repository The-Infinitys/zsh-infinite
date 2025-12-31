use tokio::runtime::Runtime;
use zsh_system::{Features, ZshModule, ZshParameter, ZshResult, export_module};

use crate::{args::PromptType, zsh};

struct ZshInfinite {
    rt: Option<Runtime>,
    old_prompt: String,
    old_rprompt: String,
}

impl Default for ZshInfinite {
    fn default() -> Self {
        Self {
            rt: None,
            old_prompt: String::new(),
            old_rprompt: String::new(),
        }
    }
}

impl ZshInfinite {
    pub fn precmd(&mut self) -> ZshResult {
        if self.rt.is_none() {
            self.rt = Some(Runtime::new().unwrap());
        }
        let rt = self.rt.as_ref().unwrap();

        let left_prompt = rt.block_on(async { zsh::build_prompt(&PromptType::Left).await.build() });
        let right_prompt =
            rt.block_on(async { zsh::build_prompt(&PromptType::Right).await.build() });
        ZshParameter::set_str("PROMPT", &left_prompt)?;
        ZshParameter::set_str("RPROMPT", &right_prompt)?;
        Ok(())
    }
}
impl ZshModule for ZshInfinite {
    fn setup(&mut self) -> ZshResult {
        Ok(())
    }

    fn boot(&mut self) -> ZshResult {
        self.rt = Some(Runtime::new().unwrap());
        self.old_prompt = ZshParameter::get_str("PROMPT").unwrap_or(String::new());
        self.old_rprompt = ZshParameter::get_str("RPROMPT").unwrap_or(String::new());
        zsh_system::eval(include_str!("assets/scripts/zmod/boot.sh"));
        Ok(())
    }

    fn features(&self) -> Features {
        Features::new().add_builtin("__zsh_infinite_internal_precmd", |_, _| {
            match ZshInfinite::with_instance(|zsh_infinite| zsh_infinite.precmd()) {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("ZshInfinite::precmd failed: {}", e.to_string());
                    1
                }
            }
        })
    }

    fn cleanup(&mut self) -> ZshResult {
        zsh_system::eval(include_str!("assets/scripts/zmod/cleanup.sh"));
        self.rt = None;
        ZshParameter::set_str("PROMPT", &self.old_prompt)?;
        ZshParameter::set_str("RPROMPT", &self.old_rprompt)?;
        eprintln!("[ZshInfinite] Module unloaded successfully.");
        Ok(())
    }
    fn finish(&mut self) -> ZshResult {
        Ok(())
    }
}

// マクロでエクスポート
export_module!(ZshInfinite);
