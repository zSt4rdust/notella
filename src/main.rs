use app::App;
use iced::{window, Application, Settings, Size};

pub mod app;
pub mod cards;
pub mod elements;
pub mod utils;

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(200.0, 150.0)),

            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
