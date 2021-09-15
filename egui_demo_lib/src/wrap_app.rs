/// All the different demo apps.
#[derive(Default)]
pub struct Apps {
    demo: crate::apps::DemoApp,
}

impl Apps {
    fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut dyn epi::App)> {
        vec![("demo", &mut self.demo as &mut dyn epi::App)].into_iter()
    }
}

/// Wraps many demo/test apps into one.
#[derive(Default)]
pub struct WrapApp {
    selected_anchor: String,
    apps: Apps,
    // backend_panel: super::backend_panel::BackendPanel,
}

impl epi::App for WrapApp {
    fn name(&self) -> &str {
        "Kintaro"
    }

    // #[cfg(feature = "persistence")]
    // fn save(&mut self, storage: &mut dyn epi::Storage) {
    // epi::set_value(storage, epi::APP_KEY, self);
    // }

    // fn max_size_points(&self) -> egui::Vec2 {
    // self.backend_panel.max_size_points_active
    // }

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

        // egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
        // egui::trace!(ui);
        // self.bar_contents(ui, frame);
        // });

        // self.backend_panel.update(ctx, frame);

        // if self.backend_panel.open || ctx.memory().everything_is_visible() {
        // egui::SidePanel::left("backend_panel").show(ctx, |ui| {
        // self.backend_panel.ui(ui, frame);
        // });
        // }

        for (anchor, app) in self.apps.iter_mut() {
            if anchor == self.selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
            }
        }

        // self.backend_panel.end_of_frame(ctx);
    }
}

impl WrapApp {
    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut epi::Frame<'_>) {
        // A menu-bar is a horizontal layout with some special styles applied.
        // egui::menu::bar(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            // ui.checkbox(&mut self.backend_panel.open, "💻 Backend");
            // ui.separator();

            for (anchor, app) in self.apps.iter_mut() {
                if ui
                    .selectable_label(self.selected_anchor == anchor, app.name())
                    .clicked()
                {
                    self.selected_anchor = anchor.to_owned();
                    if frame.is_web() {
                        ui.output().open_url(format!("#{}", anchor));
                    }
                }
            }
        });
    }
}
