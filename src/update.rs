use crate::camera::{Camera, CameraController, Projection};
use crate::clock::{Clock, ClockResult};
use crate::instance::{make_instance_buffer, Instance};
use crate::render_op::{OpStream, ToInstance};
use crate::state::{Canvas, RenderPassInput, State};
use crate::vertex::make_vertex_buffer;

impl State {
    pub fn update(&mut self) {
        update(
            &mut self.clock,
            &mut self.renderpass,
            &self.device,
            &self.queue,
            (self.size.width, self.size.height),
            &mut self.camera,
            &mut self.camera_controller,
            &self.projection,
            &self.canvas,
            &mut self.op_stream,
        )
    }
}

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
    let ClockResult {
        last_period,
        frame_count,
        total_elapsed,
    } = clock.current();

    renderpass.vertex_buffer = make_vertex_buffer(device, renderpass.vertices.as_slice());

    make_new_instances(
        total_elapsed,
        last_period,
        op_stream,
        canvas,
        device,
        &mut renderpass.instances,
        &mut renderpass.instance_buffer,
        size,
    );

    if frame_count % 400 == 0 {
        renderpass.vertices = (renderpass.vertices_fn)();
        // self.clear_color = crate::helpers::new_random_clear_color();
    }
    // self.vertices.par_iter_mut().for_each(|v| v.update());
    camera_controller.update_camera(camera, last_period);
    renderpass.uniforms.update_view_proj(&camera, &projection);
    queue.write_buffer(
        &renderpass.uniform_buffer,
        0,
        bytemuck::cast_slice(&[renderpass.uniforms]),
    );
}

fn make_new_instances(
    total_elapsed: f32,
    last_period: f32,
    op_stream: &mut OpStream,
    canvas: &Canvas,
    device: &wgpu::Device,
    instances: &mut Vec<Instance>,
    instance_buffer: &mut wgpu::Buffer,
    size: (u32, u32),
) {
    let mut new_instances: Vec<Instance> = op_stream
        .get_batch(total_elapsed)
        .into_iter()
        .map(|op| op.into_instance(&canvas.instance_displacement, canvas.n_column, canvas.n_row))
        .collect();

    instances.append(&mut new_instances);
    instances.iter_mut().for_each(|i| {
        i.update_state(last_period);
    });

    instances.retain(|i| i.life > 0.0);
    *instance_buffer = make_instance_buffer(instances, (size.0, size.1), &device);
}
