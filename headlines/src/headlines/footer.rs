use eframe::egui::{Context, RichText, TopBottomPanel};

pub fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
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
        ui.add_space(10.);
    });
}
