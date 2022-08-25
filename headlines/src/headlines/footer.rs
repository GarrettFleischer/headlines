use eframe::egui::{Context, RichText, TopBottomPanel};

use super::constants::PANEL_SPACE;

pub fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(PANEL_SPACE);
            ui.label(RichText::new("API source: newsapi.org").monospace());
            ui.hyperlink_to(
                RichText::new("Made with egui").monospace(),
                "https://github.com/emilk/egui",
            );
            ui.hyperlink_to(
                RichText::new("GarrettFleischer/Headlines").monospace(),
                "https://github.com/GarrettFleischer/headlines",
            );
        });
        ui.add_space(PANEL_SPACE);
    });
}
