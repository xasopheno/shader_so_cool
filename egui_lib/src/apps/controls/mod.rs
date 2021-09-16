mod app;
pub mod widget_gallery;
pub mod windows;

pub use {app::Controls, widget_gallery::ControlsInner, windows::Windows};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Module {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}
