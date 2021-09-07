pub struct Canvas {
    pub ratio: f32,
    pub n_pixels: f32,
    pub n_row: u32,
    pub n_column: u32,
    pub instance_displacement: cgmath::Vector3<f32>,
}

impl Canvas {
    pub fn init(size: (u32, u32)) -> Canvas {
        let ratio = size.0 as f32 / size.1 as f32;
        let n_pixels = 20.0;
        let n_row = (n_pixels * ratio) as u32;
        let n_column = n_pixels as u32;
        let instance_displacement: cgmath::Vector3<f32> =
            cgmath::Vector3::new(0.0, n_column as f32, n_pixels);

        Canvas {
            ratio,
            n_pixels,
            n_row,
            n_column,
            instance_displacement,
        }
    }
}
