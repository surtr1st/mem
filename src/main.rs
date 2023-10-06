mod subcommands;
use anyhow::{Error, Result};
use clap::{command, Parser};
use memo::{MemorizeBox, MemorizeUtils};
use subcommands::MemorizeSubcommands;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A CLI tool used for `memorize` command-lines", long_about = None)]
#[command(arg_required_else_help = true)]
struct Memorize {
    #[command(subcommand)]
    subcommands: Option<MemorizeSubcommands>,

    /// Alias for `x` usage
    alias: Option<String>,

    /// Executing a memorized command
    x: Option<String>,
}

fn main() -> Result<()> {
    let mem = Memorize::parse();

    MemorizeUtils::validate_default_path()?;

    let exec = mem.x;
    let x_alias = mem.alias;

    if let Some(_) = exec {
        let Some(alias) = &x_alias else {
            return Err(Error::msg("You haven't choose a (an) alias/command!"));
        };
        MemorizeUtils::invoke_command(&alias, &None)?;
        return Ok(());
    }

    match &mem.subcommands {
        Some(MemorizeSubcommands::Add { alias, command }) => {
            let memo_box = MemorizeBox {
                alias: alias.to_owned(),
                command: command.to_owned(),
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
        Some(MemorizeSubcommands::Use { alias, value }) => {
            let Some(a) = alias else {
                return Err(Error::msg("You haven't choose a (an) alias/command!"));
            };
            MemorizeUtils::invoke_command(&a, &value)?;
            Ok(())
        }
        Some(MemorizeSubcommands::List) => {
            MemorizeUtils::collect()?;
            Ok(())
        }
        None => {
            let Some(alias) = x_alias else {
                return Err(Error::msg("You haven't choose a (an) alias/command!"));
            };
            MemorizeUtils::invoke_command(&alias, &None)?;
            Ok(())
        }
    }
}
