mod app;

use app::App;

fn main() -> eframe::Result<(), eframe::Error> {
    eframe::run_native(
        "egui-alist",
        eframe::NativeOptions {
            centered: true,
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([540.0, 960.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
}
