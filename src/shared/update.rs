use kintaro_egui_lib::InstanceMul;

use crate::{
    canvas::Canvas,
    clock::ClockResult,
    instance::{make_instance_buffer, Instance},
    op_stream::{OpStream, ToInstance},
    shared::RenderPassInput,
    vertex::make_vertex_buffer,
};

pub fn update(
    is_playing: bool,
    time: ClockResult,
    renderpass: &mut RenderPassInput,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    size: (u32, u32),
    canvas: &Canvas,
    instance_mul: InstanceMul,
) {
    if time.frame_count % 1000 == 0 {
        // renderpass.vertices = renderpass.shape.gen().vertices;
        // self.clear_color = crate::helpers::new_random_clear_color();
    }
    renderpass.vertex_buffer = make_vertex_buffer(device, renderpass.vertices.as_slice());
    if is_playing {
        update_instances(
            &time,
            &mut renderpass.op_stream,
            canvas,
            device,
            &mut renderpass.instances,
            &mut renderpass.instance_buffer,
            size,
            instance_mul,
        );
    }
    // renderpass.vertices.iter_mut().for_each(|v| v.update());
    queue.write_buffer(
        &renderpass.uniform_buffer,
        0,
        bytemuck::cast_slice(&[renderpass.uniforms]),
    );
}

fn update_instances(
    time: &ClockResult,
    op_stream: &mut OpStream,
    canvas: &Canvas,
    device: &wgpu::Device,
    instances: &mut Vec<Instance>,
    instance_buffer: &mut wgpu::Buffer,
    size: (u32, u32),
    mul: InstanceMul,
) {
    let mut new_instances: Vec<Instance> = op_stream
        .get_batch(time.total_elapsed)
        .into_iter()
        .map(|op| {
            op.into_instance(
                &canvas.instance_displacement,
                canvas.n_column,
                canvas.n_row,
                mul,
            )
        })
        .collect();

    instances.append(&mut new_instances);
    instances.iter_mut().for_each(|i| {
        i.update_state(time.last_period);
    });

    instances.retain(|i| i.life > 0.0);
    *instance_buffer = make_instance_buffer(instances, (size.0, size.1), &device);
}
