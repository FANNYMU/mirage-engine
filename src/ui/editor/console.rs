use egui::{Context, Ui, ScrollArea};
use chrono::Local;
use crate::ui::editor::ui_components::{ConsoleLog, LogLevel, helpers};

/// Console panel for displaying logs
pub struct ConsolePanel {
    /// Whether to show console
    pub show_console: bool,
    /// Log messages for console
    pub console_logs: Vec<ConsoleLog>,
}

impl ConsolePanel {
    /// Create a new console panel
    pub fn new() -> Self {
        // Create sample console logs
        let console_logs = vec![
            ConsoleLog {
                timestamp: "12:34:56".to_string(),
                level: LogLevel::Info,
                message: "Editor initialized".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:57".to_string(),
                level: LogLevel::Info,
                message: "Scene loaded".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:58".to_string(),
                level: LogLevel::Warning,
                message: "Missing sprite reference".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:59".to_string(),
                level: LogLevel::Info,
                message: "Project assets loaded".to_string(),
            },
            ConsoleLog {
                timestamp: "12:35:00".to_string(),
                level: LogLevel::Error,
                message: "Failed to load audio file".to_string(),
            },
        ];
        
        Self {
            show_console: false,
            console_logs,
        }
    }
    
    /// Render console window
    pub fn render(&mut self, ctx: &Context) {
        if self.show_console {
            egui::Window::new("Console")
                .resizable(true)
                .min_width(400.0)
                .min_height(200.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Console");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Clear").clicked() {
                                self.console_logs.clear();
                            }
                            if ui.button("Close").clicked() {
                                self.show_console = false;
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                        for log in &self.console_logs {
                            ui.label(helpers::get_log_text(log));
                        }
                    });
                });
        }
    }
    
    /// Render project console tab
    pub fn render_project_console(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Console");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Show Console").clicked() {
                    self.show_console = true;
                }
                if ui.button("Clear").clicked() {
                    self.console_logs.clear();
                }
            });
        });
        
        ui.separator();
        
        ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for log in &self.console_logs {
                ui.label(helpers::get_log_text(log));
            }
        });
    }
    
    /// Log info message
    pub fn log_info(&mut self, message: &str) {
        self.add_log(LogLevel::Info, message);
    }
    
    /// Log warning message
    pub fn log_warning(&mut self, message: &str) {
        self.add_log(LogLevel::Warning, message);
    }
    
    /// Log error message
    pub fn log_error(&mut self, message: &str) {
        self.add_log(LogLevel::Error, message);
    }
    
    /// Add log message
    pub fn add_log(&mut self, level: LogLevel, message: &str) {
        let now = Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();
        
        self.console_logs.push(ConsoleLog {
            timestamp,
            level,
            message: message.to_string(),
        });
        
        // Limit log size
        if self.console_logs.len() > 100 {
            self.console_logs.remove(0);
        }
    }
} 