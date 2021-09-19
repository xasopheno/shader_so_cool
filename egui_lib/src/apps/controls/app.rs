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

impl epi::App for Controls {
    fn name(&self) -> &str {
        "Kintaro"
    }

    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.windows.ui(ctx);
    }
}
