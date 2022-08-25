use eframe::{
    egui::{Context, FontData, FontDefinitions, TextStyle, Visuals},
    epaint::{Color32, FontFamily, FontId},
};

use super::HeadlinesConfig;

pub struct Theme {
    pub title_color: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            title_color: Color32::WHITE,
        }
    }
}

impl Theme {
    pub fn update(&mut self, ctx: &Context, config: &HeadlinesConfig) {
        let mut visuals = if config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        visuals.hyperlink_color = if config.dark_mode {
            Color32::LIGHT_BLUE
        } else {
            Color32::RED
        };

        ctx.set_visuals(visuals);

        self.title_color = if config.dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
    }
}

pub fn configure_fonts(ctx: &Context) {
    use FontFamily::Proportional;

    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes! {"../../../MesloLGS_NF_Regular.ttf"}),
    );

    ctx.set_fonts(font_def);

    let mut style = (*ctx.style()).clone();

    style
        .text_styles
        .insert(TextStyle::Heading, FontId::new(30., Proportional));
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(20., Proportional));

    ctx.set_style(style);
}
