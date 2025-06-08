use std::path::PathBuf;
use anyhow::{Result, anyhow};

use super::audio_engine::AudioCategory;

/// Struktur yang merepresentasikan sumber audio
#[derive(Debug)]
pub struct AudioSource {
    /// ID unik untuk audio source
    pub id: String,
    
    /// Path ke file audio
    pub path: PathBuf,
    
    /// Kategori audio (musik, efek suara, dll)
    pub category: AudioCategory,
}

impl AudioSource {
    /// Membuat instance baru AudioSource
    pub fn new(id: String, path: PathBuf, category: AudioCategory) -> Result<Self> {
        // Validasi bahwa file ada
        if !path.exists() {
            return Err(anyhow!("File audio '{}' tidak ditemukan", path.display()));
        }
        
        // Validasi ekstensi file
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| anyhow!("File audio tidak memiliki ekstensi yang valid"))?;
        
        // Validasi format yang didukung
        match extension.to_lowercase().as_str() {
            "mp3" | "wav" | "flac" | "ogg" => {},
            _ => return Err(anyhow!("Format file '{}' tidak didukung", extension)),
        }
        
        Ok(Self {
            id,
            path,
            category,
        })
    }
} 