use egui::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneState {
    Active,
    Paused,
    Inactive,
    Destroyed,
}

pub trait Scene {
    fn name(&self) -> &str;
    
    fn on_load(&mut self);
    
    fn on_activate(&mut self);
    
    fn on_pause(&mut self);
    
    fn on_resume(&mut self);
    
    fn on_deactivate(&mut self);
    
    fn on_unload(&mut self);
    
    fn update(&mut self, delta_time: f32);
    
    fn render(&mut self, ctx: &Context);
    
    fn state(&self) -> SceneState;
    
    fn set_state(&mut self, state: SceneState);
} 