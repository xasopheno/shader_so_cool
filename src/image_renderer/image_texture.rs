use std::num::NonZeroU32;
use wgpu::{TextureFormat, TextureViewDescriptor};

use anyhow::*;

#[derive(Debug)]
pub struct ImageTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl ImageTexture {
    pub fn from_image(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self> {
        let img = image::io::Reader::open("./src/image_renderer/milo.png")?.decode()?;
        let rgba = img
            .as_rgba8()
            .ok_or_else(|| anyhow::format_err!("Image can't be interpreted as rgba8"))?;

        let height = rgba.height();
        let width = rgba.width();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let height = rgba.height();
        let width = rgba.width();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * width),
                rows_per_image: NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor {
            format: Some(TextureFormat::Rgba8UnormSrgb),
            ..wgpu::TextureViewDescriptor::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
