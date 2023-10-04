use clap::{arg, Subcommand};

#[derive(Subcommand, Debug)]
pub enum MemorizeSubcommands {
    /// Adding and memorize command
    Add {
        /// Specific command to be memorized
        #[arg(short, long)]
        command: String,

        /// Set alias for a command
        #[arg(short, long)]
        alias: String,
    },
    /// Delete the specific memorized command
    Del {
        /// Specific command to be memorized
        #[arg(short, long)]
        alias: String,
    },
    /// Update the specific memorized command
    Set {
        /// Set alias for a command
        #[arg(short, long)]
        alias: String,

        /// Set new value for alias
        #[arg(short = 'n', long)]
        new_alias: Option<String>,

        /// Set new value for command
        #[arg(short = 'N', long)]
        new_command: Option<String>,
    },
    /// Execute the target memorized command by its alias
    Use {
        /// Set alias for a command
        alias: String,

        /// Set value for a command
        #[arg(short = 'v', long)]
        value: String,
    },
    /// Show a list of memorized commands and its alias
    List,
}
