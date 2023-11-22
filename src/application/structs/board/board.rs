use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

use iced::{
    alignment, event,
    mouse::{self, Button, Cursor},
    touch,
    widget::canvas::{Cache, Canvas, Event, Frame, Geometry, Path, Program, Text},
    Color, Element, Font, Length, Point, Rectangle, Renderer, Size, Theme, Vector,
};
use once_cell::sync::Lazy;

use crate::application::{
    enums::{Piece, Side},
    structs::Position,
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

/// Игровая шашечная доска
pub struct Board {
    white_pieces: HashMap<Position, Piece>,
    black_pieces: HashMap<Position, Piece>,
    /// Текущий ход стороны
    current_move: Side,

    /// Хранит сгенерированные примитивы для отрисовки игровой доски
    board_cache: Cache,
    /// Хранит сгенерированные примитивы для отрисовки фигур игровой доски
    pieces_cache: Cache,
}

impl Default for Board {
    fn default() -> Self {
        let piece_default_positions = Self::get_piece_default_positions();
        Self {
            white_pieces: piece_default_positions[&Side::White]
                .iter()
                .map(|position| (*position, Piece::default()))
                .collect(),
            black_pieces: piece_default_positions[&Side::Black]
                .iter()
                .map(|position| (*position, Piece::default()))
                .collect(),
            current_move: Side::default(),
            board_cache: Cache::new(),
            pieces_cache: Cache::new(),
        }
    }
}

impl Board {
    /// Количество фигур у игрока по умолчанию
    const DEFAULT_PLAYERS_PIECES_QUANTITY: u8 = 12;
    /// Стандартный размер доски в клетках: (кол-во строк, кол-во столбцов)
    const DEFAULT_SIZE: (u8, u8) = (8, 8);
    /// Строки, на которых изначально расположены черные шашки
    const ROWS_BELONING_TO_BLACK: RangeInclusive<u8> = 0_u8..=2_u8;
    /// Строки, на которых изначально расположены белые шашки
    const ROWS_BELONING_TO_WHITE: RangeInclusive<u8> = 5_u8..=7_8;

    /// Цвет "белых" ячеек доски
    const GRAY_CELL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.75);
    /// Цвет "черных" ячеек доски
    const RED_CELL_COLOR: Color = Color::from_rgb(0.644, 0.164, 0.164);
    /// Цвет ячейки, над которой находится курсор пользователя
    const HOVERED_CELL_COLOR: Color = Color::from_rgba(0.574, 0.437, 0.855, 0.42);

    const AVAILABLE_CELL_FOR_MOVING_COLOR: Color = Color::from_rgba(0.0, 1.0, 0.0, 0.42);
    const AVAILABLE_CELL_FOR_TAKING_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 0.42);

    const BLACK_PIECE_COLOR: Color = Color::BLACK;
    const WHITE_PIECE_COLOR: Color = Color::WHITE;
    const KING_CROWN_COLOR: Color = Color::from_rgb(0.996, 0.839, 0.0);

    /// Размер ячейки доски
    const CELL_WIDTH: f32 = 80.0;
    /// Радиус фигуры
    const PIECE_RADIUS: f32 = 0.4;
    /// Радиус короны "дамки"
    const KING_CROWN_RADIUS: f32 = 0.2;

    const BOARD_MARGIN_RIGHT: f32 = 10.0;
    const SPACING_BETWEEN_TEXT: f32 = 20.0;

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Передвигает фигуру из позиции from, в позицию to
    pub fn move_piece(&mut self, side: Side, from: Position, to: Position) {
        let pieces = match side {
            Side::White => &mut self.white_pieces,
            Side::Black => &mut self.black_pieces,
        };
        let piece = pieces.get(&from).unwrap().clone();
        pieces.remove(&from);
        pieces.insert(to, piece);
        // Принудительно перерисовываем фишки на доске
        self.pieces_cache.clear();
    }

    /// Передать ход противоположной стороне
    pub fn pass_the_move(&mut self) {
        self.current_move = self.current_move.opposite();
    }

    #[inline(always)]
    fn get_board_size(cells: (u8, u8), cell_width: f32) -> (f32, f32) {
        (cells.0 as f32 * cell_width, cells.1 as f32 * cell_width)
    }

    #[inline(always)]
    /// Возвращает позицию текстового элемента на данной строке
    ///
    /// Параметр row начинается с 0
    fn get_text_line_point(row: usize) -> Point {
        let board_size = Self::get_board_size(Self::DEFAULT_SIZE, Self::CELL_WIDTH);
        Point {
            x: board_size.1 + Self::BOARD_MARGIN_RIGHT,
            y: row as f32 * Self::SPACING_BETWEEN_TEXT,
        }
    }

    fn get_piece_default_positions() -> HashMap<Side, Vec<Position>> {
        use itertools::Itertools;
        (0..Self::DEFAULT_SIZE.0)
            .cartesian_product(0..Self::DEFAULT_SIZE.1)
            // Выбираем координаты черных ячеек доски
            .filter(|(row, column)| {
                row % 2 == 0 && column % 2 == 1 || row % 2 == 1 && column % 2 == 0
            })
            .map(|pair| Position::from(pair))
            // Выбираем координаты черных ячеек на краях доски, где должны стоять шашки игроков
            .filter(|pos| {
                Self::ROWS_BELONING_TO_BLACK.contains(&pos.row)
                    || Self::ROWS_BELONING_TO_WHITE.contains(&pos.row)
            })
            // Делим фигуры на фигуры черных и белых
            .group_by(|pos| Self::ROWS_BELONING_TO_BLACK.contains(&pos.row))
            .into_iter()
            .map(|(key, positions)| (key, positions.collect::<Vec<Position>>()))
            .into_group_map()
            .into_iter()
            .map(|(key, positions_nested)| (Side::from(key), positions_nested[0].clone()))
            .collect()
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
    fn get_cell_position(point: Point, bounds: Rectangle) -> Position {
        Position {
            row: (point.y / Self::CELL_WIDTH) as u8,
            column: (point.x / Self::CELL_WIDTH) as u8,
        }
    }

    /// Возвращает строку, содержащую информацию о количестве фигур у каждой из сторон
    fn get_stats_str(&self) -> String {
        let white_men_quantity = self
            .white_pieces
            .values()
            .filter(|piece| piece.is_man())
            .count();
        let white_kings_quantity = self
            .white_pieces
            .values()
            .filter(|piece| piece.is_king())
            .count();
        let black_men_quantity = self
            .black_pieces
            .values()
            .filter(|piece| piece.is_man())
            .count();
        let black_kings_quantity = self
            .black_pieces
            .values()
            .filter(|piece| piece.is_king())
            .count();
        format!(
            "Статистика:\nШашки белых: {}\nДамки белых {}\n\nШашки чёрных {}\nДамки чёрных {}",
            white_men_quantity, white_kings_quantity, black_men_quantity, black_kings_quantity
        )
    }

    // fn get_available_cell_positions_for_moving(&self, piece_position: Position, piece: &Piece) ->

    fn get_piece_at_position(&self, position: &Position) -> Option<Piece> {
        match self.current_move {
            Side::White => &self.white_pieces,
            Side::Black => &self.black_pieces,
        }.get(position).cloned()
    }
}

