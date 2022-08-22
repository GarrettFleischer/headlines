use eframe::egui::{
    CentralPanel, Context, FontData, FontDefinitions, Layout, RichText, ScrollArea, Separator,
    TextStyle, Ui, Visuals,
};
use eframe::emath::Align;
use eframe::epaint::{Color32, FontFamily, FontId};
use eframe::{App, CreationContext};

pub mod constants;
mod footer;
mod header;
mod top_panel;

use constants::PADDING;
use footer::render_footer;
use header::render_header;
use top_panel::render_top_panel;

pub struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl NewsCardData {
    pub fn new(title: String, description: String, url: String) -> Self {
        NewsCardData {
            title,
            description,
            url,
        }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new(cc: &CreationContext, articles: Vec<NewsCardData>) -> Self {
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

        style
            .text_styles
            .insert(TextStyle::Heading, FontId::new(30., Proportional));
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(20., Proportional));

        // style.text_styles = [
        //     (TextStyle::Heading, FontId::new(30., Proportional)),
        //     (TextStyle::Body, FontId::new(20., Proportional)),
        // ]
        // .into();
        ctx.set_style(style);
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            ui.colored_label(Color32::WHITE, format!("▶ {}", a.title));
            ui.add_space(PADDING);
            ui.label(RichText::new(&a.description).text_style(TextStyle::Button));

            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = Color32::LIGHT_BLUE;
            ui.with_layout(Layout::right_to_left().with_cross_align(Align::Min), |ui| {
                ui.hyperlink_to("read more ⤴", &a.url)
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
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
        render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical().show(ui, |ui| self.render_news_cards(ui));
        });
        render_footer(ctx);
    }
}
