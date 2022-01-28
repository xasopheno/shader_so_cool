mod render_pipeline;
mod vertex;
use rand::prelude::*;
use rand::Rng;

use crate::error::KintaroError;
use crate::renderable::OrigamiConfig;

use self::vertex::OrigamiVertex;

pub struct Origami {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<OrigamiVertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub uniforms: crate::uniforms::RealtimeUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

fn random_color() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

impl Origami {
    pub fn new_random_clear_color() -> (f64, f64, f64) {
        (random_color(), random_color(), random_color())
    }
    pub fn new_random_vertices() -> Vec<OrigamiVertex> {
        (0..15)
            .into_iter()
            .map(|_| OrigamiVertex::new_random())
            .collect()
    }
    pub fn new_random_indices(n: u16) -> Vec<u16> {
        let mut rng = rand::thread_rng();
        let mut num = || rng.gen_range(0..n);

        (0..20).map(|_| num()).collect()
    }
    pub fn init(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        config: OrigamiConfig,
    ) -> Result<Self, KintaroError> {
        todo!();
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        view: &wgpu::TextureView,
        clear: bool,
    ) {
        todo!()
    }
}
