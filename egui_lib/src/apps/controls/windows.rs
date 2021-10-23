use crate::UiState;

use super::Module;
use egui::CtxRef;
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

struct Kintaro {
    #[cfg_attr(feature = "persistence", serde(skip))]
    modules: Vec<Box<dyn Module>>,
    open: BTreeSet<String>,
}

impl Kintaro {
    pub fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        Self::from_modules(
            state.clone(),
            vec![Box::new(super::widget_gallery::ControlsInner::init(
                state, n_camera,
            ))],
            n_camera,
        )
    }
}

impl Kintaro {
    pub fn from_modules(
        state: Arc<Mutex<UiState>>,
        modules: Vec<Box<dyn Module>>,
        n_camera: usize,
    ) -> Self {
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::ControlsInner::init(state.clone(), n_camera)
                .name()
                .to_owned(),
        );

        Self { modules, open }
    }

    pub fn windows(&mut self, ctx: &CtxRef) {
        let Self { modules, open } = self;
        for module in modules {
            let mut is_open = open.contains(module.name());
            module.show(ctx, &mut is_open);
            set_open(open, module.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

pub struct Windows {
    kintaro: Kintaro,
}

impl Windows {
    pub fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        Self {
            kintaro: Kintaro::init(state, n_camera),
        }
    }

    /// Show the app ui (menu bar and windows).
    /// `sidebar_ui` can be used to optionally show some things in the sidebar
    pub fn ui(&mut self, ctx: &CtxRef) {
        let Self { kintaro: _ } = self;

        {
            let mut fill = ctx.style().visuals.extreme_bg_color;
            if !cfg!(target_arch = "wasm32") {
                // Native: WrapApp uses a transparent window, so let's show that off:
                // NOTE: the OS compositor assumes "normal" blending, so we need to hack it:
                let [r, g, b, _] = fill.to_array();
                fill = egui::Color32::from_rgba_premultiplied(r, g, b, 0);
            }
            let frame = egui::Frame::none().fill(fill);
            egui::CentralPanel::default().frame(frame).show(ctx, |_| {});
        }

        self.windows(ctx);
    }

    /// Show the open windows.
    fn windows(&mut self, ctx: &CtxRef) {
        let Self { kintaro } = self;

        kintaro.windows(ctx);
    }
}
