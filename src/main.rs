mod subcommands;
use anyhow::{Error, Result};
use clap::{command, Parser};
use mem::{MemorizeBox, MemorizeUtils};
use subcommands::{GlobalArgs, MemorizeSubcommands};

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI tool used for `memorize` command-lines", long_about = None)]
struct Memorize {
    #[command(subcommand)]
    subcommands: Option<MemorizeSubcommands>,

    #[command(flatten)]
    options: GlobalArgs,
}

fn main() -> Result<()> {
    let mem = Memorize::parse();

    MemorizeUtils::validate_default_path()?;

    let alias = mem.options.alias;
    let command = mem.options.command;

    match &mem.subcommands {
        Some(MemorizeSubcommands::Add) => {
            let Some(a) = alias else {
                return Err(Error::msg("Alias is empty!"));
            };
            let Some(c) = command else {
                return Err(Error::msg("Command is empty!"));
            };
            let memo_box = MemorizeBox {
                alias: a.to_string(),
                command: c.to_string(),
            };
            MemorizeUtils::add(&memo_box)?;
            Ok(())
        }
        Some(MemorizeSubcommands::Del) => {
            let Some(a) = alias else {
                return Err(Error::msg("Alias is empty!"));
            };
            MemorizeUtils::delete_command_by_alias(&a)?;
            Ok(())
        }
        Some(MemorizeSubcommands::Set {
            new_alias,
            new_command,
        }) => {
            let Some(a) = alias else {
                return Err(Error::msg("Alias is empty!"));
            };
            if let Some(na) = new_alias {
                MemorizeUtils::update_by_alias(&a, na)?;
            }
            if let Some(nc) = new_command {
                MemorizeUtils::update_command_by_alias(&a, nc)?;
            }
            Ok(())
        }
        Some(MemorizeSubcommands::Use { value }) => {
            let Some(a) = alias else {
                return Err(Error::msg("Alias is empty!"));
            };
            MemorizeUtils::invoke_command(&a, value)?;
            Ok(())
        }
        Some(MemorizeSubcommands::List) => {
            MemorizeUtils::collect()?;
            Ok(())
        }
        None => Ok(()),
    }
}
