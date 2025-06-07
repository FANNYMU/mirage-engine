use std::collections::HashMap;
use egui::Context;
use crate::scene::{Scene, SceneState};

pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    active_scene: Option<String>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            active_scene: None,
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        let name = scene.name().to_string();
        self.scenes.insert(name, scene);
    }

    pub fn activate_scene(&mut self, name: &str) -> bool {
        let previous_active = self.active_scene.clone();
        
        if self.scenes.contains_key(name) {
            if let Some(active_name) = previous_active {
                if active_name != name {
                    if let Some(active_scene) = self.scenes.get_mut(&active_name) {
                        active_scene.on_deactivate();
                        active_scene.set_state(SceneState::Inactive);
                    }
                }
            }

            if let Some(scene) = self.scenes.get_mut(name) {
                scene.on_activate();
                scene.set_state(SceneState::Active);
                self.active_scene = Some(name.to_string());
                return true;
            }
        }
        
        false
    }

    pub fn pause_active_scene(&mut self) -> bool {
        if let Some(name) = &self.active_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                scene.on_pause();
                scene.set_state(SceneState::Paused);
                return true;
            }
        }
        false
    }

    pub fn resume_active_scene(&mut self) -> bool {
        if let Some(name) = &self.active_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                if scene.state() == SceneState::Paused {
                    scene.on_resume();
                    scene.set_state(SceneState::Active);
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_scene(&mut self, name: &str) -> bool {
        if self.active_scene.as_deref() == Some(name) {
            self.active_scene = None;
        }
        
        if let Some(mut scene) = self.scenes.remove(name) {
            scene.on_unload();
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(name) = &self.active_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                if scene.state() == SceneState::Active {
                    scene.update(delta_time);
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        if let Some(name) = &self.active_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                if scene.state() == SceneState::Active || scene.state() == SceneState::Paused {
                    scene.render(ctx);
                }
            }
        }
    }

    pub fn active_scene(&self) -> Option<&dyn Scene> {
        self.active_scene
            .as_ref()
            .and_then(|name| self.scenes.get(name))
            .map(|scene| scene.as_ref())
    }

    pub fn active_scene_mut(&mut self) -> Option<&mut Box<dyn Scene>> {
        if let Some(name) = &self.active_scene {
            self.scenes.get_mut(name)
        } else {
            None
        }
    }
} 