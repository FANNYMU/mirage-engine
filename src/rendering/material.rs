use wgpu::{
    Device, RenderPipeline, BindGroup, BindGroupLayout, Buffer,
    ColorTargetState, BlendState, VertexState, FragmentState,
    PrimitiveState, MultisampleState, DepthStencilState, CompareFunction,
    StencilState, BindGroupLayoutEntry, ShaderStages, BindingType,
    TextureSampleType, SamplerBindingType, TextureViewDimension,
    TextureFormat,
};
use crate::rendering::{Shader, Texture, Vertex};

/// A material for rendering objects
pub struct Material {
    /// Name of the material
    pub name: String,
    /// The render pipeline for this material
    pub pipeline: RenderPipeline,
    /// The bind group for this material
    pub bind_group: BindGroup,
    /// The bind group layout for this material
    pub bind_group_layout: BindGroupLayout,
    /// The model bind group layout
    pub model_bind_group_layout: BindGroupLayout,
}

impl Material {
    /// Create a new material with the given shader and texture
    pub fn new(
        device: &Device,
        name: &str,
        shader: &Shader,
        camera_bind_group_layout: &BindGroupLayout,
        model_bind_group_layout: &BindGroupLayout,
        texture: Option<&Texture>,
        format: TextureFormat,
    ) -> Self {
        // Create material bind group layout
        let material_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some(&format!("{} Material Bind Group Layout", name)),
        });
        
        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{} Pipeline Layout", name)),
            bind_group_layouts: &[
                camera_bind_group_layout,
                model_bind_group_layout,
                &material_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{} Pipeline", name)),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader.module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader.module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create a white texture if none is provided
        let owned_white_texture;
        
        // Get texture and sampler
        let (texture_view, sampler) = if let Some(texture) = texture {
            (&texture.view, &texture.sampler)
        } else {
            owned_white_texture = create_white_texture(device);
            (&owned_white_texture.view, &owned_white_texture.sampler)
        };
        
        // Create material bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
            label: Some(&format!("{} Material Bind Group", name)),
        });
        
        // Create a new model bind group layout
        let model_bind_group_layout_owned = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some(&format!("{} Model Bind Group Layout", name)),
            }
        );
        
        Self {
            name: name.to_string(),
            pipeline,
            bind_group,
            bind_group_layout: material_bind_group_layout,
            model_bind_group_layout: model_bind_group_layout_owned,
        }
    }
    
    /// Get the model bind group layout
    pub fn get_model_bind_group_layout(&self) -> &BindGroupLayout {
        &self.model_bind_group_layout
    }
    
    /// Create a new bind group
    /// 
    /// Note: This is a placeholder implementation. In a real application, you would need
    /// to pass the device and appropriate resources to create a new bind group.
    pub fn create_bind_group(&self) -> BindGroup {
        // This is a placeholder and will cause a panic in real usage
        // In a real implementation, you would need to pass the device and resources
        panic!("create_bind_group() is not implemented - you need to pass a device and resources");
    }
    
    /// Create a placeholder buffer
    /// 
    /// Note: This is a placeholder implementation. In a real application, you would need
    /// to pass the device to create a buffer.
    pub fn create_buffer(&self) -> Buffer {
        // This is a placeholder and will cause a panic in real usage
        // In a real implementation, you would need to pass the device
        panic!("create_buffer() is not implemented - you need to pass a device");
    }
}

/// Create a 1x1 white texture
fn create_white_texture(device: &Device) -> Texture {
    let size = 1u32;
    
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("White Texture"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    
    Texture {
        texture,
        view,
        sampler,
        size: (size, size),
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
    }
} 