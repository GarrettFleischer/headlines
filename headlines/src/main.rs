use eframe::egui::{
    CentralPanel, Context, FontData, FontDefinitions, Layout, ScrollArea, TextStyle, Ui, Visuals,
};
use eframe::emath::Align;
use eframe::epaint::{FontFamily, FontId, Pos2, Vec2};
use eframe::{run_native, App, CreationContext, NativeOptions};

struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

fn main() {
    let articles = Vec::from_iter({ 0..20 }.map(|i| NewsCardData {
        title: format!("Title: {}", i),
        description: format!("Description: {}", i),
        url: format!("https://example.com/{}", i),
    }));

    let mut native_options = NativeOptions::default();
    native_options.initial_window_pos = Some(Pos2::new(600., 10.));
    native_options.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(
        "Headlines",
        native_options,
        Box::new(|cc| Box::new(Headlines::new(cc, articles))),
    );
}

impl Headlines {
    fn new(cc: &CreationContext, articles: Vec<NewsCardData>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Headlines::configure_fonts(&cc.egui_ctx);
        Headlines { articles }
    }
    fn configure_fonts(ctx: &Context) {
        use FontFamily::Proportional;

        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes! {"../../MesloLGS_NF_Regular.ttf"}),
        );

        ctx.set_fonts(font_def);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(30., Proportional)),
            (TextStyle::Body, FontId::new(20., Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.label(format!("{}", a.title));
            ui.label(format!("{}", a.url));
            ui.label(format!("{}", a.description));
        }
    }

    // fn render_news_cards<'a>(&'a self) -> impl Fn(&mut Ui) -> () + 'a {
    //     move |ui| {
    //         for a in &self.articles {
    //             ui.label(format!("{}", a.title));
    //             ui.label(format!("{}", a.url));
    //             ui.label(format!("{}", a.description));
    //         }
    //     }
    // }
}

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ScrollArea::vertical().show(ui, |ui| self.render_news_cards(ui));
            })
        });
    }
}
