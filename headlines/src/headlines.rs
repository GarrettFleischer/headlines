use eframe::egui::{CentralPanel, ScrollArea, Ui};
use eframe::{App, CreationContext};

pub mod constants;
mod footer;
mod header;
mod news_card;
mod theme;
mod top_panel;

use footer::render_footer;
use header::render_header;
use news_card::NewsCardData;
use serde::{Deserialize, Serialize};
use theme::configure_fonts;
use theme::Theme;
use top_panel::render_top_panel;

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    dark_mode: bool,
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

impl HeadlinesConfig {
    pub fn save(&self) {
        confy::store("headlines", self).unwrap();
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    config: HeadlinesConfig,
    theme: Theme,
}

impl Headlines {
    pub fn new(cc: &CreationContext) -> Self {
        configure_fonts(&cc.egui_ctx);

        let articles = Vec::from_iter({ 0..20 }.map(|i| {
            NewsCardData::new(
                format!("Title: {}", i),
                format!("Description: {}", i),
                format!("https://example.com/{}", i),
            )
        }));
        let config = confy::load("headlines").unwrap_or_default();

        Headlines {
            articles,
            config,
            theme: Theme::default(),
        }
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            a.render(&self.theme, ui);
        }
    }
}

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // frame.set_decorations(false);
        self.theme.update(ctx, &self.config);

        render_top_panel(ctx, frame, &mut self.config);
        render_footer(ctx);

        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical().show(ui, |ui| self.render_news_cards(ui));
        });
    }
}
