/// All the different demo apps.
#[derive(Default)]
pub struct Apps {
    demo: crate::apps::Controls,
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
}

impl epi::App for WrapApp {
    fn name(&self) -> &str {
        "Kintaro"
    }

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

        for (anchor, app) in self.apps.iter_mut() {
            if anchor == self.selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
            }
        }
    }
}
