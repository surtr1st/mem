mod constants;
mod handler;
mod helpers;
use handler::JSONHandler;
use helpers::MemorizeHelper;
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct MemorizeBox {
    pub alias: String,
    pub command: String,
}

pub trait MemorizeUtils {
    fn validate_default_path() -> Result<(), std::io::Error>;
    fn update(content: &MemorizeBox) -> Result<String, String>;
    fn collect() -> Result<(), Box<dyn std::error::Error>>;
}

impl MemorizeUtils for MemorizeBox {
    fn validate_default_path() -> Result<(), std::io::Error> {
        let path = MemorizeHelper::use_default_path();
        let file_path = MemorizeHelper::use_default_file();
        if !Path::new(&path).is_dir() {
            fs::create_dir(&path)?;
        }
        File::create(file_path)?;
        Ok(())
    }

    fn update(content: &MemorizeBox) -> Result<String, String> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        match handler.write_into_json(content) {
            Ok(_) => Ok(String::from("Modified")),
            Err(e) => panic!("{e}"),
        }
    }

    fn collect() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        let list = handler.read_json_from_file()?;

        println!("ALIAS\tCOMMAND");
        list.iter()
            .for_each(|item| println!("{}\t{}", item.alias, item.command));
        Ok(())
    }
}
