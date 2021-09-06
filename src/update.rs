use crate::clock::{Clock, ClockResult};
use crate::instance::{make_instance_buffer, Instance};
use crate::render_op::ToInstance;
use crate::state::State;
use crate::vertex::make_vertex_buffer;

impl State {
    pub fn update(&mut self) {
        self.clock.update();
        let ClockResult {
            last_period,
            frame_count,
            total_elapsed,
        } = self.clock.current();
        self.renderpass.vertex_buffer = make_vertex_buffer(&self.device, self.vertices.as_slice());
        let mut new_instances: Vec<Instance> = self
            .op_stream
            .get_batch(total_elapsed)
            .into_iter()
            .map(|op| {
                op.into_instance(
                    &self.canvas.instance_displacement,
                    self.canvas.n_column,
                    self.canvas.n_row,
                )
            })
            .collect();

        self.renderpass.instances.append(&mut new_instances);
        self.renderpass.instances.iter_mut().for_each(|i| {
            i.update_state(last_period);
        });

        self.renderpass.instances.retain(|i| i.life > 0.0);
        self.renderpass.instance_buffer = make_instance_buffer(
            &self.renderpass.instances,
            (self.size.width, self.size.height),
            &self.device,
        );
        if frame_count % 400 == 0 {
            self.vertices = (self.vertices_fn)();
            self.clear_color = crate::helpers::new_random_clear_color();
        }
        // self.vertices.par_iter_mut().for_each(|v| v.update());
        self.camera_controller
            .update_camera(&mut self.camera, last_period);
        self.renderpass
            .uniforms
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.renderpass.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.renderpass.uniforms]),
        );
    }
}
