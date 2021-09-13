const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

pub async fn write_img(
    output_buffer: wgpu::Buffer,
    frame: u32,
    size: (u32, u32),
    device: &wgpu::Device,
) {
    let buffer_slice = output_buffer.slice(..);

    // NOTE: We have to create the mapping THEN device.poll() before await
    // the future. Otherwise the application will freeze.
    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data = buffer_slice.get_mapped_range();

    use image::{ImageBuffer, Rgba};
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(size.0, size.1, data).unwrap();
    let filename = format!("out/{:07}.png", frame);
    if frame % 100 == 0 {
        dbg!(&filename);
    }
    buffer.save(filename).unwrap();
    // self.output_buffer.unmap();
}
pub fn copy_texture_to_buffer(
    encoder: &mut wgpu::CommandEncoder,
    size: (u32, u32),
    device: &wgpu::Device,
    texture: &wgpu::Texture,
) -> wgpu::Buffer {
    let output_buffer = make_output_buffer(&device, size);

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(U32_SIZE * size.0),
                rows_per_image: std::num::NonZeroU32::new(size.1),
            },
        },
        wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
    );
    output_buffer
}

fn make_output_buffer(device: &wgpu::Device, size: (u32, u32)) -> wgpu::Buffer {
    let output_buffer_size = (U32_SIZE * size.0 * size.1) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST
        // this tells wpgu that we want to read this buffer from the cpu
        | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    device.create_buffer(&output_buffer_desc)
}
