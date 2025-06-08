use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use glam::Vec3;
use anyhow::{Result, anyhow};

use super::audio_source::AudioSource;
use super::audio_listener::AudioListener;

/// Kategori audio untuk memisahkan antara musik dan efek suara
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioCategory {
    Music,
    SoundEffect,
    Ambient,
    Voice,
}

/// Status pemutaran audio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

/// Struktur utama AudioEngine untuk mengelola semua audio dalam game
pub struct AudioEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sources: HashMap<String, Arc<AudioSource>>,
    active_sinks: HashMap<String, Sink>,
    master_volume: f32,
    category_volumes: HashMap<AudioCategory, f32>,
    listener: AudioListener,
}

impl AudioEngine {
    /// Membuat instance baru AudioEngine
    pub fn new() -> Result<Self> {
        // Inisialisasi output stream dari rodio
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| anyhow!("Gagal membuat audio stream: {}", e))?;
        
        // Inisialisasi kategori volume dengan nilai default
        let mut category_volumes = HashMap::new();
        category_volumes.insert(AudioCategory::Music, 0.8);
        category_volumes.insert(AudioCategory::SoundEffect, 1.0);
        category_volumes.insert(AudioCategory::Ambient, 0.6);
        category_volumes.insert(AudioCategory::Voice, 1.0);
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sources: HashMap::new(),
            active_sinks: HashMap::new(),
            master_volume: 1.0,
            category_volumes,
            listener: AudioListener::new(Vec3::ZERO, Vec3::Z),
        })
    }
    
    /// Memuat file audio dari path dan menyimpannya dengan ID
    pub fn load_audio(&mut self, id: &str, path: impl AsRef<Path>, category: AudioCategory) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let source = AudioSource::new(id.to_string(), path, category)?;
        self.sources.insert(id.to_string(), Arc::new(source));
        Ok(())
    }
    
    /// Memutar audio dengan ID tertentu
    pub fn play(&mut self, id: &str, looping: bool) -> Result<()> {
        let source = self.sources.get(id)
            .ok_or_else(|| anyhow!("Audio dengan ID '{}' tidak ditemukan", id))?
            .clone();
        
        // Buat sink baru untuk audio ini
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| anyhow!("Gagal membuat sink audio: {}", e))?;
        
        // Buka file audio
        let file = File::open(&source.path)
            .map_err(|e| anyhow!("Gagal membuka file audio '{}': {}", source.path.display(), e))?;
        
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)
            .map_err(|e| anyhow!("Gagal decode audio '{}': {}", source.path.display(), e))?;
        
        // Terapkan volume kategori
        let category_volume = self.category_volumes.get(&source.category).unwrap_or(&1.0);
        sink.set_volume(*category_volume * self.master_volume);
        
        // Terapkan looping jika diperlukan
        if looping {
            sink.append(decoder.repeat_infinite());
        } else {
            sink.append(decoder);
        }
        
        // Mainkan audio
        sink.play();
        
        // Simpan sink aktif
        self.active_sinks.insert(id.to_string(), sink);
        
        Ok(())
    }
    
    /// Memutar audio dengan posisi 3D (untuk audio spasial)
    pub fn play_at_position(&mut self, id: &str, position: Vec3, looping: bool) -> Result<()> {
        let source = self.sources.get(id)
            .ok_or_else(|| anyhow!("Audio dengan ID '{}' tidak ditemukan", id))?
            .clone();
        
        // Buat sink baru untuk audio ini
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| anyhow!("Gagal membuat sink audio: {}", e))?;
        
        // Buka file audio
        let file = File::open(&source.path)
            .map_err(|e| anyhow!("Gagal membuka file audio '{}': {}", source.path.display(), e))?;
        
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)
            .map_err(|e| anyhow!("Gagal decode audio '{}': {}", source.path.display(), e))?;
        
        // Hitung volume berdasarkan jarak dari listener
        let distance = position.distance(self.listener.position);
        let distance_factor = (1.0 / (1.0 + distance * 0.1)).min(1.0);
        
        // Terapkan volume kategori dan faktor jarak
        let category_volume = self.category_volumes.get(&source.category).unwrap_or(&1.0);
        sink.set_volume(*category_volume * self.master_volume * distance_factor);
        
        // Terapkan looping jika diperlukan
        if looping {
            sink.append(decoder.repeat_infinite());
        } else {
            sink.append(decoder);
        }
        
        // Mainkan audio
        sink.play();
        
        // Simpan sink aktif
        self.active_sinks.insert(id.to_string(), sink);
        
        Ok(())
    }
    
    /// Menghentikan pemutaran audio dengan ID tertentu
    pub fn stop(&mut self, id: &str) -> Result<()> {
        if let Some(sink) = self.active_sinks.remove(id) {
            sink.stop();
            Ok(())
        } else {
            Err(anyhow!("Tidak ada audio aktif dengan ID '{}'", id))
        }
    }
    
    /// Menjeda pemutaran audio dengan ID tertentu
    pub fn pause(&mut self, id: &str) -> Result<()> {
        if let Some(sink) = self.active_sinks.get(id) {
            sink.pause();
            Ok(())
        } else {
            Err(anyhow!("Tidak ada audio aktif dengan ID '{}'", id))
        }
    }
    
    /// Melanjutkan pemutaran audio yang dijeda
    pub fn resume(&mut self, id: &str) -> Result<()> {
        if let Some(sink) = self.active_sinks.get(id) {
            sink.play();
            Ok(())
        } else {
            Err(anyhow!("Tidak ada audio aktif dengan ID '{}'", id))
        }
    }
    
    /// Mengatur volume master (mempengaruhi semua audio)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.update_all_volumes();
    }
    
    /// Mengatur volume untuk kategori audio tertentu
    pub fn set_category_volume(&mut self, category: AudioCategory, volume: f32) {
        self.category_volumes.insert(category, volume.clamp(0.0, 1.0));
        self.update_all_volumes();
    }
    
    /// Memperbarui volume untuk semua sink aktif
    fn update_all_volumes(&mut self) {
        for (id, sink) in &self.active_sinks {
            if let Some(source) = self.sources.get(id) {
                let category_volume = self.category_volumes.get(&source.category).unwrap_or(&1.0);
                sink.set_volume(*category_volume * self.master_volume);
            }
        }
    }
    
    /// Memperbarui posisi listener untuk audio 3D
    pub fn update_listener(&mut self, position: Vec3, forward: Vec3) {
        self.listener.position = position;
        self.listener.forward = forward;
    }
    
    /// Mendapatkan status pemutaran audio
    pub fn get_playback_status(&self, id: &str) -> Option<PlaybackStatus> {
        if let Some(sink) = self.active_sinks.get(id) {
            if sink.is_paused() {
                Some(PlaybackStatus::Paused)
            } else {
                Some(PlaybackStatus::Playing)
            }
        } else {
            if self.sources.contains_key(id) {
                Some(PlaybackStatus::Stopped)
            } else {
                None
            }
        }
    }
    
    /// Menghentikan semua audio yang sedang diputar
    pub fn stop_all(&mut self) {
        for (_, sink) in self.active_sinks.drain() {
            sink.stop();
        }
    }
} 