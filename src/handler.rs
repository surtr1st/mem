use crate::MemorizeBox;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
pub struct JSONHandler {
    file_path: String,
}

impl JSONHandler {
    pub fn new(file_path: String) -> Self {
        JSONHandler { file_path }
    }

    fn use_mutable_json(&self) -> Result<(File, Vec<MemorizeBox>)> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)
            .with_context(|| format!("could not open file `{}`", self.file_path))?;

        let mut existing_content = String::new();
        file.read_to_string(&mut existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        let memorize_boxes: Vec<MemorizeBox> = serde_json::from_str(&existing_content)
            .with_context(|| format!("could not read to string `{}`", &existing_content))?;

        let mut_content = (file, memorize_boxes);
        Ok(mut_content)
    }

    fn update(&self, file: &mut File, items: &mut Vec<MemorizeBox>) -> Result<()> {
        let updated_content =
            serde_json::to_string_pretty(&items).with_context(|| "could not prettify string")?;
        file.seek(SeekFrom::Start(0))
            .with_context(|| "could not seek from 0")?;
        file.set_len(updated_content.len() as u64)
            .with_context(|| "could not set len from updated content")?;
        file.write_all(updated_content.as_bytes())
            .with_context(|| "could not write all")?;
        Ok(())
    }

    pub fn read_json_from_file(&self) -> Result<Vec<MemorizeBox>> {
        let file = File::open(&self.file_path)
            .with_context(|| format!("could not open file `{}`", self.file_path))?;
        let reader = std::io::BufReader::new(file);
        let content = serde_json::from_reader(reader).with_context(|| "could not read content")?;
        Ok(content)
    }

    pub fn write_into_json(&self, content: &MemorizeBox) -> Result<()> {
        let (mut file, mut memorize_boxes) = self.use_mutable_json()?;
        memorize_boxes.push(content.clone());

        self.update(&mut file, &mut memorize_boxes)?;

        Ok(())
    }

    pub fn modify_by_alias(&self, alias: &str, new_value: &str) -> Result<()> {
        let (mut file, mut memorize_boxes) = self.use_mutable_json()?;

        for item in &mut memorize_boxes {
            if item.alias == alias {
                item.alias = new_value.to_string();
            }
        }

        self.update(&mut file, &mut memorize_boxes)?;

        Ok(())
    }

    pub fn modify_command_by_alias(&self, alias: &str, new_command: &str) -> Result<()> {
        let (mut file, mut memorize_boxes) = self.use_mutable_json()?;

        for item in &mut memorize_boxes {
            if item.alias == alias {
                item.command = new_command.to_string();
            }
        }

        self.update(&mut file, &mut memorize_boxes)?;

        Ok(())
    }

    pub fn delete_property_by_alias(&self, alias: &str) -> Result<()> {
        let (mut file, mut memorize_boxes) = self.use_mutable_json()?;

        let index_to_delete = memorize_boxes.iter().position(|item| item.alias == alias);

        if let Some(index) = index_to_delete {
            memorize_boxes.remove(index);
        }

        self.update(&mut file, &mut memorize_boxes)?;

        Ok(())
    }

    pub fn is_unique(&self, key: &str) -> bool {
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
