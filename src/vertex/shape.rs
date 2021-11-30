use crate::{
    gen::{GenColor, GenIndex, GenPosition, Index},
    op_stream::{self, OpStream},
};
use rand::prelude::*;

use super::Vertex;

pub struct ShapeGenResult {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub n_vertices: usize,
    pub n_indices: usize,
    pub position: Box<dyn GenPosition>,
    pub color: Box<dyn GenColor>,
    pub indices: Box<dyn GenIndex>,
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
    fn gen(&self, n_vertices: usize) -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..n_vertices as u16)
    }
}

impl GenPosition for RandPosition {
    fn gen(&self) -> Position {
        let mut rng = rand::thread_rng();
        // let mut xrng = rand::thread_rng();
        // let mut yrng = rand::thread_rng();
        // let mut zrng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        // let mut yr = || yrng.gen::<f32>() * 2.0 - 1.0;
        // let mut zr = || zrng.gen::<f32>() * 2.0 - 1.0;
        // let mut zr = || zrng.gen::<f32>();
        // let mut z = zr();
        // let z = if z < 0.5 { z * -10.0 } else { z };
        Position {
            x: r(),
            y: r(),
            z: r(),
        }
    }
}

impl Shape {
    pub fn gen(&mut self, op_stream: &OpStream) -> ShapeGenResult {
        ShapeGenResult {
            vertices: (0..self.n_vertices)
                .into_iter()
                .map(|_| Vertex::from_shape(self, op_stream))
                .collect(),
            indices: (0..self.n_indices)
                .into_iter()
                .map(|_| self.indices.gen(self.n_indices))
                .collect(),
        }
    }

    pub fn update(&mut self) {
        self.color.update()
    }

    pub fn with_color(&self, color: Box<dyn GenColor>) -> Self {
        let mut clone = self.clone();
        clone.color = color;
        clone
    }
}

impl Vertex {
    pub fn from_shape(shape: &mut Shape, op_stream: &OpStream) -> Self {
        let position = shape.position.gen();
        let color = shape.color.gen(op_stream);
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
