mod constants;
mod handler;
mod helpers;
use anyhow::{Context, Error, Result};
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
    pub fn validate_default_path() -> Result<()> {
        let path = MemorizeHelper::use_default_path();
        let file_path = MemorizeHelper::use_default_file();

        if !Path::new(&path).is_dir() {
            let message = format!("could not create within the directory: {path}");
            fs::create_dir(&path).with_context(|| message)?;
        }

        if !Path::new(&file_path).exists() {
            let empty_array = json!([]);
            let message = format!("could not create within the directory: {file_path}");
            let mut created_file = File::create(file_path).with_context(|| message)?;
            created_file
                .write_all(empty_array.to_string().as_bytes())
                .with_context(|| "could not write all into file")?;
        }

        Ok(())
    }

    pub fn update(content: &MemorizeBox) -> Result<()> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        if !handler.is_unique(&content.alias) {
            return Err(Error::msg("Alias existed! Please set another alias!"));
        }
        handler.write_into_json(content)?;
        Ok(())
    }

    pub fn collect() -> Result<()> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        let list = handler
            .read_json_from_file()
            .with_context(|| "could not read conten from file")?;

        let header = MemorizeHelper::use_left_aligned(vec!["ALIAS", "COMMAND"]);
        println!("{}", header);
        list.iter().for_each(|item| {
            let body = format!("{:<20}\t{:<20}", item.alias, item.command);
            println!("{}", body);
        });
        Ok(())
    }
}
