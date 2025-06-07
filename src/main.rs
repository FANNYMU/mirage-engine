mod core;
mod scene;
mod ui;
mod rendering;
mod utils;
mod ecs;

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
}

impl EditorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            editor_ui: EditorUI::new(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update editor UI
        self.editor_ui.update(ctx, 1.0 / 60.0); // Fixed delta time for now
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
