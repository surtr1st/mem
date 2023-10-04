mod subcommands;
use clap::{command, Parser};
use mem::{MemorizeBox, MemorizeUtils};
use subcommands::MemorizeSubcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Memorize {
    #[command(subcommand)]
    subcommands: Option<MemorizeSubcommands>,
}

fn main() -> Result<(), String> {
    let mem = Memorize::parse();

    MemorizeUtils::validate_default_path()?;

    match &mem.subcommands {
        Some(MemorizeSubcommands::Add { alias, command }) => {
            let memo_box = MemorizeBox {
                alias: alias.to_string(),
                command: command.to_string(),
            };
            MemorizeUtils::update(&memo_box)?;
            Ok(())
        }
        Some(MemorizeSubcommands::Del { .. }) => Ok(()),
        Some(MemorizeSubcommands::Set { .. }) => Ok(()),
        Some(MemorizeSubcommands::Use { .. }) => Ok(()),
        Some(MemorizeSubcommands::List) => {
            MemorizeUtils::collect()?;
            Ok(())
        }
        None => Ok(()),
    }
}
