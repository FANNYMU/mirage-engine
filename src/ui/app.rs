use eframe::{egui, CreationContext};
use winit::window::Window;
use std::sync::Arc;
use std::time::Instant;
use log::info;

/// Main application for the Mirage Engine
pub struct MirageApp {
    window: Option<Arc<Window>>,
    last_update: Instant,
}

impl MirageApp {
    /// Create a new app with eframe context
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        info!("Creating MirageApp with eframe");
        Self {
            window: None,
            last_update: Instant::now(),
        }
    }

    /// Create a new app without UI for headless mode
    pub fn new_headless() -> Self {
        info!("Creating headless MirageApp");
        Self {
            window: None,
            last_update: Instant::now(),
        }
    }

    /// Run the app with a window manager
    pub fn run_with_window(self) {
        info!("Running with window manager");
        // TODO: Implement window manager mode
    }

    /// Set the window
    pub fn set_window(&mut self, window: Arc<Window>) {
        self.window = Some(window);
    }
}

impl eframe::App for MirageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Main UI panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mirage Engine");
            ui.label(format!("FPS: {:.1}", 1.0 / delta_time));
            
            // Add more UI elements here
        });

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
} 