use std::fmt::Debug;

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

pub trait GenColor: dyn_clone::DynClone + Debug {
    fn gen(&self) -> Color;
}
pub trait GenPosition: dyn_clone::DynClone + Debug {
    fn gen(&self) -> Position;
}
pub trait GenVertex: dyn_clone::DynClone + Debug {
    fn gen(&self) -> Vec<Vertex>;
}
pub trait GenIndex: dyn_clone::DynClone + Debug {
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

fn new() {
    let colorset = ColorSet {
        colors: vec![
            Color {
                r: 179.0,
                g: 118.0,
                b: 71.0,
                shade: 70.0,
            },
            Color {
                r: 154.0,
                g: 255.0,
                b: 153.0,
                shade: 100.0,
            },
            Color {
                r: 255.0,
                g: 183.0,
                b: 128.0,
                shade: 100.0,
            },
            Color {
                r: 129.0,
                g: 102.0,
                b: 255.0,
                shade: 100.0,
            },
            Color {
                r: 97.0,
                g: 80.0,
                b: 179.0,
                shade: 70.0,
            },
            Color {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                shade: 70.0,
            },
        ],
    };
    let colors = colorset
        .colors
        .iter()
        .map(|c| Color {
            r: c.r / 255.0,
            g: c.g / 255.0,
            b: c.b / 255.0,
            shade: c.shade / 100.0,
        })
        .collect();
    let colorset = ColorSet { colors };
}
