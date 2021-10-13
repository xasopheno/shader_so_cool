use std::rc::Rc;

use crate::color::{Color, Gen};
use rand::prelude::*;

use super::Vertex;

#[derive(Clone)]
pub struct Shape {
    pub n_vertices: usize,
    pub position_gen: Rc<dyn Gen<Position>>,
    pub color_gen: Rc<dyn Gen<Color>>,
}

pub struct RandPosition;
pub struct RandIndex;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
pub fn new_random_indices(n: u16) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut num = || rng.gen_range(0..n);

    (0..30).map(|_| num()).collect()
}

impl Gen<Position> for RandPosition {
    fn gen(&self, _idx: usize) -> Position {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Position {
            x: r(),
            y: r(),
            z: r(),
        }
    }
}

impl Gen<Vec<Vertex>> for Shape {
    fn gen(&self, _index: usize) -> Vec<Vertex> {
        (0..self.n_vertices)
            .into_iter()
            .map(|idx| Vertex::from_shape(self, idx))
            .collect()
    }
}

impl Vertex {
    pub fn from_shape(shape: &Shape, idx: usize) -> Self {
        let position = shape.position_gen.gen(idx);
        let color = shape.color_gen.gen(idx);
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
