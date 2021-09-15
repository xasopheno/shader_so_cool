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
        "WereSoCool: Kintaro"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                use super::View;
                self.ui(ui);
            });
    }
}

impl super::View for WidgetGallery {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();
    }
}

impl WidgetGallery {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { boolean, .. } = self;

        ui.add(doc_link_label("Camera", "button"));
        if ui.button("Camera1").clicked() {
            *boolean = !*boolean;
        }
        // ui.add(doc_link_label("CameraButton2", "button"));
        if ui.button("Camera2").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();
    }
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    let label = format!("{}:", title);
    let url = format!("https://docs.rs/egui?search={}", search_term);
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(label, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}
