use std::sync::{Arc, Mutex};

use crate::{InstanceMul, UiState};

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
        // ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(235, 72, 170));
        ui.visuals_mut().override_text_color = Some(egui::Color32::GOLD);
        ui.style_mut().body_text_style = egui::TextStyle::Heading;
        let Self { state, n_camera } = self;
        let mut s = state.lock().unwrap();
        let mut volume = s.volume;
        // let mut instance_mul_x = s.instance_mul.x;
        // let mut instance_mul_y = s.instance_mul.y;
        // let mut instance_mul_size = s.instance_mul.size;
        let InstanceMul {
            mut x,
            mut y,
            mut size,
            ..
        } = s.instance_mul;
        let frames = s.frames;

        let now = std::time::Instant::now();
        let elapsed = now.duration_since(s.time).as_secs();
        s.frames += 1;

        let fps = frames / (elapsed + 1);

        ui.vertical(|ui| {
            // ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.horizontal(|ui| {
                if ui.button(if s.play { "Pause" } else { "Play" }).clicked() {
                    s.play = !s.play
                }
                if ui.button("Save").clicked() {
                    s.save = true
                }
                ui.label(format!("fps: {}", fps.to_string()));
                ui.end_row();
                // if ui.button("Reset").clicked() {
                // s.reset = true
                // }
                // ui.end_row();
            });
            ui.horizontal_wrapped(|ui| {
                // ui.label("Volume:");
                if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                    s.volume = volume
                };
                ui.end_row();

                // ui.label("Camera:");
                // (0..*n_camera).into_iter().for_each(|idx| {
                // if ui
                // .button(format!("  {}  ", (idx + 65) as u8 as char))
                // .clicked()
                // {
                // s.camera_index = idx as usize;
                // }
                // });
                ui.end_row();
            });
            ui.vertical(|ui| {
                ui.label("x:");
                if ui.add(egui::Slider::new(&mut x, 0.0..=2000.0)).changed() {
                    s.instance_mul.x = x
                };
                ui.end_row();
                ui.label("y:");
                if ui.add(egui::Slider::new(&mut y, 0.0..=2000.0)).changed() {
                    s.instance_mul.y = y
                };
                // ui.label("z:");
                // if ui.add(egui::Slider::new(&mut z, 0.0..=25.0)).changed() {
                // s.instance_mul.z = z
                // };
                ui.end_row();
                ui.label("size:");
                if ui.add(egui::Slider::new(&mut size, 0.0..=2000.0)).changed() {
                    s.instance_mul.size = size
                };
                // ui.label("life:");
                // if ui.add(egui::Slider::new(&mut life, 0.0..=10.0)).changed() {
                // s.instance_mul.life = life
                // };
                // ui.label("length:");
                // if ui.add(egui::Slider::new(&mut length, 0.0..=25.0)).changed() {
                // s.instance_mul.length = length
                // };
                // ui.end_row();
            });
        });
    }
}
