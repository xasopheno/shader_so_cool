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

pub fn make_renderpass(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    window_size: (u32, u32),
    format: wgpu::TextureFormat,
    mut shape: Shape,
    instancer: Box<dyn Instancer>,
    name: String,
) -> RenderPassInput {
    let ShapeGenResult { vertices, indices } = shape.gen(&vec![name.to_string()]);
    shape.update();
    let (instances, instance_buffer) = make_instances_and_instance_buffer(0, window_size, device);
    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        crate::uniforms::RealtimeUniforms::new(device);
    let render_pipeline =
        create_render_pipeline(device, shader, &uniform_bind_group_layout, format);

    RenderPassInput {
        vertex_buffer: create_vertex_buffer(device, vertices.as_slice()),
        index_buffer: create_index_buffer(device, indices.as_slice()),
        vertices,
        uniform_bind_group,
        instances,
        instance_buffer,
        instancer: instancer.clone(),
        uniform_buffer,
        uniforms,
        shape: shape.clone(),
        render_pipeline,
    }
}

pub fn make_renderpasses(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    window_size: (u32, u32),
    format: wgpu::TextureFormat,
    mut shape: Shape,
    instancer: Box<dyn Instancer>,
    // ops: Vec<Op4D>,
    // names: Vec<String>,
) -> Vec<RenderPassInput> {
    ["a"]
        .into_iter()
        .map(|_tag| {
            let ShapeGenResult { vertices, indices } = shape.gen(&vec!["a".to_string()]);
            shape.update();
            let (instances, instance_buffer) =
                make_instances_and_instance_buffer(0, window_size, device);
            let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
                crate::uniforms::RealtimeUniforms::new(device);
            let render_pipeline =
                create_render_pipeline(device, shader, &uniform_bind_group_layout, format);

            RenderPassInput {
                vertex_buffer: create_vertex_buffer(device, vertices.as_slice()),
                index_buffer: create_index_buffer(device, indices.as_slice()),
                vertices,
                uniform_bind_group,
                instances,
                instance_buffer,
                instancer: instancer.clone(),
                uniform_buffer,
                uniforms,
                shape: shape.clone(),
                render_pipeline,
            }
        })
        .collect()
}
