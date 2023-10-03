mod constants;
use constants::{DEFAULT_PATH, HOME};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env, fs,
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct MemorizeBox {
    pub alias: String,
    pub command: String,
}

pub trait MemorizeUtils {
    fn validate_default_path(self, file_name: &str) -> Result<(), std::io::Error>;
    fn update(self, content: &str) -> Result<String, String>;
}

impl MemorizeUtils for MemorizeBox {
    fn validate_default_path(self, file_name: &str) -> Result<(), std::io::Error> {
        let home_dir = env::var(HOME).expect("should been read `HOME` value");
        let path = format!("{home_dir}/{DEFAULT_PATH}");
        let file_path = format!("{home_dir}/{DEFAULT_PATH}/{file_name}");
        if !Path::new(&path).is_dir() {
            fs::create_dir(&path)?;
        }
        File::create(file_path)?;
        Ok(())
    }

    fn update(self, content: &str) -> Result<String, String> {
        Ok(String::from("Modified"))
    }
}

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
}
