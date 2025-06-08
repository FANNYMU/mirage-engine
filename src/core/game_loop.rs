use eframe::{App, Frame};
use egui::Context;
use winit::event::Event as WinitEvent;

use crate::core::{DeltaTime, EventSystem, UpdateEvent, RenderEvent};
use crate::scene::SceneManager;
use crate::ui::WindowManager;

pub struct GameLoop {
    delta_time: DeltaTime,
    scene_manager: SceneManager,
    event_system: EventSystem,
    running: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            delta_time: DeltaTime::new(),
            scene_manager: SceneManager::new(),
            event_system: EventSystem::new(),
            running: true,
        }
    }

    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }
    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }
    
    pub fn event_system_mut(&mut self) -> &mut EventSystem {
        &mut self.event_system
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
    
    pub fn run_with_window(mut self, window_manager: WindowManager) {
        window_manager.run(move |_window, event| {
            match event {
                WinitEvent::MainEventsCleared => {
                    let dt = self.delta_time.update();
                    
                    self.event_system.publish(UpdateEvent { delta_time: dt });
                    
                    self.scene_manager.update(dt);
                    
                    self.event_system.publish(RenderEvent);
                }
                _ => {}
            }
        });
    }
}

impl App for GameLoop {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let dt = self.delta_time.update();
        
        self.event_system.publish(UpdateEvent { delta_time: dt });

        self.scene_manager.update(dt);
        
        self.event_system.publish(RenderEvent);

        self.scene_manager.render(ctx);

        ctx.request_repaint();
    }
} 