use eframe::{App, Frame};
use egui::Context;

use crate::core::DeltaTime;
use crate::scene::SceneManager;

pub struct GameLoop {
    delta_time: DeltaTime,
    scene_manager: SceneManager,
    running: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            delta_time: DeltaTime::new(),
            scene_manager: SceneManager::new(),
            running: true,
        }
    }

    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.delta_seconds()
    }

    pub fn fps(&self) -> f32 {
        self.delta_time.fps()
    }
}

impl App for GameLoop {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let dt = self.delta_time.update();

        self.scene_manager.update(dt);

        self.scene_manager.render(ctx);

        ctx.request_repaint();
    }
} 