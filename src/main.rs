mod constants;
mod subcommands;
use clap::{command, Parser};
use constants::{DEFAULT_PATH, HOME};
use std::{env, fs, fs::File, path::Path};
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
