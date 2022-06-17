use std::fmt::Debug;

pub type Index = u16;

use crate::{
    color::Color,
    vertex::{shape::Position, Vertex},
};

pub trait GenColor: dyn_clone::DynClone + Debug {
    fn gen(&self, names: &Vec<String>) -> Color;
    fn update(&mut self);
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
