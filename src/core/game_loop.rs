use eframe::{App, Frame};
use egui::Context;
use winit::event::Event as WinitEvent;

use crate::core::{DeltaTime, EventSystem, UpdateEvent, RenderEvent};
use crate::scene::SceneManager;
use crate::ui::WindowManager;

/// Struct utama untuk game loop
pub struct GameLoop {
    delta_time: DeltaTime,
    scene_manager: SceneManager,
    event_system: EventSystem,
    running: bool,
}

impl GameLoop {
    /// Membuat instance baru dari GameLoop
    pub fn new() -> Self {
        Self {
            delta_time: DeltaTime::new(),
            scene_manager: SceneManager::new(),
            event_system: EventSystem::new(),
            running: true,
        }
    }

    /// Mendapatkan referensi ke scene manager
    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    /// Mendapatkan referensi mutable ke scene manager
    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }
    
    /// Mendapatkan referensi ke event system
    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }
    
    /// Mendapatkan referensi mutable ke event system
    pub fn event_system_mut(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }

    /// Menghentikan game loop
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Mengecek apakah game loop masih berjalan
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Mendapatkan delta time dalam detik
    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.delta_seconds()
    }

    /// Mendapatkan FPS saat ini
    pub fn fps(&self) -> f32 {
        self.delta_time.fps()
    }
    
    /// Menjalankan game loop dengan window manager
    pub fn run_with_window(mut self, window_manager: WindowManager) {
        window_manager.run(move |_window, event| {
            match event {
                WinitEvent::MainEventsCleared => {
                    // Update delta time
                    let dt = self.delta_time.update();
                    
                    // Publish update event
                    self.event_system.publish(UpdateEvent { delta_time: dt });
                    
                    // Update scene aktif
                    self.scene_manager.update(dt);
                    
                    // Publish render event
                    self.event_system.publish(RenderEvent);
                }
                _ => {}
            }
        });
    }
}

impl App for GameLoop {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Update delta time
        let dt = self.delta_time.update();
        
        // Publish update event
        self.event_system.publish(UpdateEvent { delta_time: dt });

        // Update scene aktif
        self.scene_manager.update(dt);
        
        // Publish render event
        self.event_system.publish(RenderEvent);

        // Render scene aktif
        self.scene_manager.render(ctx);

        // Meminta repaint untuk frame berikutnya
        ctx.request_repaint();
    }
} 