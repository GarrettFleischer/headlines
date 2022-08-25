use eframe::{
    egui::{Layout, RichText, Separator, TextStyle, Ui},
    emath::Align,
};

use super::{constants::PADDING, theme::Theme};

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

    pub fn render(&self, theme: &Theme, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(theme.title_color, format!("▶ {}", self.title));
        ui.add_space(PADDING);
        ui.label(RichText::new(&self.description).text_style(TextStyle::Button));

        ui.add_space(PADDING);
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.hyperlink_to("read more ⤴", &self.url)
        });

        ui.add_space(PADDING);
        ui.add(Separator::default());
    }
}
