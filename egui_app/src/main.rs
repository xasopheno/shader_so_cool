fn main() {
    let app = egui_demo_lib::WrapApp::default();
    let options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
