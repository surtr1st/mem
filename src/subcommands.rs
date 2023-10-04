use clap::{arg, command, Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum MemorizeSubcommands {
    /// Adding and memorize command
    #[command(arg_required_else_help = true)]
    Add,
    /// Delete the specific memorized command
    #[command(arg_required_else_help = true)]
    Del,
    /// Update the specific memorized command
    #[command(arg_required_else_help = true)]
    Set {
        /// Set new value for alias
        #[arg(short = 'n', long)]
        new_alias: Option<String>,

        /// Set new value for command
        #[arg(short = 'N', long)]
        new_command: Option<String>,
    },
    /// Execute the target memorized command by its alias
    #[command(arg_required_else_help = true)]
    Use {
        /// Set value for a command
        #[arg(short = 'v', long)]
        value: String,
    },
    /// Show a list of memorized commands and its alias
    List,
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Specific command to be memorized
    #[arg(short, long, global = true)]
    pub command: Option<String>,

    /// Set alias for a command
    #[arg(short, long, global = true)]
    pub alias: Option<String>,
}