impl Program<Message> for Board {
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle<f32>,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        use itertools::Itertools;
        let board = self.board_cache.draw(renderer, bounds.size(), |frame| {
            frame.with_save(|frame| {
                frame.scale(Self::CELL_WIDTH);
                for (row, column) in
                    (0..Self::DEFAULT_SIZE.0).cartesian_product(0..Self::DEFAULT_SIZE.1)
                {
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

                for (position, piece) in &self.black_pieces {
                    Self::draw_piece(frame, *position, *piece, &Self::BLACK_PIECE_COLOR);
                }

                for (position, piece) in &self.white_pieces {
                    Self::draw_piece(frame, *position, *piece, &Self::WHITE_PIECE_COLOR);
                }
            });
        });

        let overlay = {
            let mut frame = Frame::new(renderer, bounds.size());
            if let Some(Position { row, column }) = cursor
                .position_in(bounds)
                .map(|point| Self::get_cell_position(point, bounds))
            {
                if (0..Self::DEFAULT_SIZE.0).contains(&row)
                    && (0..Self::DEFAULT_SIZE.1).contains(&column)
                {
                    frame.with_save(|frame| {
                        frame.scale(Self::CELL_WIDTH);
                        frame.fill_rectangle(
                            Point::new(column as f32, row as f32),
                            Size::UNIT,
                            Self::HOVERED_CELL_COLOR,
                        );
                    });

                    frame.fill_text(Text {
                        content: format!("Текущая ячейка: ({}, {})", row, column),
                        position: Self::get_text_line_point(0),
                        ..OVERLAY_TEXT_PRESET.clone()
                    });
                }
            }
            frame.fill_text(Text {
                content: format!("Сейчас ходят: {}", self.current_move),
                position: Self::get_text_line_point(1),
                ..OVERLAY_TEXT_PRESET.clone()
            });
            frame.fill_text(Text {
                content: self.get_stats_str(),
                position: Self::get_text_line_point(2),
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

        // TODO отладка!
        // Выбор фишки для перемещения или выбор позиции, в которую переместить фишку
        if let Some(cursor_position) = cursor.position() {
            if let Mouse(ButtonPressed(Button::Left)) = event {
                match *state {
                    State::None => {
                        let initial_position = Self::get_cell_position(cursor_position, bounds);
                        if let Some(piece) = self.get_piece_at_position(&initial_position) {
                            *state = State::MovingPiece {
                                initial_position,
                                piece,
                            };
                            log::info!(
                                "Пользователь выбрал фишку: {} на позиции {}",
                                piece,
                                initial_position
                            )
                        }
                    }
                    State::MovingPiece {
                        initial_position,
                        piece,
                    } => {
                        let result_position = Self::get_cell_position(cursor_position, bounds);
                        *state = State::None;
                        return (
                            Status::Captured,
                            Some(Message::MovePiece {
                                from: initial_position,
                                to: result_position,
                                side: self.current_move,
                            }),
                        );
                    }
                }
            }
        }

        (event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Pointer
    }
}
