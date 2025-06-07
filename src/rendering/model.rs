use std::{path::Path, sync::Arc};
use anyhow::Result;
use glam::{Mat4, Vec3, Quat};
use wgpu::{Device, Queue, BindGroup, Buffer, BufferUsages};
use wgpu::util::DeviceExt;
use crate::rendering::{Mesh, Material};
use bytemuck::{Pod, Zeroable};

/// A 3D model with mesh, material, and transform
pub struct Model {
    /// The mesh for this model
    pub mesh: Arc<Mesh>,
    /// The material for this model
    pub material: Arc<Material>,
    /// The transform for this model
    pub transform: ModelTransform,
    /// The model bind group
    pub model_bind_group: Option<BindGroup>,
    /// The model uniform buffer
    model_buffer: Option<Buffer>,
}

/// The transform of an object in 3D space
#[derive(Debug, Clone)]
pub struct ModelTransform {
    /// Position in 3D space
    pub position: Vec3,
    /// Rotation as a quaternion
    pub rotation: Quat,
    /// Scale in 3D space
    pub scale: Vec3,
}

impl Default for ModelTransform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl ModelTransform {
    /// Create a new transform
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new transform with a position
    pub fn with_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
    
    /// Create a new transform with a position and scale
    pub fn with_position_scale(position: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale,
        }
    }
    
    /// Get the model matrix for this transform
    pub fn model_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);
        let rotation = Mat4::from_quat(self.rotation);
        let scale = Mat4::from_scale(self.scale);
        
        translation * rotation * scale
    }
}

/// Uniform buffer for model data
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ModelUniform {
    /// Model matrix
    pub model: [[f32; 4]; 4],
}

impl Model {
    /// Create a new model
    pub fn new(
        mesh: Arc<Mesh>,
        material: Arc<Material>,
        transform: ModelTransform,
    ) -> Self {
        Self {
            mesh,
            material,
            transform,
            model_bind_group: None,
            model_buffer: None,
        }
    }
    
    /// Create a new model with device
    pub fn new_with_device(
        device: &Device,
        mesh: Arc<Mesh>,
        material: Arc<Material>,
        transform: ModelTransform,
    ) -> Self {
        // Create model uniform buffer
        let model_matrix = transform.model_matrix();
        let model_uniform = ModelUniform {
            model: model_matrix.to_cols_array_2d(),
        };
        
        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Uniform Buffer"),
            contents: bytemuck::cast_slice(&[model_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        // Create model bind group layout
        let model_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Model Bind Group Layout"),
        });
        
        // Create model bind group
        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout: &model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_buffer.as_entire_binding(),
            }],
        });
        
        Self {
            mesh,
            material,
            transform,
            model_bind_group: Some(model_bind_group),
            model_buffer: Some(model_buffer),
        }
    }
    
    /// Update the transform of this model
    pub fn update_transform(&self, queue: &Queue, transform: &ModelTransform) {
        if let Some(buffer) = &self.model_buffer {
            let model_matrix = transform.model_matrix();
            let model_uniform = ModelUniform {
                model: model_matrix.to_cols_array_2d(),
            };
            
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[model_uniform]));
        }
    }
    
    /// Load a model from an OBJ file
    pub async fn load_obj<P: AsRef<Path> + Clone>(
        _device: &Device,
        _queue: &Queue,
        obj_path: P,
    ) -> Result<Self> {
        let _obj_file = std::fs::read_to_string(&obj_path)?;
        
        // TODO: Implement OBJ loading
        
        Err(anyhow::anyhow!("OBJ loading not implemented yet"))
    }
}

// Re-export Transform as ModelTransform for compatibility
pub use self::ModelTransform as Transform; 