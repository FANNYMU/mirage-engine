use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use log::{info, debug};

pub struct KeyboardState {
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub keys_released: Vec<VirtualKeyCode>,
    pub keys_down: Vec<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            keys_down: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_released(&self, key: VirtualKeyCode) -> bool {
        self.keys_released.contains(&key)
    }
}

pub struct MouseState {
    pub position: (f64, f64),
    pub buttons_pressed: Vec<MouseButton>,
    pub buttons_released: Vec<MouseButton>,
    pub buttons_down: Vec<MouseButton>,
    pub scroll_delta: (f32, f32),
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            buttons_pressed: Vec::new(),
            buttons_released: Vec::new(),
            buttons_down: Vec::new(),
            scroll_delta: (0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.scroll_delta = (0.0, 0.0);
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.buttons_down.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }
}

pub struct WindowManager {
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    window_size: (u32, u32),
    should_close: bool,
}

impl WindowManager {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
            .with_resizable(true)
            .build(&event_loop)
            .expect("Failed to create window");
        
        info!("Window created: {}x{}", width, height);
        
        Self {
            event_loop: Some(event_loop),
            window: Some(window),
            keyboard_state: KeyboardState::new(),
            mouse_state: MouseState::new(),
            window_size: (width, height),
            should_close: false,
        }
    }
    
    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }
    
    pub fn window_size(&self) -> (u32, u32) {
        self.window_size
    }
    
    pub fn keyboard_state(&self) -> &KeyboardState {
        &self.keyboard_state
    }
    
    pub fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }
    
    pub fn should_close(&self) -> bool {
        self.should_close
    }
    
    pub fn request_close(&mut self) {
        self.should_close = true;
    }
    
    pub fn run<F>(mut self, mut callback: F) 
    where 
        F: FnMut(&mut Self, &Event<()>) + 'static
    {
        let event_loop = self.event_loop.take().expect("Event loop already taken");
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    info!("Window close requested");
                    self.should_close = true;
                }
                
                Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                    debug!("Window resized to {}x{}", physical_size.width, physical_size.height);
                    self.window_size = (physical_size.width, physical_size.height);
                }
                
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    self.handle_keyboard_input(input);
                }
                
                Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                    self.mouse_state.position = (position.x, position.y);
                }
                
                Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {
                    self.handle_mouse_input(state, button);
                }
                
                Event::WindowEvent { event: WindowEvent::MouseWheel { delta, .. }, .. } => {
                    match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            self.mouse_state.scroll_delta = (x, y);
                        }
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            self.mouse_state.scroll_delta = (pos.x as f32, pos.y as f32);
                        }
                    }
                }
                
                Event::MainEventsCleared => {
                    if self.should_close {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                
                _ => {}
            }
            
            callback(&mut self, &event);
            
            if let Event::MainEventsCleared = event {
                self.keyboard_state.update();
                self.mouse_state.update();
            }
        });
    }
    
    fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    if !self.keyboard_state.keys_down.contains(&key_code) {
                        self.keyboard_state.keys_pressed.push(key_code);
                        self.keyboard_state.keys_down.push(key_code);
                    }
                }
                ElementState::Released => {
                    if let Some(index) = self.keyboard_state.keys_down.iter().position(|&k| k == key_code) {
                        self.keyboard_state.keys_down.remove(index);
                    }
                    self.keyboard_state.keys_released.push(key_code);
                }
            }
        }
    }
    
    fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_state.buttons_down.contains(&button) {
                    self.mouse_state.buttons_pressed.push(button);
                    self.mouse_state.buttons_down.push(button);
                }
            }
            ElementState::Released => {
                if let Some(index) = self.mouse_state.buttons_down.iter().position(|&b| b == button) {
                    self.mouse_state.buttons_down.remove(index);
                }
                self.mouse_state.buttons_released.push(button);
            }
        }
    }
} 