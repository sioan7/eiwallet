mod app;

fn main() {
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::Vec2::new(1200f32, 800f32)),
        ..Default::default()
    };
    eframe::run_native(
        "eiwallet",
        native_options,
        Box::new(|cc| Box::new(app::EiwalletApp::new(cc))),
    );
}
