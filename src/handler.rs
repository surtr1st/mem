use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::MemorizeBox;

#[derive(Clone, Copy)]
pub struct JSONHandler<'j> {
    file_path: &'j str,
}

impl<'j> JSONHandler<'j> {
    pub fn new(file_path: &'j str) -> Self {
        JSONHandler { file_path }
    }

    pub fn read_json_from_file(self) -> Result<Vec<MemorizeBox>, std::io::Error> {
        let file = File::open(self.file_path)?;
        let reader = std::io::BufReader::new(file);
        let content = serde_json::from_reader(reader)?;
        Ok(content)
    }

    pub fn write_into_json(self, content: &MemorizeBox) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file_path)?;

        let mut existing_content = String::new();
        file.read_to_string(&mut existing_content)?;

        let mut memorize_boxes: Vec<MemorizeBox> = serde_json::from_str(&existing_content)?;

        memorize_boxes.push(content.clone());

        let updated_content = serde_json::to_string_pretty(&memorize_boxes)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(updated_content.as_bytes())?;
        Ok(())
    }
}
