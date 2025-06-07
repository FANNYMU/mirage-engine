use wgpu::{
    util::DeviceExt, Buffer, BufferUsages, Device, VertexBufferLayout, VertexAttribute,
    VertexFormat, VertexStepMode,
};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use anyhow::Result;

/// A vertex with position, normal, texture coordinates, and color
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    /// Create a new vertex
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2, color: [f32; 4]) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            tex_coords: tex_coords.to_array(),
            color,
        }
    }

    /// Get the vertex buffer layout for this vertex type
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Position
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                // Normal
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                // Texture coordinates
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x2,
                },
                // Color
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// A mesh represents a collection of vertices and indices that form a 3D model
pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    pub name: String,
}

impl Mesh {
    /// Create a new mesh from vertices and indices
    pub fn new(
        device: &Device,
        name: &str,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Result<Self> {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", name)),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        Ok(Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            name: name.to_string(),
        })
    }

    /// Get the vertex buffer
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    /// Get the index buffer
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    /// Get the number of indices
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    /// Create a quad mesh (rectangle)
    pub fn create_quad(device: &Device, width: f32, height: f32) -> Result<Self> {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let vertices = [
            // Bottom left
            Vertex::new(
                Vec3::new(-half_width, -half_height, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            // Bottom right
            Vertex::new(
                Vec3::new(half_width, -half_height, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(1.0, 1.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            // Top right
            Vertex::new(
                Vec3::new(half_width, half_height, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(1.0, 0.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            // Top left
            Vertex::new(
                Vec3::new(-half_width, half_height, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(0.0, 0.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
        ];

        let indices = [
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
        ];

        Self::new(device, "Quad", &vertices, &indices)
    }
} 