use bytemuck;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OrigamiUniforms {
    view_proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}
impl OrigamiUniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(
        &mut self,
        camera: &crate::camera::Camera,
        projection: &crate::camera::Projection,
    ) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
