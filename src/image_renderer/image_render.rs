use super::image_texture::ImageTexture;
pub use super::{
    create_image_render_pipeline::create_image_render_pipeline,
    image_vertex::make_image_vertices_and_indices,
};

#[derive(Debug)]
pub struct ImageRender {
    pub frame: usize,
    pub num_indices: u32,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ImageRender {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        image_texture: &ImageTexture,
    ) -> Self {
        let (bind_group, render_pipeline) =
            create_image_render_pipeline(device, format, image_texture);
        let (vertex_buffer, index_buffer, num_indices) = make_image_vertices_and_indices(device);
        Self {
            frame: 0,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            bind_group,
        }
    }
    pub fn render_pass(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Image Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Image Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}
