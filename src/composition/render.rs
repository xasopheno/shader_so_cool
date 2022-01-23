use crate::{
    error::KintaroError,
    renderable::{Renderable, RenderableEnum, RenderableInput},
};
use kintaro_egui_lib::InstanceMul;
use wgpu::TextureView;

use crate::clock::Clock;

use super::Composition;

impl Composition {
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        clock: &impl Clock,
        instance_mul: InstanceMul,
        view: &TextureView,
    ) -> Result<(), KintaroError> {
        let clock_result = clock.current();
        self.camera.update(clock_result.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        let render_input = RenderableInput {
            device,
            queue,
            view,
            clock_result,
            canvas: &self.canvas,
            config: &self.config,
            size,
            view_position,
            view_proj,
            instance_mul,
            clear: false,
        };

        self.renderables.iter_mut().for_each(|renderable| {
            renderable.update(&render_input).unwrap();
            renderable.render_pass(&render_input).unwrap();
        });
        Ok(())
    }
}
