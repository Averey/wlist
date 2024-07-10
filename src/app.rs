use eframe::egui::{
    self, Align, Button, Label, Color32, Layout, RichText, TextStyle, TopBottomPanel,
};

struct CardData {
    title: String,
    desc: String,
    url: String,
}

pub struct App {
    items: Vec<CardData>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Self {
        //    items: vec![],
        //}
        let iter = (0..20).map(|i| CardData {
            title: format!("title{}", i),
            desc: format!("desc{i}"),
            url: "https://www.baidu.com".to_string(),
        });
        Self {
            items: Vec::from_iter(iter),
        }
    }

    fn render_top_panel(&self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add(Label::new(
                        RichText::new("📓").text_style(TextStyle::Heading),
                    ));
                });
                // controls
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let close_btn = ui.add(Button::new(
                        RichText::new("❌").text_style(TextStyle::Body),
                    ));
                    if close_btn.clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button("🔄").clicked() {

                    }
                    if ui.button("🌙").clicked() {

                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn render_cards(&self, ui: &mut egui::Ui) {
        self.items.iter().for_each(|i| {
            // title
            ui.add_space(5.0);
            ui.colored_label(
                Color32::from_rgb(255, 255, 255), 
                format!("▶ {}", i.title),
            );

            // desc
            ui.add_space(5.0);
            ui.add(
                Label::new(&i.desc)
            );

            // hyperlinks
            ui.style_mut().visuals.hyperlink_color = Color32::from_rgb(0, 255, 255);
            ui.add_space(5.0);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.hyperlink_to("read more ⤴", &i.url);
            });
            ui.add_space(5.0);
            ui.separator();

        });
    }

}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_cards(ui);
            });
        });
        egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.label("Bottom");
        });
    }
}

