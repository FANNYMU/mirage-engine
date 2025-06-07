use std::sync::Arc;
use wgpu::{
    Device, Queue, Surface, SurfaceConfiguration, Adapter,
    TextureFormat, TextureUsages, PresentMode, ShaderModule,
    RenderPipeline, CommandEncoder, TextureView,
};
use winit::window::Window;
use anyhow::Result;
use log::{info, warn};

/// Main renderer that handles the GPU device and rendering pipeline
pub struct Renderer {
    surface: Surface,
    device: Arc<Device>,
    queue: Arc<Queue>,
    config: SurfaceConfiguration,
    size: (u32, u32),
    clear_color: wgpu::Color,
}

impl Renderer {
    /// Create a new renderer with the given window
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        let size = (size.width, size.height);

        // Create instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // Create surface
        let surface = unsafe { instance.create_surface(window) }?;

        // Get adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find an appropriate adapter"))?;

        // Log adapter info
        log_adapter_info(&adapter);

        // Create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        // Get surface capabilities and preferred format
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Configure surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        info!("Renderer initialized with surface format: {:?}", surface_format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        })
    }

    /// Get a reference to the device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get a reference to the queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Get the current surface configuration
    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    /// Get the current surface size
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Set the clear color
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = wgpu::Color { r, g, b, a };
    }

    /// Resize the renderer surface
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            info!("Renderer resized to {}x{}", width, height);
        } else {
            warn!("Attempted to resize renderer to invalid dimensions: {}x{}", width, height);
        }
    }

    /// Begin a new render pass
    pub fn begin_frame(&self) -> Result<(CommandEncoder, TextureView)> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        Ok((encoder, view))
    }

    /// End the current frame and submit the command buffer
    pub fn end_frame(&self, encoder: CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }
    
    /// Render a simple frame with the clear color
    pub fn render_frame(&self) -> Result<()> {
        // Get output texture
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Create render pass
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Simple Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // Render pass automatically dropped here
        }
        
        // Submit command buffer
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Present the frame
        output.present();
        
        Ok(())
    }

    /// Create a shader module from WGSL source
    pub fn create_shader(&self, source: &str, label: Option<&str>) -> ShaderModule {
        self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    }
}

/// Log information about the graphics adapter
fn log_adapter_info(adapter: &Adapter) {
    let info = adapter.get_info();
    info!("Graphics adapter: {} ({:?})", info.name, info.backend);
    info!("Device type: {:?}", info.device_type);
    info!("Driver info: {:?}", info.driver);
} 