use eframe::epaint::{Pos2, Vec2};
use eframe::{run_native, NativeOptions};

mod headlines;
use headlines::Headlines;

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.initial_window_pos = Some(Pos2::new(600., 10.));
    native_options.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(
        "Headlines",
        native_options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
