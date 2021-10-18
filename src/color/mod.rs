pub mod helpers;
use std::fmt::Debug;

use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::gen::GenColor;

#[derive(Clone, Debug)]
pub struct RandColor;

#[derive(Clone, Debug)]
pub struct ColorSet {
    pub colors: Vec<Color>,
}

#[derive(Clone, Debug)]
pub struct ColorSets {
    n: usize,
    colorsets: Vec<ColorSet>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub shade: f32,
}

impl GenColor for ColorSets {
    fn gen(&self) -> Color {
        self.colorsets[self.n]
            .colors
            .choose(&mut rand::thread_rng())
            .expect("color choice failed")
            .to_owned()
    }
    fn update(&mut self) {
        self.n = (self.n + 1) % self.colorsets.len();
    }
}

impl GenColor for ColorSet {
    fn gen(&self) -> Color {
        self.colors
            .choose(&mut rand::thread_rng())
            .expect("color choice failed")
            .to_owned()
    }
    fn update(&mut self) {}
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
    fn update(&mut self) {}
}
