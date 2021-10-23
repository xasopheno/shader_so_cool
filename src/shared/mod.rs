pub mod helpers;
mod render_pass;
mod render_pipeline;
mod update;

pub use helpers::{
    make_color_attachments, new_random_clear_color, new_random_indices, new_random_vertices,
};
pub use render_pass::{render_pass, RenderPassInput};
pub use render_pipeline::create_render_pipeline;
pub use update::update;
