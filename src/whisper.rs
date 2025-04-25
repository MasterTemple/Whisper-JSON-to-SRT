use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnalignedWhisperXFile {
    pub segments: Vec<UnalignedWhisperXSegment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnalignedWhisperXSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

impl UnalignedWhisperXFile {
    pub fn read_from(path: impl AsRef<Path>) -> crate::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::read_from_str(&contents)
    }
    pub fn read_from_str(contents: &str) -> crate::Result<Self> {
        let file: UnalignedWhisperXFile = serde_json::from_str(contents)?;
        Ok(file)
    }
}
