mod core;
mod scene;
mod ui;
mod rendering;
mod utils;

use eframe::{NativeOptions, run_native};
use ui::MirageApp;
use log::LevelFilter;

fn main() -> Result<(), eframe::Error> {
    // Inisialisasi logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    
    // Konfigurasi aplikasi
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        resizable: true,
        vsync: true,
        ..Default::default()
    };
    
    // Run aplikasi
    run_native(
        "Mirage Engine",
        options,
        Box::new(|cc| Box::new(MirageApp::new(cc)))
    )
}
