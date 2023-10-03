mod subcommands;
use clap::{command, Parser};
use subcommands::MemorizeSubcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Memorize {
    #[command(subcommand)]
    subcommands: Option<MemorizeSubcommands>,
}

fn main() -> Result<(), String> {
    let mem = Memorize::parse();

    match &mem.subcommands {
        Some(MemorizeSubcommands::Add { .. }) => Ok(()),
        Some(MemorizeSubcommands::Del { .. }) => Ok(()),
        Some(MemorizeSubcommands::Set { .. }) => Ok(()),
        Some(MemorizeSubcommands::Use { .. }) => Ok(()),
        Some(MemorizeSubcommands::List) => Ok(()),
        None => Ok(()),
    }
}
