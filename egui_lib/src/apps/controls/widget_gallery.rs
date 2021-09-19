use std::sync::{Arc, Mutex};

use crate::UiState;

#[derive(Debug)]
pub struct ControlsInner {
    state: Arc<Mutex<UiState>>,
    n_camera: usize,
}

impl ControlsInner {
    pub fn init(state: Arc<Mutex<UiState>>, n_camera: usize) -> Self {
        Self {
            state: state.clone(),
            n_camera,
        }
    }
}

impl super::Module for ControlsInner {
    fn name(&self) -> &'static str {
        "Kintaro"
    }

    fn show(&mut self, ctx: &egui::CtxRef, _open: &mut bool) {
        egui::Window::new(self.name())
            // .open(open)
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                use super::View;
                self.ui(ui);
            });
    }
}

impl super::View for ControlsInner {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            egui::Grid::new("Kintaro")
                .num_columns(1)
                .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });
    }
}

impl ControlsInner {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(235, 72, 170));
        ui.style_mut().body_text_style = egui::TextStyle::Heading;
        let Self { state, n_camera } = self;
        let mut s = state.lock().unwrap();
        let mut volume = s.volume;

        ui.horizontal_wrapped(|ui| {
            ui.label("Volume:");
            if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                s.volume = volume
            };
            ui.end_row();

            ui.label("Camera:");
            (0..*n_camera).into_iter().for_each(|idx| {
                if ui
                    .button(format!("  {}  ", (idx + 65) as u8 as char))
                    .clicked()
                {
                    s.camera_index = idx as usize;
                }
            });
            ui.end_row();
        });
    }
}
