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
        command: String,
    },
    /// Update the specific memorized command
    Set {
        /// Specific command to be memorized
        #[arg(short, long)]
        command: String,

        /// Set alias for a command
        #[arg(short, long)]
        alias: String,

        /// Set new value for to be updated command
        #[arg(short, long)]
        to: String,
    },
    /// Execute the target memorized command by its alias
    Use {
        /// Set alias for a command
        #[arg(short, long)]
        alias: String,
    },
    /// Show a list of memorized commands and its alias
    List,
}
