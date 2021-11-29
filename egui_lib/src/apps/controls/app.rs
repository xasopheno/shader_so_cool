use epi::App;
use std::sync::{Arc, Mutex};

use crate::UiState;

pub struct Controls {
    windows: super::Windows,
}

impl Controls {
    pub fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        Self {
            windows: super::Windows::init(state, n_camera),
        }
    }
}

impl App for Controls {
    fn name(&self) -> &str {
        "Kintaro"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.windows.ui(ctx);
    }
}
