use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my_tool", version, about = "CLI tool with nested subcommands")]
pub struct Args {
    /// Show verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// zsh related commands
    #[command(hide = true)]
    Zsh {
        #[command(subcommand)]
        command: ZshCommands,
    },
    /// Update the application
    Update,
    /// Install a component
    Install,
    /// Uninstall a component
    Uninstall,
    /// debug theme
    #[cfg(debug_assertions)]
    Dev,
    /// Manage Zsh theme
    Theme,
}

#[derive(Subcommand)]
pub enum ZshCommands {
    /// Prompt related commands
    Prompt {
        #[command(subcommand)]
        side: PromptType,
    },
}

#[derive(Subcommand, Clone, Copy, PartialEq, Eq)]
pub enum PromptType {
    /// Generate left prompt
    Left,
    /// Generate right prompt
    Right,
    /// Generate transient prompt
    Transient {
        /// Exit code of the previous command
        #[arg(long, short = 'e')]
        exit_code: Option<i32>,
    },
}
