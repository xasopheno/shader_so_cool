use crate::camera::{Camera, CameraController, Projection};
use crate::{
    canvas::Canvas,
    clock::{Clock, ClockResult},
    instance::{make_instance_buffer, Instance},
    render_op::{OpStream, ToInstance},
    shared::RenderPassInput,
    vertex::make_vertex_buffer,
};

pub fn update(
    clock: &mut impl Clock,
    renderpass: &mut RenderPassInput,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    size: (u32, u32),
    camera: &mut Camera,
    camera_controller: &mut CameraController,
    projection: &Projection,
    canvas: &Canvas,
    op_stream: &mut OpStream,
) {
    clock.update();
    let time = clock.current();

    renderpass.vertex_buffer = make_vertex_buffer(device, renderpass.vertices.as_slice());

    update_instances(
        &time,
        op_stream,
        canvas,
        device,
        &mut renderpass.instances,
        &mut renderpass.instance_buffer,
        size,
    );

    if time.frame_count % 400 == 0 {
        renderpass.vertices = (renderpass.vertices_fn)();
        // self.clear_color = crate::helpers::new_random_clear_color();
    }
    // self.vertices.par_iter_mut().for_each(|v| v.update());
    camera_controller.update_camera(camera, time.last_period);
    renderpass.uniforms.update_view_proj(&camera, &projection);
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
) {
    let mut new_instances: Vec<Instance> = op_stream
        .get_batch(time.total_elapsed)
        .into_iter()
        .map(|op| op.into_instance(&canvas.instance_displacement, canvas.n_column, canvas.n_row))
        .collect();

    instances.append(&mut new_instances);
    instances.iter_mut().for_each(|i| {
        i.update_state(time.last_period);
    });

    instances.retain(|i| i.life > 0.0);
    *instance_buffer = make_instance_buffer(instances, (size.0, size.1), &device);
}
