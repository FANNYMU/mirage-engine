use eframe::{App, CreationContext, Frame};
use egui::{Context, CentralPanel, TopBottomPanel, Ui};
use crate::core::GameLoop;

pub struct MirageApp {
    game_loop: GameLoop,
    show_fps: bool,
}

impl MirageApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            game_loop: GameLoop::new(),
            show_fps: true,
        }
    }

    fn top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Mirage Engine");
            ui.separator();
            
            if self.show_fps {
                let fps = self.game_loop.fps();
                ui.label(format!("FPS: {:.1}", fps));
            }
            
            ui.checkbox(&mut self.show_fps, "Show FPS");
        });
    }
    
    fn main_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |_ui| {
        });
    }
}

impl App for MirageApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.top_panel(ui);
        });
        
        self.main_panel(ctx);
        
        self.game_loop.update(ctx, frame);
    }
} 