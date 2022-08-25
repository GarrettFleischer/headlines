use eframe::{
    egui::{menu, Context, Layout, RichText, TextStyle, TopBottomPanel},
    emath::Align,
};

use super::{constants::PADDING, HeadlinesConfig};

pub fn render_top_panel(ctx: &Context, frame: &mut eframe::Frame, config: &mut HeadlinesConfig) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(PADDING);
        menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| ui.heading("📓"));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let close_btn = ui.button(RichText::new("❌").text_style(TextStyle::Body));
                let refresh_btn = ui.button(RichText::new("🔄").text_style(TextStyle::Body));
                let theme_btn = ui.button(RichText::new("🌓").text_style(TextStyle::Body));

                if close_btn.clicked() {
                    frame.close();
                }

                if theme_btn.clicked() {
                    config.dark_mode = !config.dark_mode;
                    config.save();
                }
            });
        });
        ui.add_space(PADDING);
    });
}
