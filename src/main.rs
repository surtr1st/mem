use clap::{arg, command, Parser};
use std::{env, fs, fs::File, path::Path};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Memorize {
    #[arg(short, long)]
    alias: String,

    #[arg(short, long)]
    path: std::path::PathBuf,
}

const HOME: &str = "HOME";
const DEFAULT_PATH: &str = ".local/share/mem";

fn main() {
    let args = Memorize::parse();
    let file_name = args.path.file_name().unwrap().to_string_lossy().to_string();
    let file_content = fs::read_to_string(args.path)
        .expect(&format!("should been read the content of {}", &file_name));
    for content in file_content.lines() {
        println!("{content}");
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
