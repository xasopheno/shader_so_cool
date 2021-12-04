pub mod helpers;
// use std::collections::BTreeMap;
use indexmap::IndexMap;
use std::fmt::Debug;

use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::colorsets_from_vec_hex_strings;
use crate::gen::GenColor;
use crate::op_stream::OpStream;

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

#[derive(Clone, Debug)]
pub struct ColorMap {
    pub colors: IndexMap<String, Box<dyn GenColor>>,
    pub default: Box<dyn GenColor>,
}

impl GenColor for ColorMap {
    fn gen(&self, op_stream: &OpStream) -> Color {
        for (name, color) in self.colors.iter() {
            dbg!(&name, color);
            if op_stream.names.contains(name) {
                return color.gen(op_stream);
            }
        }
        self.default.gen(op_stream)
    }
    fn update(&mut self) {}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub shade: f32,
}

impl GenColor for ColorSets {
    fn gen(&self, op_stream: &OpStream) -> Color {
        self.colorsets[self.n].gen(op_stream)
    }
    fn update(&mut self) {
        self.n = (self.n + 1) % self.colorsets.len();
    }
}

impl GenColor for ColorSet {
    fn gen(&self, _op_stream: &OpStream) -> Color {
        self.colors
            .choose(&mut rand::thread_rng())
            .expect("color choice failed")
            .to_owned()
    }
    fn update(&mut self) {}
}

impl GenColor for RandColor {
    fn gen(&self, _op_stream: &OpStream) -> Color {
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

impl Default for ColorSets {
    fn default() -> Self {
        colorsets_from_vec_hex_strings(vec![
            vec!["#6655aa", "#222222"],
            vec!["#eeaC88", "#121312", "#333333"],
            vec![
                "#213CFB", "#310CFA", "#6688aa", "#111111", "#121212", "#101010",
            ],
            vec!["#660000", "#100101", "#300002"],
            vec!["#473859", "#222222"],
            vec!["#300300", "#333333"],
            vec!["#001931", "#000000", "#222200"],
            vec!["#a000a0", "#000000", "#2303aa", "#333333"],
            vec!["#473859", "#222222"],
            vec!["#348348", "#112312"],
            vec!["#0000ee", "#0e000e"],
            //
            vec!["#333333", "#111111", "#777777"],
            vec!["#660000", "#100101", "#300002", "#100001", "#010210"],
            vec!["#473850", "#222222", "#001001"],
            vec!["#112112", "#000033"],
            vec!["#ff00ff", "#000000"],
            vec!["#38881a", "#333333"],
            vec!["#aa10e4", "#333333"],
        ])
    }
}
