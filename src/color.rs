use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::vertex::{shape::Position, Vertex};

#[derive(Clone, Debug)]
pub struct RandColor;

pub type Index = u16;

#[derive(Clone, Debug)]
pub struct ColorSet {
    pub colors: Vec<Color>,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub shade: f32,
}

pub trait GenColor: dyn_clone::DynClone {
    fn gen(&self) -> Color;
}
pub trait GenPosition: dyn_clone::DynClone {
    fn gen(&self) -> Position;
}
pub trait GenVertex: dyn_clone::DynClone {
    fn gen(&self) -> Vec<Vertex>;
}
pub trait GenIndex: dyn_clone::DynClone {
    fn gen(&self, n_vertices: usize) -> Index;
}

dyn_clone::clone_trait_object!(GenColor);
dyn_clone::clone_trait_object!(GenPosition);
dyn_clone::clone_trait_object!(GenIndex);
dyn_clone::clone_trait_object!(GenVertex);

impl GenColor for ColorSet {
    fn gen(&self) -> Color {
        self.colors
            .choose(&mut rand::thread_rng())
            .expect("color choice failed")
            .to_owned()
    }
}

impl GenColor for RandColor {
    fn gen(&self) -> Color {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;

        Color {
            r: r(),
            g: r(),
            b: r(),
            shade: r(),
        }
    }
}
