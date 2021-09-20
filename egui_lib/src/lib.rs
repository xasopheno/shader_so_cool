//! Demo-code for showing how egui is used.
//!
//! The demo-code is also used in benchmarks and tests.

mod apps;
mod wrap_app;

pub use apps::Windows; // used for tests
pub use wrap_app::{InstanceMul, UiState, WrapApp};

#[test]
fn test_egui_e2e() {
    let mut demo_windows = crate::Windows::default();
    let mut ctx = egui::CtxRef::default();
    let raw_input = egui::RawInput::default();

    const NUM_FRAMES: usize = 5;
    for _ in 0..NUM_FRAMES {
        ctx.begin_frame(raw_input.clone());
        demo_windows.ui(&ctx);
        let (_output, shapes) = ctx.end_frame();
        let clipped_meshes = ctx.tessellate(shapes);
        assert!(!clipped_meshes.is_empty());
    }
}

#[test]
fn test_egui_zero_window_size() {
    let mut demo_windows = crate::Windows::default();
    let mut ctx = egui::CtxRef::default();
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO)),
        ..Default::default()
    };

    const NUM_FRAMES: usize = 5;
    for _ in 0..NUM_FRAMES {
        ctx.begin_frame(raw_input.clone());
        demo_windows.ui(&ctx);
        let (_output, shapes) = ctx.end_frame();
        let clipped_meshes = ctx.tessellate(shapes);
        assert!(clipped_meshes.is_empty(), "There should be nothing to show");
    }
}
