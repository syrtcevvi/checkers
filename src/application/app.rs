use std::{cell::RefCell, rc::Rc};

use iced::{
    event::Event,
    executor,
    keyboard::{self, KeyCode, Modifiers},
    subscription, theme, time,
    widget::{
        button, checkbox, column, container, pick_list, row, slider, text, text_input, Row, Text,
    },
    Alignment, Application, Color, Command, Element, Length, Renderer, Subscription, Theme,
};

use crate::application::{
    enums::{Message, Side},
    structs::Board,
};

#[derive(Default)]
pub struct Checkers {
    /// Игральная доска
    board: Board,
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
        use crate::application::structs::BoardMessage;
        match message {
            Message::Board(board_message) => match board_message {
                BoardMessage::MovePiece { from, to, side } => {
                    self.board.move_piece(side, from, to);
                    self.board.pass_the_move();
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![self.board.view().map(Message::Board), button("Нажми меня!"),].into()
    }
}
