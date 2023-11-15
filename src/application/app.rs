use std::{cell::RefCell, rc::Rc};

use iced::{
    executor,
    widget::{button, column, row, Text},
    Application, Command, Element, Renderer, Theme,
};

use crate::application::{enums::Message, structs::Board};

#[derive(Default)]
pub struct Checkers {
    /// Игральная доска
    board: Rc<RefCell<Board>>,
}

impl Application for Checkers {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Шашки с системой контроля версий")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {}
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        button("Нажми меня!").into()
    }
}
