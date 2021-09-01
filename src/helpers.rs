use crate::vertex::Vertex;
use rand::Rng;

#[allow(dead_code)]
fn random_color() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn new_clear_color() -> (f64, f64, f64) {
    (0.7, 0.3, 0.6)
}

#[allow(dead_code)]
pub fn new_random_clear_color() -> (f64, f64, f64) {
    (random_color(), random_color(), random_color())
}

#[allow(dead_code)]
pub fn new_random_vertices() -> Vec<Vertex> {
    (0..50).into_iter().map(|_| Vertex::new_random()).collect()
}

#[allow(dead_code)]
pub fn new_random_indices(n: u16) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut num = || rng.gen_range(0..n);

    (0..50).map(|_| num()).collect()
}

#[allow(dead_code)]
pub fn new_shape_vertices() -> Vec<Vertex> {
    let size = 1.0;
    vec![
        Vertex::new(size, size, 0.0),
        Vertex::new(-size, size, 0.0),
        Vertex::new(-size, -size, 0.0),
        Vertex::new(size, -size, 0.0),
    ]
}

#[allow(dead_code)]
pub fn new_shape_indices(_n: u16) -> Vec<u16> {
    vec![0, 1, 2, 0, 2, 3]
}
