use crate::instance::make_instances_and_instance_buffer;
use crate::shared::create_render_pipeline;
use crate::shared::RenderPassInput;
use crate::vertex::shape::ShapeGenResult;
use crate::vertex::{create_index_buffer, create_vertex_buffer};
use crate::Instancer;
use crate::Shape;

use super::OpStream;

pub fn make_renderpasses(
    device: &wgpu::Device,
    // op_streams: Vec<OpStream>,
    shader: &wgpu::ShaderModule,
    window_size: (u32, u32),
    format: wgpu::TextureFormat,
    mut shape: Shape,
    instancer: Box<dyn Instancer>,
) -> Vec<RenderPassInput> {
    // op_streams
    // .iter()
    // .map(|op_stream| {
    let mut op_stream = OpStream::init_empty();
    let ShapeGenResult { vertices, indices } = shape.gen(&op_stream);
    shape.update();
    let (instances, instance_buffer) = make_instances_and_instance_buffer(0, window_size, device);
    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        crate::uniforms::RealtimeUniforms::new(device);
    let render_pipeline =
        create_render_pipeline(device, shader, &uniform_bind_group_layout, format);

    let result = RenderPassInput {
        vertex_buffer: create_vertex_buffer(device, vertices.as_slice()),
        index_buffer: create_index_buffer(device, indices.as_slice()),
        op_stream,
        // vertex_buffer: create_vertex_buffer(device, &[]),
        // index_buffer: create_index_buffer(device, &[]),
        vertices: vec![],
        uniform_bind_group,
        instances,
        instance_buffer,
        instancer: instancer.clone(),
        uniform_buffer,
        uniforms,
        shape: shape.clone(),
        render_pipeline,
    };
    vec![result]
    // })
    // .collect()
}
