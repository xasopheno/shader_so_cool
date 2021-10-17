use crate::color::{GenColor, GenIndex, GenPosition, GenVertex, Index};
use rand::prelude::*;

use super::Vertex;

#[derive(Clone, Debug)]
pub struct Shape {
    pub n_vertices: usize,
    pub n_indices: usize,
    pub position_gen: Box<dyn GenPosition>,
    pub color_gen: Box<dyn GenColor>,
    pub indices_gen: Box<dyn GenIndex>,
}

#[derive(Copy, Clone, Debug)]
pub struct RandPosition;
#[derive(Copy, Clone, Debug)]
pub struct RandIndex;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GenIndex for RandIndex {
    #[allow(dead_code)]
    fn gen(&self, n_vertices: usize) -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..n_vertices as u16)
    }
}

impl GenPosition for RandPosition {
    fn gen(&self) -> Position {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Position {
            x: r(),
            y: r(),
            z: r(),
        }
    }
}

impl Shape {
    pub fn gen(&self) -> (Vec<Vertex>, Vec<Index>) {
        (
            (0..self.n_vertices)
                .into_iter()
                .map(|_| Vertex::from_shape(self))
                .collect(),
            (0..self.n_indices)
                .into_iter()
                .map(|_| self.indices_gen.gen(self.n_indices))
                .collect(),
        )
    }
}

impl Vertex {
    pub fn from_shape(shape: &Shape) -> Self {
        let position = shape.position_gen.gen();
        let color = shape.color_gen.gen();
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Self {
            position: [position.x, position.y, position.z],
            color: [
                color.r * color.shade,
                color.g * color.shade,
                color.b * color.shade,
            ],
            direction: [r(), r(), r()],
            velocity: r() * 0.4,
        }
    }
}
