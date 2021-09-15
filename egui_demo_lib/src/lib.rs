//! Demo-code for showing how egui is used.
//!
//! The demo-code is also used in benchmarks and tests.

mod apps;
mod backend_panel;
pub(crate) mod frame_history;
mod wrap_app;

pub use apps::DemoWindows; // used for tests
pub use wrap_app::WrapApp;

// ----------------------------------------------------------------------------

/// Create a [`Hyperlink`](crate::Hyperlink) to this egui source code file on github.
#[doc(hidden)]
#[macro_export]
macro_rules! __egui_github_link_file {
    () => {
        crate::__egui_github_link_file!("(source code)")
    };
    ($label:expr) => {
        egui::github_link_file!("https://github.com/emilk/egui/blob/master/", $label).small()
    };
}

/// Create a [`Hyperlink`](crate::Hyperlink) to this egui source code file and line on github.
#[doc(hidden)]
#[macro_export]
macro_rules! __egui_github_link_file_line {
    () => {
        crate::__egui_github_link_file_line!("(source code)")
    };
    ($label:expr) => {
        egui::github_link_file_line!("https://github.com/emilk/egui/blob/master/", $label).small()
    };
}

// ----------------------------------------------------------------------------

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris. Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.";

// ----------------------------------------------------------------------------

#[test]
fn test_egui_e2e() {
    let mut demo_windows = crate::DemoWindows::default();
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
    let mut demo_windows = crate::DemoWindows::default();
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
