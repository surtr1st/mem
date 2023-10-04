mod subcommands;
use anyhow::Result;
use clap::{command, Parser};
use mem::{MemorizeBox, MemorizeUtils};
use subcommands::MemorizeSubcommands;

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI tool used for `memorize` command-lines", long_about = None)]
struct Memorize {
    #[command(subcommand)]
    subcommands: Option<MemorizeSubcommands>,
}

fn main() -> Result<()> {
    let mem = Memorize::parse();

    MemorizeUtils::validate_default_path()?;

    match &mem.subcommands {
        Some(MemorizeSubcommands::Add { alias, command }) => {
            let memo_box = MemorizeBox {
                alias: alias.to_string(),
                command: command.to_string(),
            };
            MemorizeUtils::add(&memo_box)?;
            Ok(())
        }
        Some(MemorizeSubcommands::Del { alias }) => {
            MemorizeUtils::delete_command_by_alias(&alias)?;
            Ok(())
        }
        Some(MemorizeSubcommands::Set {
            alias,
            new_alias,
            new_command,
        }) => {
            if let Some(na) = new_alias {
                if let Some(a) = alias {
                    MemorizeUtils::update_by_alias(a, na)?;
                }
            }

            if let Some(nc) = new_command {
                if let Some(a) = alias {
                    MemorizeUtils::update_command_by_alias(a, nc)?;
                }
            }

            Ok(())
        }
        Some(MemorizeSubcommands::Use { alias }) => {
            MemorizeUtils::invoke_command(&alias)?;
            Ok(())
        }
        Some(MemorizeSubcommands::List) => {
            MemorizeUtils::collect()?;
            Ok(())
        }
        None => Ok(()),
    }
}
