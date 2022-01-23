use thiserror::Error;

#[derive(Error, Debug)]
pub enum KintaroError {
    #[error("Anyhow Error")]
    AnyhowError(#[from] anyhow::Error),
    #[error("Image Error")]
    ImageError(#[from] image::ImageError),
    #[error("Wgpu Error")]
    WgpuSurfaceError(#[from] wgpu::SurfaceError),
    #[error("WereSoCool Error")]
    WereSoCool(#[from] weresocool::error::Error),
    #[error("IO Error")]
    Disconnect(#[from] std::io::Error),
    #[error("`{0}`")]
    Message(String),
}
