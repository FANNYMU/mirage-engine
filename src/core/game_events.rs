use crate::core::Event;
use winit::event::{VirtualKeyCode, MouseButton};

/// Event untuk window resize
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

impl Event for WindowResizeEvent {
    fn name(&self) -> &'static str {
        "WindowResizeEvent"
    }
}

/// Event untuk key press
pub struct KeyPressEvent {
    pub key: VirtualKeyCode,
}

impl Event for KeyPressEvent {
    fn name(&self) -> &'static str {
        "KeyPressEvent"
    }
}

/// Event untuk key release
pub struct KeyReleaseEvent {
    pub key: VirtualKeyCode,
}

impl Event for KeyReleaseEvent {
    fn name(&self) -> &'static str {
        "KeyReleaseEvent"
    }
}

/// Event untuk mouse button press
pub struct MousePressEvent {
    pub button: MouseButton,
    pub x: f64,
    pub y: f64,
}

impl Event for MousePressEvent {
    fn name(&self) -> &'static str {
        "MousePressEvent"
    }
}

/// Event untuk mouse button release
pub struct MouseReleaseEvent {
    pub button: MouseButton,
    pub x: f64,
    pub y: f64,
}

impl Event for MouseReleaseEvent {
    fn name(&self) -> &'static str {
        "MouseReleaseEvent"
    }
}

/// Event untuk mouse move
pub struct MouseMoveEvent {
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
}

impl Event for MouseMoveEvent {
    fn name(&self) -> &'static str {
        "MouseMoveEvent"
    }
}

/// Event untuk mouse scroll
pub struct MouseScrollEvent {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl Event for MouseScrollEvent {
    fn name(&self) -> &'static str {
        "MouseScrollEvent"
    }
}

/// Event untuk window close
pub struct WindowCloseEvent;

impl Event for WindowCloseEvent {
    fn name(&self) -> &'static str {
        "WindowCloseEvent"
    }
}

/// Event untuk update frame
pub struct UpdateEvent {
    pub delta_time: f32,
}

impl Event for UpdateEvent {
    fn name(&self) -> &'static str {
        "UpdateEvent"
    }
}

/// Event untuk render frame
pub struct RenderEvent;

impl Event for RenderEvent {
    fn name(&self) -> &'static str {
        "RenderEvent"
    }
} 