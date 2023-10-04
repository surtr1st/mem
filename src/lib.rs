mod constants;
mod handler;
mod helpers;
use anyhow::{Context, Error, Result};
use handler::JSONHandler;
use helpers::MemorizeHelper;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, fs::File, io::Write, path::Path, process::Command, rc::Rc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemorizeBox {
    pub alias: String,
    pub command: String,
}

pub struct MemorizeUtils;

impl MemorizeUtils {
    fn use_json_handler() -> Rc<JSONHandler> {
        Rc::new(JSONHandler::new(MemorizeHelper::use_default_file()))
    }

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

    pub fn add(content: &MemorizeBox) -> Result<()> {
        let handler = Self::use_json_handler();
        if !handler.is_unique(&content.alias) {
            return Err(Error::msg("Alias existed! Please set another alias!"));
        }
        handler.write_into_json(content)?;
        Ok(())
    }

    pub fn update_by_alias(target: &str, new_value: &str) -> Result<()> {
        let handler = Self::use_json_handler();
        handler.modify_by_alias(target, new_value)?;
        Ok(())
    }

    pub fn update_command_by_alias(target: &str, new_value: &str) -> Result<()> {
        let handler = Self::use_json_handler();
        handler.modify_command_by_alias(target, new_value)?;
        Ok(())
    }

    pub fn delete_command_by_alias(target: &str) -> Result<()> {
        let handler = Self::use_json_handler();
        handler.delete_property_by_alias(target)?;
        Ok(())
    }

    pub fn collect() -> Result<()> {
        let handler = Self::use_json_handler();
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

    pub fn invoke_command(alias: &str, value: &Option<String>) -> Result<()> {
        let handler = Self::use_json_handler();
        let list = handler.read_json_from_file()?;
        if let Some(index) = list.iter().position(|item| item.alias == alias) {
            if let Some(memo) = list.get(index) {
                let separated_parts = memo.command.split(" ").collect::<Vec<_>>();
                let mut command = Command::new(&separated_parts[0]);
                let excluded_first_element: Vec<_> = separated_parts.iter().skip(1).collect();
                for arg in excluded_first_element {
                    command.arg(arg);
                }
                if let Some(v) = value {
                    command.arg(v);
                }
                command
                    .spawn()
                    .with_context(|| format!("Failed to execute command: `{}`", &memo.command))?;
            }
        }
        Ok(())
    }
}
