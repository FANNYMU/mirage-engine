mod core;
mod scene;
mod ui;
mod rendering;
mod utils;
mod ecs;
mod audio;

use eframe::{NativeOptions, run_native};
use ui::{MirageApp, EditorUI};
use log::LevelFilter;
use std::env;
use std::sync::Arc;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    let use_winit = args.len() > 1 && args[1] == "--winit";
    
    if use_winit {
        // Run with window manager (winit)
        run_with_winit().await?;
        Ok(())
    } else {
        // Configure application
        let options = NativeOptions {
            initial_window_size: Some(egui::vec2(1280.0, 720.0)),
            resizable: true,
            vsync: true,
            centered: true,
            ..Default::default()
        };
        
        // Run application with editor UI
        run_native(
            "Mirage Engine Editor",
            options,
            Box::new(|cc| Box::new(EditorApp::new(cc)))
        )?;
        
        Ok(())
    }
}

/// The main editor application
struct EditorApp {
    editor_ui: EditorUI,
    last_update_time: std::time::Instant,
}

impl EditorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set default egui style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        cc.egui_ctx.set_style(style);
        
        Self {
            editor_ui: EditorUI::new(),
            last_update_time: std::time::Instant::now(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Calculate delta time
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.last_update_time).as_secs_f32();
        self.last_update_time = now;
        
        // Update editor UI
        self.editor_ui.update(ctx, delta_time);
        
        // Maintain window size
        if let Some(original_size) = self.editor_ui.get_original_size() {
            let current_size = frame.info().window_info.size;
            if (current_size.x - original_size[0]).abs() > 1.0 || (current_size.y - original_size[1]).abs() > 1.0 {
                frame.set_window_size(egui::vec2(original_size[0], original_size[1]));
            }
        }
        
        // Request continuous repainting
        ctx.request_repaint();
    }
}

async fn run_with_winit() -> Result<(), Box<dyn std::error::Error>> {
    // Create event loop
    let event_loop = EventLoop::new();
    
    // Create window
    let window = WindowBuilder::new()
        .with_title("Mirage Engine (winit)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)?;
    
    // Create renderer
    let mut renderer = rendering::Renderer::new(&window).await?;
    
    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                // Resize renderer
                renderer.resize(size.width, size.height);
            },
            Event::MainEventsCleared => {
                // Redraw the window
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Render frame
                if let Err(e) = renderer.render_frame() {
                    log::error!("Failed to render frame: {}", e);
                }
            },
            _ => (),
        }
    });
}
