// https://github.com/creativcoder/headlines/blob/ep9/headlines/src/headlines.rs
// https://github.com/emilk/egui/blob/master/examples/custom_window_frame/src/main.rs

mod app;

use app::App;

fn main() -> eframe::Result<(), eframe::Error> {
    eframe::run_native(
        "egui-alist",
        eframe::NativeOptions {
            centered: true,
            viewport: eframe::egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_inner_size([270.0, 480.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
}
