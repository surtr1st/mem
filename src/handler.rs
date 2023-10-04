use serde_json::Value;
use std::fs::{File, OpenOptions};

#[derive(Clone, Copy)]
pub struct JSONHandler<'j> {
    file_path: &'j str,
}

impl<'j> JSONHandler<'j> {
    pub fn new(file_path: &'j str) -> Self {
        JSONHandler { file_path }
    }

    pub fn read_json_from_file(self) -> Result<Vec<Value>, std::io::Error> {
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
