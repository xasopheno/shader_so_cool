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
            .open(open)
            .resizable(false)
            .default_width(200.0)
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
                .num_columns(2)
                .spacing([20.0, 1.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });
    }
}

impl ControlsInner {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { boolean, .. } = self;

        ui.colored_label(Color32::GOLD, "Camera");
        if ui.button("A").clicked() {
            *boolean = !*boolean;
        }
        if ui.button("B").clicked() {
            *boolean = !*boolean;
        }
        if ui.button("C").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();
    }
}
