use egui::{Context, Ui, ScrollArea, Color32, RichText, Vec2};
use std::collections::HashMap;
use crate::audio::AudioCategory;
use crate::ui::editor::ui_components::AudioFile;

/// Audio panel module for managing and playing audio
pub struct AudioPanel {
    /// Show audio panel
    pub show_audio_panel: bool,
    /// Audio files
    pub audio_files: Vec<AudioFile>,
    /// Master volume
    pub master_volume: f32,
    /// Category volumes
    pub category_volumes: HashMap<AudioCategory, f32>,
    /// Currently playing audio
    pub playing_audio: HashMap<String, bool>,
}

impl AudioPanel {
    /// Create a new audio panel
    pub fn new() -> Self {
        let mut category_volumes = HashMap::new();
        category_volumes.insert(AudioCategory::Music, 0.8);
        category_volumes.insert(AudioCategory::SoundEffect, 0.7);
        category_volumes.insert(AudioCategory::Ambient, 0.6);
        category_volumes.insert(AudioCategory::Voice, 1.0);
        
        Self {
            show_audio_panel: false,
            audio_files: vec![
                AudioFile {
                    name: "Background Music".to_string(),
                    path: "/Audio/music.mp3".to_string(),
                    category: AudioCategory::Music,
                    duration: Some(120.0),
                },
                AudioFile {
                    name: "Explosion".to_string(),
                    path: "/Audio/explosion.wav".to_string(),
                    category: AudioCategory::SoundEffect,
                    duration: Some(2.5),
                },
                AudioFile {
                    name: "Wind".to_string(),
                    path: "/Audio/wind.ogg".to_string(),
                    category: AudioCategory::Ambient,
                    duration: Some(30.0),
                },
            ],
            master_volume: 0.7,
            category_volumes,
            playing_audio: HashMap::new(),
        }
    }
    
    /// Render the audio panel
    pub fn render(&mut self, ctx: &Context, log_info: &mut dyn FnMut(&str)) {
        if self.show_audio_panel {
            egui::Window::new("Audio Mixer")
                .resizable(true)
                .min_width(400.0)
                .min_height(300.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Audio Mixer");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                self.show_audio_panel = false;
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Master Volume:");
                        ui.add(egui::Slider::new(&mut self.master_volume, 0.0..=1.0));
                    });
                    
                    ui.separator();
                    ui.heading("Categories");
                    
                    for (category, volume) in &mut self.category_volumes {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:?}:", category));
                            ui.add(egui::Slider::new(volume, 0.0..=1.0));
                        });
                    }
                    
                    ui.separator();
                    ui.heading("Audio Files");
                    
                    ScrollArea::vertical().show(ui, |ui| {
                        // Create a copy of audio files to avoid borrowing issues
                        let audio_files_copy: Vec<_> = self.audio_files
                            .iter()
                            .map(|audio| (audio.name.clone(), audio.path.clone(), audio.category))
                            .collect();
                        
                        for (i, (name, path, category)) in audio_files_copy.iter().enumerate() {
                            ui.push_id(i, |ui| {
                                ui.horizontal(|ui| {
                                    let is_playing = self.playing_audio.get(name).copied().unwrap_or(false);
                                    
                                    if ui.button(if is_playing { "⏹" } else { "▶" }).clicked() {
                                        let new_state = !is_playing;
                                        self.playing_audio.insert(name.clone(), new_state);
                                        
                                        if new_state {
                                            log_info(&format!("Playing audio: {}", name));
                                        } else {
                                            log_info(&format!("Stopped audio: {}", name));
                                        }
                                    }
                                    
                                    ui.label(RichText::new(name).strong());
                                    ui.label(format!("({:?})", category));
                                    
                                    if is_playing {
                                        // Draw mock waveform
                                        let rect = ui.allocate_space(Vec2::new(120.0, 20.0)).1;
                                        self.draw_mock_waveform(ui, rect, true);
                                    }
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label(format!("Path: {}", path));
                                });
                                
                                ui.separator();
                            });
                        }
                    });
                });
        }
    }
    
    /// Render project audio tab
    pub fn render_project_audio(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        ui.horizontal(|ui| {
            ui.heading("Audio");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Show Mixer").clicked() {
                    self.show_audio_panel = true;
                }
            });
        });
        
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Master Volume:");
            ui.add(egui::Slider::new(&mut self.master_volume, 0.0..=1.0));
        });
        
        ui.separator();
        
        ScrollArea::vertical().show(ui, |ui| {
            // Create a copy of audio files to avoid borrowing issues
            let audio_files_copy: Vec<_> = self.audio_files
                .iter()
                .map(|audio| (
                    audio.name.clone(), 
                    audio.path.clone(), 
                    audio.category,
                    audio.duration
                ))
                .collect();
            
            for (i, (audio_name, audio_path, category, duration)) in audio_files_copy.iter().enumerate() {
                ui.push_id(i, |ui| {
                    ui.horizontal(|ui| {
                        let is_playing = self.playing_audio.get(audio_name).copied().unwrap_or(false);
                        
                        if ui.button(if is_playing { "⏹" } else { "▶" }).clicked() {
                            let new_state = !is_playing;
                            self.playing_audio.insert(audio_name.clone(), new_state);
                            
                            if new_state {
                                log_info(&format!("Playing audio: {}", audio_name));
                            } else {
                                log_info(&format!("Stopped audio: {}", audio_name));
                            }
                        }
                        
                        ui.vertical(|ui| {
                            ui.label(RichText::new(audio_name).strong());
                            ui.label(format!("Category: {:?}", category));
                            if let Some(dur) = duration {
                                ui.label(format!("Duration: {:.1}s", dur));
                            }
                            ui.label(format!("Path: {}", audio_path));
                        });
                        
                        if is_playing {
                            // Draw mock waveform
                            let rect = ui.allocate_space(Vec2::new(120.0, 20.0)).1;
                            self.draw_mock_waveform(ui, rect, true);
                        }
                    });
                    
                    ui.separator();
                });
            }
        });
    }
    
    /// Draw a mock waveform
    pub fn draw_mock_waveform(&self, ui: &mut Ui, rect: egui::Rect, is_playing: bool) {
        let painter = ui.painter();
        
        let color = if is_playing {
            Color32::from_rgb(100, 200, 100)
        } else {
            Color32::from_rgb(150, 150, 150)
        };
        
        // Draw a simple waveform representation
        for i in 0..20 {
            let x = rect.left() + (i as f32 * rect.width() / 20.0);
            let height = (i as f32).sin().abs() * rect.height() * 0.8;
            let y_center = rect.center().y;
            
            painter.line_segment(
                [egui::pos2(x, y_center - height / 2.0), egui::pos2(x, y_center + height / 2.0)],
                egui::Stroke::new(2.0, color),
            );
        }
    }
} 