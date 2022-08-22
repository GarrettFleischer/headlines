use eframe::egui::{Separator, Ui};

use crate::headlines::constants::PADDING;

pub fn render_header(ui: &mut Ui) {
    ui.add_space(PADDING);

    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });

    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.));
}
