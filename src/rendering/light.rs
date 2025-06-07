use glam::{Vec3, Vec4};
use wgpu::{Device, Queue, Buffer, BindGroup, BindGroupLayout, BufferUsages};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

/// Base trait for all light types
pub trait Light {
    /// Get the light data for the shader
    fn get_light_data(&self) -> LightData;
    
    /// Get the light type
    fn get_type(&self) -> LightType;
}

/// Types of lights supported
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LightType {
    Directional = 0,
    Point = 1,
}

/// Light data for the shader
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightData {
    pub position: [f32; 3],
    pub light_type: u32,
    pub color: [f32; 3],
    pub intensity: f32,
    pub direction: [f32; 3],
    pub range: f32,
}

/// Directional light (sun-like)
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl DirectionalLight {
    /// Create a new directional light
    pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}

impl Light for DirectionalLight {
    fn get_light_data(&self) -> LightData {
        LightData {
            position: [0.0, 0.0, 0.0], // Not used for directional lights
            light_type: LightType::Directional as u32,
            color: [self.color.x, self.color.y, self.color.z],
            intensity: self.intensity,
            direction: [self.direction.x, self.direction.y, self.direction.z],
            range: 0.0, // Not used for directional lights
        }
    }
    
    fn get_type(&self) -> LightType {
        LightType::Directional
    }
}

/// Point light (omni-directional)
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub range: f32,
}

impl PointLight {
    /// Create a new point light
    pub fn new(position: Vec3, color: Vec3, intensity: f32, range: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            range,
        }
    }
}

impl Light for PointLight {
    fn get_light_data(&self) -> LightData {
        LightData {
            position: [self.position.x, self.position.y, self.position.z],
            light_type: LightType::Point as u32,
            color: [self.color.x, self.color.y, self.color.z],
            intensity: self.intensity,
            direction: [0.0, 0.0, 0.0], // Not used for point lights
            range: self.range,
        }
    }
    
    fn get_type(&self) -> LightType {
        LightType::Point
    }
}

/// Light manager that handles all lights in the scene
pub struct LightManager {
    lights: Vec<Box<dyn Light>>,
    light_buffer: Option<Buffer>,
    light_bind_group: Option<BindGroup>,
    light_bind_group_layout: Option<BindGroupLayout>,
    light_count_buffer: Option<Buffer>,
    max_lights: usize,
}

impl LightManager {
    /// Create a new light manager
    pub fn new(max_lights: usize) -> Self {
        Self {
            lights: Vec::new(),
            light_buffer: None,
            light_bind_group: None,
            light_bind_group_layout: None,
            light_count_buffer: None,
            max_lights,
        }
    }
    
    /// Add a light to the manager
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        if self.lights.len() < self.max_lights {
            self.lights.push(light);
        }
    }
    
    /// Get the number of lights
    pub fn light_count(&self) -> usize {
        self.lights.len()
    }
    
    /// Initialize the light buffer and bind group
    pub fn initialize(&mut self, device: &Device) {
        // Create light buffer with space for max_lights
        let light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Light Buffer"),
            size: std::mem::size_of::<LightData>() as u64 * self.max_lights as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create light count buffer
        let light_count_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Count Buffer"),
            contents: bytemuck::cast_slice(&[self.lights.len() as u32]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        
        // Create light bind group layout
        let light_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Light Bind Group Layout"),
        });
        
        // Create light bind group
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_count_buffer.as_entire_binding(),
                },
            ],
            label: Some("Light Bind Group"),
        });
        
        self.light_buffer = Some(light_buffer);
        self.light_bind_group_layout = Some(light_bind_group_layout);
        self.light_bind_group = Some(light_bind_group);
        self.light_count_buffer = Some(light_count_buffer);
    }
    
    /// Update the light buffer with current light data
    pub fn update(&self, queue: &Queue) {
        if let Some(light_buffer) = &self.light_buffer {
            let mut light_data = Vec::with_capacity(self.max_lights);
            
            // Add data for each light
            for light in &self.lights {
                light_data.push(light.get_light_data());
            }
            
            // Pad with empty lights if needed
            while light_data.len() < self.max_lights {
                light_data.push(LightData {
                    position: [0.0, 0.0, 0.0],
                    light_type: 0,
                    color: [0.0, 0.0, 0.0],
                    intensity: 0.0,
                    direction: [0.0, 0.0, 0.0],
                    range: 0.0,
                });
            }
            
            // Write light data to buffer
            queue.write_buffer(light_buffer, 0, bytemuck::cast_slice(&light_data));
            
            // Update light count
            if let Some(light_count_buffer) = &self.light_count_buffer {
                queue.write_buffer(light_count_buffer, 0, bytemuck::cast_slice(&[self.lights.len() as u32]));
            }
        }
    }
    
    /// Get the light bind group
    pub fn bind_group(&self) -> Option<&BindGroup> {
        self.light_bind_group.as_ref()
    }
    
    /// Get the light bind group layout
    pub fn bind_group_layout(&self) -> Option<&BindGroupLayout> {
        self.light_bind_group_layout.as_ref()
    }
} 