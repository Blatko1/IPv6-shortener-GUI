mod app;
mod error;
mod shortener;

use app::IPv6ShortenApp;
use eframe::egui;

pub const WIN_WIDTH: f32 = 600.0;
pub const WIN_HEIGHT: f32 = 240.0;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIN_WIDTH, WIN_HEIGHT)),
        //icon_data: todo!(),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "IPv6 Shortener",
        options,
        Box::new(|_cc| Box::new(IPv6ShortenApp::new())),
    );
}
