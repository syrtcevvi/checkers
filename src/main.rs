mod application;
use iced::{window, Application, Result, Settings};

use application::Checkers;

fn main() -> Result {
    Checkers::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
