use crate::{
    camera::Cameras,
    canvas::Canvas,
    error::KintaroError,
    renderable::{Renderable, RenderableInput},
};
use kintaro_egui_lib::InstanceMul;
use weresocool::generation::json::Op4D;
use winit::event::{ElementState, VirtualKeyCode};

use crate::clock::Clock;

use super::Composition;

impl Composition {
    pub fn handle_keyboard_input(
        &mut self,
        key: VirtualKeyCode,
        state: ElementState,
        cameras: &mut Cameras,
    ) {
        cameras.current.controller.process_keyboard(key, state);
        self.renderables.0.iter_mut().for_each(|renderable| {
            renderable.process_keyboard(key, state);
        });
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        clock: &impl Clock,
        instance_mul: InstanceMul,
        canvas: &Canvas,
        cameras: &mut Cameras,
        ops: &Vec<Op4D>,
    ) -> Result<(), KintaroError> {
        let clock_result = clock.current();
        cameras.current.update(clock_result.last_period);

        let view_position: [f32; 4] = cameras.current.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (cameras.current.projection.calc_matrix() * cameras.current.calc_matrix()).into();

        let render_input = RenderableInput {
            device,
            queue,
            clock_result,
            canvas,
            size,
            view_position,
            view_proj,
            instance_mul,
            clear: false,
            frames: &self.frames,
            ops,
        };

        for (idx, renderable) in self.renderables.0.iter_mut().enumerate() {
            renderable.update(&render_input)?;
            renderable.render_pass(&render_input, idx == 0)?;
        }

        Ok(())
    }
}
