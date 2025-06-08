use egui::{Color32, RichText};
use crate::audio::AudioCategory;

/// Console log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Console log entry
#[derive(Debug, Clone)]
pub struct ConsoleLog {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

/// Project panel tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTab {
    Files,
    Console,
    Audio,
}

/// Project file type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectFileType {
    Folder,
    Scene,
    Script,
    Texture,
    Audio,
    Other,
}

/// Project file
#[derive(Debug, Clone)]
pub struct ProjectFile {
    pub name: String,
    pub file_type: ProjectFileType,
    pub path: String,
    pub children: Vec<ProjectFile>,
    pub expanded: bool,
    pub parent_path: Option<String>,
}

/// Component type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Transform,
    Camera,
    Light,
    SpriteRenderer,
    Rigidbody2D,
    BoxCollider2D,
    LuaScript,
    AudioSource,
    AudioListener,
}

/// Entity component
#[derive(Debug, Clone)]
pub struct EntityComponent {
    pub name: String,
    pub component_type: ComponentType,
    pub removable: bool,
}

/// Entity transform data
#[derive(Debug, Clone)]
pub struct EntityTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

/// Hierarchy item for representing entity hierarchies
#[derive(Debug, Clone)]
pub struct HierarchyItem {
    pub id: u32,
    pub name: String,
    pub children: Vec<HierarchyItem>,
}

/// Audio file information
#[derive(Debug, Clone)]
pub struct AudioFile {
    pub name: String,
    pub path: String,
    pub category: AudioCategory,
    pub duration: Option<f32>,
}

/// Scene view tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneViewTool {
    /// Select entities
    Select,
    /// Move entities
    Move,
    /// Rotate entities
    Rotate,
    /// Scale entities
    Scale,
}

/// Helper functions for UI components
pub mod helpers {
    use super::*;
    
    pub fn get_log_color(level: LogLevel) -> Color32 {
        match level {
            LogLevel::Info => Color32::from_rgb(220, 220, 220),
            LogLevel::Warning => Color32::from_rgb(255, 200, 0),
            LogLevel::Error => Color32::from_rgb(255, 0, 0),
        }
    }
    
    pub fn get_log_prefix(level: LogLevel) -> &'static str {
        match level {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
    
    pub fn get_log_text(log: &ConsoleLog) -> RichText {
        let color = get_log_color(log.level);
        let prefix = get_log_prefix(log.level);
        RichText::new(format!("[{}] [{}] {}", log.timestamp, prefix, log.message)).color(color)
    }
} 