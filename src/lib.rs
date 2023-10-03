mod constants;
mod helpers;
use helpers::MemorizeHelper;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs,
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct MemorizeBox {
    pub alias: String,
    pub command: String,
}

pub trait MemorizeUtils {
    fn validate_default_path(self) -> Result<(), std::io::Error>;
    fn update(self, content: &Value) -> Result<String, String>;
    fn collect(self);
}

impl MemorizeUtils for MemorizeBox {
    fn validate_default_path(self) -> Result<(), std::io::Error> {
        let path = MemorizeHelper::use_default_path();
        let file_path = MemorizeHelper::use_default_file();
        if !Path::new(&path).is_dir() {
            fs::create_dir(&path)?;
        }
        File::create(file_path)?;
        Ok(())
    }

    fn update(self, content: &Value) -> Result<String, String> {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        match handler.write_into_json(content) {
            Ok(_) => Ok(String::from("Modified")),
            Err(e) => panic!("{e}"),
        }
    }

    fn collect(self) {
        let file_path = MemorizeHelper::use_default_file();
        let handler = JSONHandler::new(&file_path);
        let json_content = handler.read_json_from_file();
        match json_content {
            Ok(content) => {
                handler.print_pretty(&content);
            }
            Err(e) => panic!("{e}"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct JSONHandler<'j> {
    file_path: &'j str,
}

impl<'j> JSONHandler<'j> {
    pub fn new(file_path: &'j str) -> Self {
        JSONHandler { file_path }
    }

    pub fn read_json_from_file(self) -> Result<Value, std::io::Error> {
        let file = File::open(self.file_path)?;
        let reader = std::io::BufReader::new(file);
        let content = serde_json::from_reader(reader)?;
        Ok(content)
    }

    pub fn write_into_json(self, content: &Value) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.file_path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, content)?;
        Ok(())
    }

    pub fn print_pretty(self, content: &Value) {
        if let Ok(pretty_content) = serde_json::to_string_pretty(&content) {
            println!("{pretty_content}");
        }
    }
}
