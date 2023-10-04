use crate::MemorizeBox;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone, Copy)]
pub struct JSONHandler<'j> {
    file_path: &'j str,
}

impl<'j> JSONHandler<'j> {
    pub fn new(file_path: &'j str) -> Self {
        JSONHandler { file_path }
    }

    pub fn read_json_from_file(self) -> Result<Vec<MemorizeBox>> {
        let file = File::open(self.file_path)
            .with_context(|| format!("could not open file `{}`", self.file_path))?;
        let reader = std::io::BufReader::new(file);
        let content = serde_json::from_reader(reader).with_context(|| "could not read content")?;
        Ok(content)
    }

    pub fn write_into_json(self, content: &MemorizeBox) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file_path)
            .with_context(|| format!("could not open file `{}`", self.file_path))?;

        let mut existing_content = String::new();
        file.read_to_string(&mut existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        let mut memorize_boxes: Vec<MemorizeBox> = serde_json::from_str(&existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        memorize_boxes.push(content.clone());

        let updated_content = serde_json::to_string_pretty(&memorize_boxes)
            .with_context(|| "could not prettify string")?;
        file.seek(SeekFrom::Start(0))
            .with_context(|| "could not seek from 0")?;
        file.set_len(updated_content.len() as u64)
            .with_context(|| "could not set len from updated content")?;
        file.write_all(updated_content.as_bytes())
            .with_context(|| "could not write all")?;

        Ok(())
    }

    pub fn modify_by_alias(self, alias: &str, new_value: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file_path)
            .with_context(|| format!("could not open file `{}`", self.file_path))?;

        let mut existing_content = String::new();
        file.read_to_string(&mut existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        let mut memorize_boxes: Vec<MemorizeBox> = serde_json::from_str(&existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        for item in &mut memorize_boxes {
            if item.alias == alias {
                item.alias = new_value.to_string();
            }
        }

        let updated_content = serde_json::to_string_pretty(&memorize_boxes)
            .with_context(|| "could not prettify string")?;
        file.seek(SeekFrom::Start(0))
            .with_context(|| "could not seek from 0")?;
        file.set_len(updated_content.len() as u64)
            .with_context(|| "could not set len from updated content")?;
        file.write_all(updated_content.as_bytes())
            .with_context(|| "could not write all")?;

        Ok(())
    }

    pub fn is_unique(self, key: &str) -> bool {
        let json = self.read_json_from_file();
        if let Ok(list) = json {
            for item in list {
                if item.alias == key {
                    return false;
                }
            }
        }
        true
    }
}
