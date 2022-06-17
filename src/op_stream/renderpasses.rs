use crate::instance::make_instances_and_instance_buffer;
use crate::op_stream::OpInput;
use crate::shared::create_render_pipeline;
use crate::shared::RenderPassInput;
use crate::vertex::shape::ShapeGenResult;
use crate::vertex::{create_index_buffer, create_vertex_buffer};
use crate::Instancer;
use crate::Shape;
use weresocool::generation::json::Op4D;

use super::OpStream;

pub fn make_renderpasses(
    device: &wgpu::Device,
    // op_streams: OpInput,
    shader: &wgpu::ShaderModule,
    window_size: (u32, u32),
    format: wgpu::TextureFormat,
    mut shape: Shape,
    instancer: Box<dyn Instancer>,
    ops: Vec<Op4D>,
) -> Vec<RenderPassInput> {
    // op_streams
    // .iter()
    // .map(|op_stream| {
    let ShapeGenResult { vertices, indices } = shape.gen(&vec!["a".to_string()]);
    shape.update();
    let (instances, instance_buffer) = make_instances_and_instance_buffer(0, window_size, device);
    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        crate::uniforms::RealtimeUniforms::new(device);
    let render_pipeline =
        create_render_pipeline(device, shader, &uniform_bind_group_layout, format);

    let result = RenderPassInput {
        vertex_buffer: create_vertex_buffer(device, vertices.as_slice()),
        index_buffer: create_index_buffer(device, indices.as_slice()),
        // vertex_buffer: create_vertex_buffer(device, &[]),
        // index_buffer: create_index_buffer(device, &[]),
        vertices,
        uniform_bind_group,
        instances,
        instance_buffer,
        instancer: instancer.clone(),
        uniform_buffer,
        uniforms,
        shape: shape.clone(),
        render_pipeline,
        ops,
    };
    vec![result]
    // })
    // .collect()
}
