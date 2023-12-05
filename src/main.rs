mod application;
use dotenv::dotenv;
use iced::{window, Application, Result, Settings};

use application::Checkers;

fn main() -> Result {
    dotenv().ok();
    Checkers::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
