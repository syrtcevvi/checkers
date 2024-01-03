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
    alignment, event,
    mouse::{self, Button, Cursor},
    widget::canvas::{Cache, Canvas, Event, Frame, Geometry, Path, Program, Text},
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme,
};
use once_cell::sync::Lazy;

use crate::application::{
    enums::{Piece, Route, Side},
    structs::{GameData, Position},
};

use super::{Message, State};

// Преднастроенная конфигурация текстового элемента, отображаемого в overlay
static OVERLAY_TEXT_PRESET: Lazy<Text> = Lazy::new(|| Text {
    color: Color::BLACK,
    size: 20.0,
    horizontal_alignment: alignment::Horizontal::Left,
    vertical_alignment: alignment::Vertical::Top,
    ..Text::default()
});

/// Игровая доска игры "Шашки"
pub struct Board {
    /// Данные о состоянии игрового поля
    game_data: Rc<RefCell<GameData>>,
    /// Хранит сгенерированные примитивы для отрисовки игровой доски
    board_cache: Cache,
    /// Хранит сгенерированные примитивы для отрисовки фигур игровой доски
    pieces_cache: Cache,
}

impl Board {
    /// Цвет "белых" ячеек доски
    const GRAY_CELL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.75);
    /// Цвет "черных" ячеек доски
    const RED_CELL_COLOR: Color = Color::from_rgb(0.644, 0.164, 0.164);
    /// Цвет ячейки, над которой находится курсор пользователя
    const HOVERED_CELL_COLOR: Color = Color::from_rgba(0.574, 0.437, 0.855, 0.42);

    const AVAILABLE_CELL_FOR_MOVING_COLOR: Color = Color::from_rgba(0.0, 1.0, 0.0, 1.0);
    const AVAILABLE_CELL_FOR_TAKING_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);

    const BLACK_PIECE_COLOR: Color = Color::BLACK;
    const BLACK_PIECE_MOVING_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.42);
    const WHITE_PIECE_COLOR: Color = Color::WHITE;
    const WHITE_PIECE_MOVING_COLOR: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.42);
    const KING_CROWN_COLOR: Color = Color::from_rgb(0.996, 0.839, 0.0);

    /// Размер ячейки доски
    const CELL_WIDTH: f32 = 80.0;
    /// Радиус фигуры
    const PIECE_RADIUS: f32 = 0.4;
    /// Радиус короны "дамки"
    const KING_CROWN_RADIUS: f32 = 0.2;

    const BOARD_MARGIN_RIGHT: f32 = 10.0;
    const SPACING_BETWEEN_TEXT: f32 = 20.0;

    pub fn new(game_data: Rc<RefCell<GameData>>) -> Self {
        Self {
            game_data,
            board_cache: Cache::new(),
            pieces_cache: Cache::new(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Ставит игровую доску в очередь для перерисовки
    pub fn update(&self) {
        self.board_cache.clear();
        self.pieces_cache.clear();
    }

    fn get_board_size(&self) -> (f32, f32) {
        let cells = self.game_data.borrow().board_cells();
        let cell_width = Self::CELL_WIDTH;
        (cells.0 as f32 * cell_width, cells.1 as f32 * cell_width)
    }

    #[inline(always)]
    /// Возвращает позицию текстового элемента на данной строке
    ///
    /// Параметр row начинается с 0
    fn get_text_line_point(&self, row: usize) -> Point {
        let board_size = self.get_board_size();
        Point {
            x: board_size.1 + Self::BOARD_MARGIN_RIGHT,
            y: row as f32 * Self::SPACING_BETWEEN_TEXT,
        }
    }

    /// Рисует фигуру на указанной позиции на игральной доске
    fn draw_piece(frame: &mut Frame, position: Position, piece: Piece, color: &Color) {
        let center = Point {
            x: position.column as f32 + 0.5,
            y: position.row as f32 + 0.5,
        };
        match piece {
            Piece::Man => frame.fill(&Path::circle(center, Self::PIECE_RADIUS), *color),
            Piece::King => {
                frame.fill(&Path::circle(center, Self::PIECE_RADIUS), *color);
                frame.fill(
                    &Path::circle(center, Self::KING_CROWN_RADIUS),
                    Self::KING_CROWN_COLOR,
                );
            }
        }
    }

    /// Возвращает позицию ячейки игральной доски, которая содержит в себе данную точку
    fn get_cell_position(point: Point) -> Position {
        Position {
            row: (point.y / Self::CELL_WIDTH) as i8,
            column: (point.x / Self::CELL_WIDTH) as i8,
        }
    }

    /// Возвращает строку, содержащую информацию о количестве фигур у каждой из сторон
    fn get_stats_str(&self) -> String {
        let game_data = self.game_data.borrow();
        let white_men_quantity = game_data
            .white_pieces
            .values()
            .filter(|piece| piece.is_man())
            .count();
        let white_kings_quantity = game_data
            .white_pieces
            .values()
            .filter(|piece| piece.is_king())
            .count();
        let black_men_quantity = game_data
            .black_pieces
            .values()
            .filter(|piece| piece.is_man())
            .count();
        let black_kings_quantity = game_data
            .black_pieces
            .values()
            .filter(|piece| piece.is_king())
            .count();
        format!(
            "Статистика:\nШашки белых: {}\nДамки белых {}\n\nШашки чёрных {}\nДамки чёрных {}",
            white_men_quantity, white_kings_quantity, black_men_quantity, black_kings_quantity
        )
    }

    fn get_piece_at_position(&self, position: Position) -> Option<Piece> {
        let game_data = self.game_data.borrow();
        match game_data.current_move {
            Side::White => &game_data.white_pieces,
            Side::Black => &game_data.black_pieces,
        }
        .get(&position)
        .cloned()
    }
}

impl Program<Message> for Board {
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle<f32>,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let game_data = self.game_data.borrow();
        let board = self.board_cache.draw(renderer, bounds.size(), |frame| {
            frame.with_save(|frame| {
                frame.scale(Self::CELL_WIDTH);
                for (row, column) in game_data.board_cell_coordinates() {
                    let color = if (row + column) % 2 == 0 {
                        Self::GRAY_CELL_COLOR
                    } else {
                        Self::RED_CELL_COLOR
                    };
                    frame.fill_rectangle(Point::new(row as f32, column as f32), Size::UNIT, color);
                }
            });
        });

        let pieces = self.pieces_cache.draw(renderer, bounds.size(), |frame| {
            frame.with_save(|frame| {
                frame.scale(Self::CELL_WIDTH);

                for (position, piece) in &game_data.black_pieces {
                    Self::draw_piece(frame, *position, *piece, &Self::BLACK_PIECE_COLOR);
                }

                for (position, piece) in &game_data.white_pieces {
                    Self::draw_piece(frame, *position, *piece, &Self::WHITE_PIECE_COLOR);
                }
            });
        });

        let overlay = {
            let mut frame = Frame::new(renderer, bounds.size());
            if let Some(position) = cursor.position_in(bounds).map(Self::get_cell_position) {
                // Если пользователь указывает на одну из ячеек игральной доски
                if game_data.is_inside_board(position) {
                    // Подсвечиваем ячейку доски, над которой находится курсор пользователя
                    frame.with_save(|frame| {
                        frame.scale(Self::CELL_WIDTH);
                        frame.fill_rectangle(
                            Point::new(position.column as f32, position.row as f32),
                            Size::UNIT,
                            Self::HOVERED_CELL_COLOR,
                        );
                        // Отрисовываем выбранную фигуру во время перемещения
                        match state {
                            State::MovingPiece {
                                piece,
                                initial_position,
                            } => {
                                let moving_piece_color = match game_data.current_move {
                                    Side::White => Self::WHITE_PIECE_COLOR,
                                    Side::Black => Self::BLACK_PIECE_COLOR,
                                };
                                // Показываем, что фигуры как будто бы уже нет на прежней позиции
                                frame.fill_rectangle(
                                    Point::new(
                                        initial_position.column as f32,
                                        initial_position.row as f32,
                                    ),
                                    Size::UNIT,
                                    Self::HOVERED_CELL_COLOR,
                                );
                                let available_routes =
                                    game_data.get_available_routes(*initial_position, *piece);

                                // Отрисовываем возможные ходы для даной фигуры
                                for route in &available_routes {
                                    match route {
                                        Route::Movement(position) => {
                                            frame.fill_rectangle(
                                                Point::new(
                                                    position.column as f32,
                                                    position.row as f32,
                                                ),
                                                Size::UNIT,
                                                Self::AVAILABLE_CELL_FOR_MOVING_COLOR,
                                            );
                                        }
                                        Route::Taking(position, ..) => {
                                            frame.fill_rectangle(
                                                Point::new(
                                                    position.column as f32,
                                                    position.row as f32,
                                                ),
                                                Size::UNIT,
                                                Self::AVAILABLE_CELL_FOR_TAKING_COLOR,
                                            );
                                        }
                                    };
                                }

                                if game_data
                                    .get_route_containing_position(&available_routes, position)
                                    .is_some()
                                {
                                    Self::draw_piece(frame, position, *piece, &moving_piece_color);
                                }
                            }
                            _ => {}
                        }
                    });

                    frame.fill_text(Text {
                        content: format!("Текущая ячейка: {}", position),
                        position: self.get_text_line_point(0),
                        ..OVERLAY_TEXT_PRESET.clone()
                    });
                }
            }
            frame.fill_text(Text {
                content: format!("Сейчас ходят: {}", game_data.current_move),
                position: self.get_text_line_point(1),
                ..OVERLAY_TEXT_PRESET.clone()
            });
            frame.fill_text(Text {
                content: self.get_stats_str(),
                position: self.get_text_line_point(2),
                ..OVERLAY_TEXT_PRESET.clone()
            });

            frame.into_geometry()
        };

        vec![board, pieces, overlay]
    }

    fn update(
        &self,
        state: &mut State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        use event::Status;
        use mouse::Event::*;
        use Event::*;
        if let Mouse(ButtonPressed(Button::Right)) = event {
            *state = State::None;
            return (Status::Captured, None);
        }

        // Выбор фишки для перемещения или выбор позиции, в которую переместить фишку
        if let Some(cursor_position) = cursor.position_in(bounds) {
            let game_data = self.game_data.borrow();
            if let Mouse(ButtonPressed(Button::Left)) = event {
                match *state {
                    State::None => {
                        let initial_position = Self::get_cell_position(cursor_position);
                        if let Some(piece) = self.get_piece_at_position(initial_position) {
                            *state = State::MovingPiece {
                                initial_position,
                                piece,
                            };
                        }
                    }
                    State::MovingPiece {
                        initial_position,
                        piece,
                    } => {
                        let result_position = Self::get_cell_position(cursor_position);
                        let available_routes =
                            game_data.get_available_routes(initial_position, piece);
                        // Если пользователь совершает перемещение в корректную ячейку
                        if let Some(route) = game_data
                            .get_route_containing_position(&available_routes, result_position)
                        {
                            *state = State::None;
                            match route {
                                Route::Movement(_position) => {
                                    return (
                                        Status::Captured,
                                        Some(Message::MovePiece {
                                            from: initial_position,
                                            to: result_position,
                                            side: game_data.current_move,
                                        }),
                                    );
                                }
                                Route::Taking(_position, taken_pieces_positions) => {
                                    return (
                                        Status::Captured,
                                        Some(Message::TakePieces {
                                            from: initial_position,
                                            to: result_position,
                                            side: game_data.current_move,
                                            taken_pieces_positions,
                                        }),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }

        (event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Pointer
    }
}
