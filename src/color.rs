use rand::prelude::*;
use rand::seq::SliceRandom;

pub type ColorSet = Vec<Color>;
pub struct RandColor;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub shade: f32,
}

pub trait Gen<T>
where
    T: Sized + Clone,
{
    fn gen(&self, idx: usize) -> T;
}

impl Gen<Color> for ColorSet {
    fn gen(&self, _idx: usize) -> Color {
        *self
            .choose(&mut rand::thread_rng())
            .expect("color choice failed")
    }
}

impl Gen<Color> for RandColor {
    fn gen(&self, _idx: usize) -> Color {
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
