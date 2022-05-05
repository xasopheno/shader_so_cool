use crate::{
    error::KintaroError,
    renderable::{Renderable, RenderableInput},
};
use kintaro_egui_lib::InstanceMul;
use wgpu::TextureView;
use winit::event::{ElementState, VirtualKeyCode};

use crate::clock::Clock;

use super::Composition;

impl Composition {
    pub fn handle_keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.camera.controller.process_keyboard(key, state);
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
        // view: &TextureView,
    ) -> Result<(), KintaroError> {
        let clock_result = clock.current();
        self.camera.update(clock_result.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        let render_input = RenderableInput {
            device,
            queue,
            // view,
            clock_result,
            canvas: &self.canvas,
            config: &self.config,
            size,
            view_position,
            view_proj,
            instance_mul,
            clear: false,
            frames: &self.frames,
        };

        for (idx, renderable) in self.renderables.0.iter_mut().enumerate() {
            renderable.update(&render_input)?;
            renderable.render_pass(&render_input, idx == 0)?;
        }

        Ok(())
    }
}
