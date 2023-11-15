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

/// Игровая шашечная доска
pub struct Board {
    white_pieces: HashMap<Position, Piece>,
    black_pieces: HashMap<Position, Piece>,
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
}
