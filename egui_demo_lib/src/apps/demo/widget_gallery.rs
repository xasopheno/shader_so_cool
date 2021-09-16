#[derive(Debug, PartialEq)]
/// Shows off one example of each major type of widget.
pub struct WidgetGallery {
    boolean: bool,
}

impl Default for WidgetGallery {
    fn default() -> Self {
        Self { boolean: false }
    }
}

impl super::Demo for WidgetGallery {
    fn name(&self) -> &'static str {
        "Kintaro"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                use super::View;
                self.ui(ui);
            });
    }
}

impl super::View for WidgetGallery {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            egui::Grid::new("Kintaro")
                .num_columns(2)
                .spacing([20.0, 2.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });
    }
}

impl WidgetGallery {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { boolean, .. } = self;

        ui.label("Camera");
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
