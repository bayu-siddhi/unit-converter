use anyhow::{Ok, Result, bail};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct HistoryRecord {
    pub value: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct History {
    pub list: Vec<HistoryRecord>,
}

impl History {
    const FILE_PATH: &'static str = "data/history.json";

    pub fn load() -> Result<Self> {
        let file_path: &Path = Path::new(Self::FILE_PATH);
        if !file_path.exists() {
            return Ok(Self::default());
        }

        let str_json: String = fs::read_to_string(file_path)?;
        let list: History = serde_json::from_str(&str_json)?;
        return Ok(list);
    }

    pub fn save(&self) -> Result<()> {
        let file_path: &Path = Path::new(Self::FILE_PATH);
        let data: String = serde_json::to_string_pretty(&self)?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(file_path, data)?;
        return Ok(());
    }

    pub fn add(&mut self, record: String) {
        self.list.push(HistoryRecord { value: record });
    }

    pub fn print(&self) -> Result<()> {
        if self.list.is_empty() {
            bail!("No conversion history found.");
        }

        println!("Conversion History:");
        for (i, record) in self.list.iter().enumerate() {
            println!("{}. {}", i + 1, record.value);
        }

        return Ok(());
    }
}
