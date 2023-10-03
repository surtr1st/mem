mod constants;
use constants::{DEFAULT_PATH, HOME};
use std::{env, fs, fs::File, path::Path};

pub fn validate_default_path(file_name: &str) -> Result<(), std::io::Error> {
    let home_dir = env::var(HOME).expect("should been read `HOME` value");
    let path = format!("{home_dir}/{DEFAULT_PATH}");
    let file_path = format!("{home_dir}/{DEFAULT_PATH}/{file_name}");
    if !Path::new(&path).is_dir() {
        fs::create_dir(&path)?;
    }
    File::create(file_path)?;
    Ok(())
}
