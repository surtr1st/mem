mod constants;
mod handler;
mod helpers;
use handler::JSONHandler;
use helpers::MemorizeHelper;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, fs::File, io::Write, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct MemorizeBox {
    pub alias: String,
    pub command: String,
}

pub struct MemorizeUtils;

impl MemorizeUtils {
    pub fn validate_default_path() -> Result<(), String> {
        let path = MemorizeHelper::use_default_path();
        let file_path = MemorizeHelper::use_default_file();

        if !Path::new(&path).is_dir() {
            let message = format!("should create within the directory: {path}");
            fs::create_dir(&path).expect(&message);
        }

        if !Path::new(&file_path).exists() {
            let empty_array = json!([]);
            let message = format!("should write all into file");
            let mut created_file = File::create(file_path).expect("should create default file");
            created_file
                .write_all(empty_array.to_string().as_bytes())
                .expect(&message);
        }

        Ok(())
    }

    pub fn update(content: &MemorizeBox) -> Result<(), String> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        if !handler.is_unique(&content.alias) {
            return Err(String::from("Alias existed! Please set another alias!"));
        }
        match handler.write_into_json(content) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e.kind())),
        }
    }

    pub fn collect() -> Result<(), String> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        let list = handler
            .read_json_from_file()
            .expect("should read content from file");

        println!("ALIAS\tCOMMAND");
        list.iter()
            .for_each(|item| println!("{}\t{}", item.alias, item.command));
        Ok(())
    }
}
