use std::path::PathBuf;
use anyhow::{Result, anyhow};

use super::audio_engine::AudioCategory;

/// Structure representing an audio source
#[derive(Debug)]
pub struct AudioSource {
    /// Unique ID for the audio source
    pub id: String,
    
    /// Path to the audio file
    pub path: PathBuf,
    
    /// Audio category (music, sound effects, etc.)
    pub category: AudioCategory,
}

impl AudioSource {
    /// Create a new AudioSource instance
    pub fn new(id: String, path: PathBuf, category: AudioCategory) -> Result<Self> {
        // Validate that the file exists
        if !path.exists() {
            return Err(anyhow!("Audio file '{}' not found", path.display()));
        }
        
        // Validate file extension
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| anyhow!("Audio file doesn't have a valid extension"))?;
        
        // Validate supported formats
        match extension.to_lowercase().as_str() {
            "mp3" | "wav" | "flac" | "ogg" => {},
            _ => return Err(anyhow!("File format '{}' not supported", extension)),
        }
        
        Ok(Self {
            id,
            path,
            category,
        })
    }
} 