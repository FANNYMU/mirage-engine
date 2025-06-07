use std::path::Path;
use anyhow::Result;
use image::{GenericImageView, DynamicImage};
use wgpu::{
    Device, Queue, Texture as WgpuTexture, TextureView, Sampler, TextureUsages,
    TextureDescriptor, Extent3d, TextureDimension, TextureViewDescriptor,
    SamplerDescriptor, FilterMode, AddressMode, CompareFunction, TextureFormat,
};

/// A texture with a view and sampler
pub struct Texture {
    /// The underlying WGPU texture
    pub texture: WgpuTexture,
    /// The texture view
    pub view: TextureView,
    /// The texture sampler
    pub sampler: Sampler,
    /// The size of the texture (width, height)
    pub size: (u32, u32),
    /// The format of the texture
    pub format: TextureFormat,
}

impl Texture {
    /// The format to use for depth textures
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    /// Create a new texture from raw data
    pub fn from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    /// Create a new texture from a file
    pub fn from_file<P: AsRef<Path> + Clone>(
        device: &Device,
        queue: &Queue,
        path: P,
        label: Option<&str>,
    ) -> Result<Self> {
        let img = image::open(path.clone())?;
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let label_str = label.unwrap_or(&path_str);
        Self::from_image(device, queue, &img, Some(label_str))
    }

    /// Create a new texture from an image
    pub fn from_image(
        device: &Device,
        queue: &Queue,
        img: &DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = TextureFormat::Rgba8UnormSrgb;

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
            size: dimensions,
            format,
        })
    }

    /// Create a new depth texture
    pub fn create_depth_texture(
        device: &Device,
        width: u32,
        height: u32,
        label: &str,
    ) -> Self {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            compare: Some(CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size: (width, height),
            format: Self::DEPTH_FORMAT,
        }
    }
} 