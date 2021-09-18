use std::sync::{Arc, Mutex};

use egui::Color32;

use crate::UiState;

#[derive(Debug)]
pub struct ControlsInner {
    boolean: bool,
    state: Arc<Mutex<UiState>>,
}

impl ControlsInner {
    pub fn init(state: Arc<Mutex<UiState>>) -> Self {
        Self {
            boolean: false,
            state: state.clone(),
        }
    }
}

impl super::Module for ControlsInner {
    fn name(&self) -> &'static str {
        "Kintaro"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
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
                // .spacing([10.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });
    }
}

impl ControlsInner {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        ui.visuals_mut().override_text_color = Some(egui::Color32::GOLD);
        let Self { boolean, state } = self;
        let mut s = state.lock().unwrap();
        let mut volume = s.volume;

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            // ui.colored_label(Color32::GOLD, "Volume");
            if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                s.volume = volume
            };
            ui.end_row();

            // ui.colored_label(Color32::GOLD, "Camera");
            if ui.button("Camera A").clicked() {
                s.camera_index = 0;
            }
            if ui.button("Camera B").clicked() {
                s.camera_index = 1;
            }
            if ui.button("Camera C").clicked() {
                s.camera_index = 2;
            }
            if ui.button("Camera D").clicked() {
                s.camera_index = 3;
            }
            ui.end_row();
        });
    }
}
