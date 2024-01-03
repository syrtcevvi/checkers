/*
Copyright 2023 Сырцев Вадим Игоревич

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::{cell::RefCell, rc::Rc};

use iced::{
    event::Event, executor, subscription, widget::column, window, Application, Command, Element,
    Renderer, Subscription, Theme,
};

use crate::application::{
    enums::Message,
    structs::{Board, BoardMessage, GameData},
};

pub struct Checkers {
    /// Игральная доска
    board: Board,
    /// Данные о состоянии игры
    game_data: Rc<RefCell<GameData>>,
}

impl Default for Checkers {
    fn default() -> Self {
        let game_data = Rc::new(RefCell::new(GameData::default()));
        Self {
            board: Board::new(game_data.clone()),
            game_data,
        }
    }
}

impl Application for Checkers {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from(r#"Игра "Шашки""#)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Board(board_message) => {
                match board_message {
                    BoardMessage::MovePiece { from, to, side } => {
                        let mut game_data = self.game_data.borrow_mut();
                        game_data.move_piece(side, from, to);
                        if game_data.is_turning_to_king_condition_satisfied(side, to) {
                            game_data.turn_man_to_king(side, to);
                            // После превращения шашки в дамку ход продолжается
                        } else {
                            game_data.pass_the_move();
                        }
                    }
                    BoardMessage::TakePieces {
                        from,
                        to,
                        side,
                        taken_pieces_positions,
                    } => {
                        let mut game_data = self.game_data.borrow_mut();
                        // Убираем все фишки на пути, так как они были "съедены" во время взятия
                        game_data.remove_pieces(&taken_pieces_positions, side.opposite());
                        game_data.move_piece(side, from, to);
                        if game_data.is_turning_to_king_condition_satisfied(side, to) {
                            game_data.turn_man_to_king(side, to);
                            // После превращения шашки в дамку ход продолжается
                        } else {
                            game_data.pass_the_move();
                        }
                        if game_data.is_game_ended() {
                            return Command::perform(std::future::ready(()), |_| {
                                Message::Board(BoardMessage::Restart)
                            });
                        }
                    }
                    BoardMessage::Restart => {
                        self.game_data.replace(GameData::default());
                    }
                }
                self.board.update();
            }
            Message::EventOccured(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    return window::close();
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccured)
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        column![self.board.view().map(Message::Board)].into()
    }
}
