use std::num::NonZeroU32;

use anyhow::*;
use image::GenericImageView;

use crate::shared::create_render_pipeline;

#[derive(Debug)]
pub struct ImageTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Copy, Clone, Debug)]
pub struct ImageDims {
    nrows: u32,
    ncols: u32,
}

pub struct ImageRender {
    pub frame: usize,
    // pub shader: wgpu::ShaderModule,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ImageRender {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let (pipeline_layout, render_pipeline) = create_image_render_pipeline(device, queue);
        Self {
            frame: 0,
            render_pipeline,
        }
    }
}

pub fn create_image_render_pipeline(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::PipelineLayout, wgpu::RenderPipeline) {
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
    });
    let (image_bind_group, image_bind_group_layout, image_texture) =
        create_image_bind_group(device, queue);
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),           // NEW!
        bind_group_layouts: &[&image_bind_group_layout], // NEW!
        push_constant_ranges: &[],
    });
    (
        render_pipeline_layout,
        create_render_pipeline(device, &shader, None, wgpu::TextureFormat::Bgra8UnormSrgb),
    )
}

pub fn create_image_bind_group(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout, ImageTexture) {
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        // This is only for TextureSampleType::Depth
                        comparison: false,
                        // This should be true if the sample_type of the texture is:
                        //     TextureSampleType::Float { filterable: true }
                        // Otherwise you'll get an error.
                        filtering: true,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });
    let diffuse_texture = ImageTexture::from_image(&device, &queue).unwrap();
    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    });

    (
        diffuse_bind_group,
        texture_bind_group_layout,
        diffuse_texture,
    )
}

impl ImageTexture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // img: &image::DynamicImage,
        // label: Option<&str>,
    ) -> Result<Self> {
        let img = image::io::Reader::open("./happy-tree-cartoon.png")?.decode()?;
        let dimensions = img.dimensions();
        let rgba = img
            .as_rgba8()
            .ok_or_else(|| anyhow::format_err!("Image can't be interpreted as rgba8"))?;
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let nrows = rgba.height();
        let ncols = rgba.width();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: ncols,
                height: nrows,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
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
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    // pub fn save_rgb_texture(
    // device: &wgpu::Device,
    // queue: &wgpu::Queue,
    // dims: &ImageDims,
    // texture: &wgpu::Texture,
    // // path: P,
    // ) -> Result<(), anyhow::Error> {
    // dbg!(dims);
    // let staging = device.create_buffer(&wgpu::BufferDescriptor {
    // label: None,
    // size: (dims.num_pixels() * 4) as _,
    // usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    // mapped_at_creation: false,
    // });
    // let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    // label: Some("ImageEncoder"),
    // });
    // encoder.copy_texture_to_buffer(
    // wgpu::ImageCopyTexture {
    // texture,
    // mip_level: 0,
    // origin: wgpu::Origin3d::ZERO,
    // aspect: wgpu::TextureAspect::All,
    // },
    // wgpu::ImageCopyBuffer {
    // buffer: &staging,
    // layout: wgpu::ImageDataLayout {
    // offset: 0,
    // bytes_per_row: std::num::NonZeroU32::new(4 * dims.ncols),
    // rows_per_image: std::num::NonZeroU32::new(dims.nrows),
    // },
    // },
    // wgpu::Extent3d {
    // width: dims.ncols,
    // height: dims.nrows,
    // depth_or_array_layers: 1,
    // },
    // );
    // queue.submit(Some(encoder.finish()));
    // Ok(())
    // }

    // pub fn load_texture(
    // device: &wgpu::Device,
    // queue: &wgpu::Queue,
    // // path: &str,
    // ) -> Result<(ImageDims, wgpu::Texture, Vec<u8>), anyhow::Error> {
    // let img = image::io::Reader::open("./happy-tree-cartoon.png")?.decode()?;
    // let rgba = img
    // .as_rgba8()
    // .ok_or_else(|| anyhow::format_err!("Image can't be interpreted as rgba8"))?;
    // let nrows = rgba.height();
    // let ncols = rgba.width();
    // let texture = device.create_texture(&wgpu::TextureDescriptor {
    // label: None,
    // size: wgpu::Extent3d {
    // width: ncols,
    // height: nrows,
    // depth_or_array_layers: 1,
    // },
    // mip_level_count: 1,
    // sample_count: 1,
    // dimension: wgpu::TextureDimension::D2,
    // format: wgpu::TextureFormat::Rgba8Unorm,
    // usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
    // });
    // queue.write_texture(
    // wgpu::ImageCopyTexture {
    // texture: &texture,
    // mip_level: 0,
    // origin: wgpu::Origin3d::ZERO,
    // aspect: wgpu::TextureAspect::All,
    // },
    // &rgba,
    // wgpu::ImageDataLayout {
    // offset: 0,
    // bytes_per_row: std::num::NonZeroU32::new(4 * ncols),
    // rows_per_image: std::num::NonZeroU32::new(nrows),
    // },
    // wgpu::Extent3d {
    // width: ncols,
    // height: nrows,
    // depth_or_array_layers: 1,
    // },
    // );
    // let dims = ImageDims { nrows, ncols };
    // Ok((dims, texture, rgba.to_vec()))
    // }
}
