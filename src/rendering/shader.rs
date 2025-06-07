use wgpu::{Device, ShaderModule};

/// A shader module that can be used for rendering
pub struct Shader {
    pub module: ShaderModule,
    pub name: String,
    pub entry_point: String,
}

impl Shader {
    /// Create a new shader from WGSL source code
    pub fn from_wgsl(device: &Device, source: &str, name: &str) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Self {
            module,
            name: name.to_string(),
            entry_point: "main".to_string(),
        }
    }

    /// Create a new shader from WGSL source code with a specific entry point
    pub fn from_wgsl_with_entry_point(
        device: &Device,
        source: &str,
        name: &str,
        entry_point: &str,
    ) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Self {
            module,
            name: name.to_string(),
            entry_point: entry_point.to_string(),
        }
    }
}

/// Default 2D sprite shader
pub fn create_sprite_shader(device: &Device) -> Shader {
    let shader_src = r#"
    struct VertexInput {
        @location(0) position: vec3<f32>,
        @location(1) normal: vec3<f32>,
        @location(2) tex_coords: vec2<f32>,
        @location(3) color: vec4<f32>,
    };

    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) tex_coords: vec2<f32>,
        @location(1) color: vec4<f32>,
    };

    struct CameraUniform {
        view_proj: mat4x4<f32>,
    };
    @group(0) @binding(0) var<uniform> camera: CameraUniform;

    struct ModelUniform {
        model: mat4x4<f32>,
    };
    @group(1) @binding(0) var<uniform> model: ModelUniform;

    @vertex
    fn vs_main(in: VertexInput) -> VertexOutput {
        var out: VertexOutput;
        out.clip_position = camera.view_proj * model.model * vec4<f32>(in.position, 1.0);
        out.tex_coords = in.tex_coords;
        out.color = in.color;
        return out;
    }

    @group(2) @binding(0) var t_diffuse: texture_2d<f32>;
    @group(2) @binding(1) var s_diffuse: sampler;

    @fragment
    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
        let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
        return tex_color * in.color;
    }
    "#;

    Shader::from_wgsl(device, shader_src, "Sprite Shader")
}

/// Default 2D unlit shader
pub fn create_unlit_shader(device: &Device) -> Shader {
    let shader_src = r#"
    struct VertexInput {
        @location(0) position: vec3<f32>,
        @location(1) normal: vec3<f32>,
        @location(2) tex_coords: vec2<f32>,
        @location(3) color: vec4<f32>,
    };

    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) color: vec4<f32>,
    };

    struct CameraUniform {
        view_proj: mat4x4<f32>,
    };
    @group(0) @binding(0) var<uniform> camera: CameraUniform;

    struct ModelUniform {
        model: mat4x4<f32>,
    };
    @group(1) @binding(0) var<uniform> model: ModelUniform;

    @vertex
    fn vs_main(in: VertexInput) -> VertexOutput {
        var out: VertexOutput;
        out.clip_position = camera.view_proj * model.model * vec4<f32>(in.position, 1.0);
        out.color = in.color;
        return out;
    }

    @fragment
    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
        return in.color;
    }
    "#;

    Shader::from_wgsl(device, shader_src, "Unlit Shader")
} 