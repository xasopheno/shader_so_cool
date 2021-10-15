use crate::color::{Color, GenColor, GenIndex, GenPosition, GenVertex};
use rand::prelude::*;

use super::Vertex;

#[derive(Clone)]
pub struct Shape {
    pub n_vertices: usize,
    pub position_gen: Box<dyn GenPosition>,
    pub color_gen: Box<dyn GenColor>,
    pub indices_gen: Box<dyn GenIndex>,
}

pub type Index = i16;
pub type Indices = Vec<i16>;
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

// impl Gen<Indices> for RandIndices {
// fn gen(&self) -> Indices {
// let mut rng = rand::thread_rng();
// let mut num = || rng.gen_range(0..n);

// (0..30).map(|_| num()).collect()
// }
// }
//
impl GenIndex for RandIndex {
    #[allow(dead_code)]
    fn gen(&self, n_vertices: usize) -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..n_vertices as u16)
    }
}

#[allow(dead_code)]
pub fn new_random_indices(n: u16) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut num = || rng.gen_range(0..n);

    (0..30).map(|_| num()).collect()
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

impl GenVertex for Shape {
    fn gen(&self) -> Vec<Vertex> {
        (0..self.n_vertices)
            .into_iter()
            .map(|_| Vertex::from_shape(self))
            .collect()
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
