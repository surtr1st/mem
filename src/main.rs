mod constants;
use clap::{arg, command, Parser, Subcommand};
use constants::{DEFAULT_PATH, HOME};
use std::{env, fs, fs::File, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Memorize {
    #[command(subcommand)]
    command: Option<MemorizeSubcommands>,
}

#[derive(Subcommand, Debug)]
enum MemorizeSubcommands {
    /// Adding and memorize command
    Add {
        /// Specific command to be memorized
        #[arg(short, long)]
        command: Option<String>,

        /// Set alias for a command
        #[arg(short, long)]
        alias: Option<String>,
    },
    /// Delete the specific memorized command
    Del {
        /// Specific command to be memorized
        #[arg(short, long)]
        command: Option<String>,
    },
    /// Update the specific memorized command
    Set {
        /// Specific command to be memorized
        #[arg(short, long)]
        command: Option<String>,
    },
    /// Execute the target memorized command by its alias
    Use {
        /// Set alias for a command
        #[arg(short, long)]
        alias: Option<String>,
    },
    /// Show a list of memorized commands and its alias
    List,
}

fn main() -> Result<(), String> {
    let mem = Memorize::parse();
    match &mem.command {
        Some(MemorizeSubcommands::Add { alias, command }) => {
            let Some(_) = alias else {
                let message = format!(
                    "Alias for executing command haven't set! For instance `mem add --command env $HOME --alias ehe`"
                );
                return Err(message);
            };
            let Some(_) = command else {
                let message =
                    format!("Please add a specific command rather whitespace or non-command!");
                return Err(message);
            };
            Ok(())
        }
        Some(MemorizeSubcommands::Del { command }) => {
            let Some(_) = command else {
                let message =
                    format!("Please add a specific command rather whitespace or non-command!");
                return Err(message);
            };
            Ok(())
        }
        Some(MemorizeSubcommands::Set { command }) => Ok(()),
        Some(MemorizeSubcommands::Use { alias }) => Ok(()),
        Some(MemorizeSubcommands::List) => Ok(()),
        None => Ok(()),
    }
}

pub fn write_into(file_name: &str) -> Result<(), std::io::Error> {
    let home_dir = env::var(HOME).expect("should been read `HOME` value");
    let path = format!("{home_dir}/{DEFAULT_PATH}");
    let file_path = format!("{home_dir}/{DEFAULT_PATH}/{file_name}");
    let is_dir = Path::new(&path).is_dir();
    let is_file = Path::new(&file_path).is_file();
    if !is_dir {
        fs::create_dir(&path)?;
    }
    if !is_file {
        File::create(path)?;
    }
    Ok(())
}
