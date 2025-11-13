//! # Conversion History Module
//!
//! This module manages the persistence of conversion history. It provides functionality
//! to load, save, add, and display conversion records from a JSON file.

use anyhow::{Ok, Result, bail};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// A single entry in the conversion history.
#[derive(Serialize, Deserialize)]
pub struct HistoryRecord {
    pub value: String,
}

/// The collection of all conversion history records.
#[derive(Serialize, Deserialize, Default)]
pub struct History {
    pub list: Vec<HistoryRecord>,
}

impl History {
    const FILE_PATH: &'static str = "data/history.json";

    /// Loads conversion history from the JSON file.
    ///
    /// If the file does not exist, it returns a new, empty `History` instance.
    ///
    /// ## Returns
    ///
    /// An `anyhow::Result<Self>` which is the loaded `History` on success, or an
    /// error if the file cannot be read or parsed.
    ///
    pub fn load() -> Result<Self> {
        let file_path: &Path = Path::new(Self::FILE_PATH);
        if !file_path.exists() {
            return Ok(Self::default());
        }

        let str_json: String = fs::read_to_string(file_path)?;
        let list: History = serde_json::from_str(&str_json)?;
        return Ok(list);
    }

    /// Saves the current conversion history to the JSON file.
    ///
    /// It serializes the `History` struct into a pretty-printed JSON string and 
    /// writes it to the file. It also ensures the parent directory exists.
    ///
    /// ## Returns
    ///
    /// An `anyhow::Result<()>` indicating success or failure of the save operation.
    ///
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

    /// Adds a new conversion record to the history.
    ///
    /// ## Arguments
    ///
    /// * `record` - The `String` representation of the conversion result to add.
    ///
    pub fn add(&mut self, record: String) {
        self.list.push(HistoryRecord { value: record });
    }

    /// Prints the entire conversion history to the console.
    ///
    /// If the history is empty, it returns an error with a corresponding message.
    ///
    /// ## Returns
    ///
    /// An `anyhow::Result<()>` which is `Ok(())` on success, or an error if
    /// the history is empty.
    ///
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
