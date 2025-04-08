#![windows_subsystem = "windows"]

use iced::{Settings, Size, font::Font, window};
use simple::Counter;

fn main() -> iced::Result {
    iced::application("DEMO", Counter::update, Counter::view)
        .settings(Settings {
            default_font: Font::with_name("微软雅黑"),
            ..Settings::default()
        })
        .window(window::Settings {
            size: Size {
                width: 400.0,
                height: 300.0,
            },
            ..window::Settings::default()
        })
        .run()
}
