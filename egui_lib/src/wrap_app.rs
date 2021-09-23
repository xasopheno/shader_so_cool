use std::sync::{Arc, Mutex};

/// All the different demo apps.
pub struct Apps {
    controls: crate::apps::Controls,
}

impl Apps {
    fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        Apps {
            controls: crate::apps::Controls::init(state.clone(), n_camera),
        }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut dyn epi::App)> {
        vec![("demo", &mut self.controls as &mut dyn epi::App)].into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct UiState {
    pub play: bool,
    pub volume: f32,
    pub camera_index: usize,
    pub instance_mul: InstanceMul,
    pub reset: bool,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct InstanceMul {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub life: f32,
    pub size: f32,
    pub length: f32,
}
impl Default for InstanceMul {
    fn default() -> Self {
        Self {
            x: 3.0,
            y: 8.0,
            z: 1.0,
            life: 2.0,
            size: 6.0,
            length: 1.0,
        }
    }
}

/// Wraps many demo/test apps into one.
pub struct WrapApp {
    selected_anchor: String,
    apps: Apps,
}

impl WrapApp {
    pub fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        WrapApp {
            selected_anchor: "".to_string(),
            apps: Apps::init(state, n_camera),
        }
    }
}

impl epi::App for WrapApp {
    fn name(&self) -> &str {
        "Kintaro"
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // we set a `CentralPanel` fill color in `demo_windows.rs`
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if let Some(web_info) = frame.info().web_info.as_ref() {
            if let Some(anchor) = web_info.web_location_hash.strip_prefix('#') {
                self.selected_anchor = anchor.to_owned();
            }
        }

        if self.selected_anchor.is_empty() {
            self.selected_anchor = self.apps.iter_mut().next().unwrap().0.to_owned();
        }

        for (anchor, app) in self.apps.iter_mut() {
            if anchor == self.selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
            }
        }
    }
}
