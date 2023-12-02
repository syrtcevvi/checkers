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
    structs::{Board, GameData},
};

pub struct Checkers {
    /// Игральная доска
    board: Board,
    /// Данные о состоянии игры
    game_data: Rc<RefCell<GameData>>,
}

impl Application for Checkers {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let game_data = Rc::new(RefCell::new(GameData::default()));
        (
            Self {
                board: Board::new(game_data.clone()),
                game_data: game_data,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Шашки с системой контроля версий")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use crate::application::structs::BoardMessage;
        match message {
            Message::Board(board_message) => {
                match board_message {
                    BoardMessage::MovePiece { from, to, side } => {
                        let mut game_data = self.game_data.borrow_mut();
                        game_data.move_piece(side, from, to);
                        // TODO Проверка условия превращения шашки в дамку
                        game_data.pass_the_move();
                    }
                }
                self.board.update();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![self.board.view().map(Message::Board), button("Нажми меня!"),].into()
    }
}
