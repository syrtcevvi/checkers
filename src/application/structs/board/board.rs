use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

use iced::{
    alignment, event,
    mouse::{self, Cursor},
    touch,
    widget::canvas::{Cache, Canvas, Event, Frame, Geometry, Path, Program, Text},
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector,
};

use crate::application::{
    enums::{Piece, Side},
    structs::Position,
};

use super::{Interaction, Message};

/// Игровая шашечная доска
pub struct Board {
    white_pieces: HashMap<Position, Piece>,
    black_pieces: HashMap<Position, Piece>,

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

    const GRAY_CELL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.75);
    const RED_CELL_COLOR: Color = Color::from_rgb(0.644, 0.164, 0.164);
    const HOVERED_CELL_COLOR: Color = Color::from_rgba(0.574, 0.437, 0.855, 0.42);

    const BLACK_PIECE_COLOR: Color = Color::BLACK;
    const WHITE_PIECE_COLOR: Color = Color::WHITE;

    /// Размер ячейки доски
    const CELL_WIDTH: f32 = 80.0;
    /// Радиус фигуры
    const PIECE_RADIUS: f32 = 0.4;

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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

    fn draw_piece(frame: &mut Frame, position: &Position, piece: &Piece, color: &Color) {
        let center = Point {
            x: position.column as f32 + 0.5,
            y: position.row as f32 + 0.5,
        };
        frame.fill(&Path::circle(center, Self::PIECE_RADIUS), *color);
    }

    fn get_position(point: Point, bounds: Rectangle) -> Position {
        Position {
            row: (point.y / Self::CELL_WIDTH) as u8,
            column: (point.x / Self::CELL_WIDTH) as u8,
        }
    }
}

impl Program<Message> for Board {
    type State = Interaction;

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
                    Self::draw_piece(frame, position, piece, &Self::BLACK_PIECE_COLOR);
                }

                for (position, piece) in &self.white_pieces {
                    Self::draw_piece(frame, position, piece, &Self::WHITE_PIECE_COLOR);
                }
            });
        });

        let overlay = {
            let mut frame = Frame::new(renderer, bounds.size());
            if let Some(Position { row, column }) = cursor
                .position_in(bounds)
                .map(|point| Self::get_position(point, bounds))
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

                    // Преднастроенный текстовый элемент TODO to statics
                    let overlay_text_preset: Text = Text {
                        color: Color::BLACK,
                        size: 14.0,
                        horizontal_alignment: alignment::Horizontal::Right,
                        vertical_alignment: alignment::Vertical::Bottom,
                        ..Text::default()
                    };
                    frame.fill_text(Text {
                        content: format!("({}, {})", row, column),
                        position: Point::new(frame.width(), frame.height() - 5.0),
                        ..overlay_text_preset
                    });
                }
            }
            frame.into_geometry()
        };

        vec![board, pieces, overlay]
    }

    fn update(
        &self,
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
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