use eframe::egui::{menu, Context, Layout, RichText, TextStyle, TopBottomPanel};

pub fn render_top_panel(ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(10.);
        menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(), |ui| ui.heading("📓"));
            ui.with_layout(Layout::right_to_left(), |ui| {
                let close_btn = ui.button(RichText::new("❌").text_style(TextStyle::Body));
                let refresh_btn = ui.button(RichText::new("🔄").text_style(TextStyle::Body));
                let theme_btn = ui.button(RichText::new("🌓").text_style(TextStyle::Body));
            });
        });
        ui.add_space(10.);
    });
}
