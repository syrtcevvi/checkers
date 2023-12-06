use std::{collections::HashMap, ops::RangeInclusive};

use crate::application::{
    enums::{Piece, Route, Side},
    structs::Position,
};

#[derive(Debug, Clone)]
pub struct GameData {
    pub white_pieces: HashMap<Position, Piece>,
    pub black_pieces: HashMap<Position, Piece>,
    /// Текущий ход стороны
    pub current_move: Side,
}

impl Default for GameData {
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
        }
    }
}

impl GameData {
    /// Стандартный размер доски в клетках: (кол-во строк, кол-во столбцов)
    const DEFAULT_SIZE: (i8, i8) = (8, 8);
    /// Строки, на которых изначально расположены черные шашки
    const ROWS_BELONING_TO_BLACK: RangeInclusive<i8> = 0..=2;
    const BLACK_FIRST_ROW: i8 = 0;
    /// Строки, на которых изначально расположены белые шашки
    const ROWS_BELONING_TO_WHITE: RangeInclusive<i8> = 5..=7;
    const WHITE_FIRST_ROW: i8 = 0;

    /// Возвращает размер игральной доски в ячейках
    pub fn board_cells(&self) -> (i8, i8) {
        Self::DEFAULT_SIZE
    }

    /// Проверяет, выполнено ли для фигуры условие того, что она превращается в дамку
    pub fn is_turning_to_king_condition_satisfied(&self, side: Side, position: Position) -> bool {
        let piece = match side {
            Side::White => self.white_pieces.get(&position),
            Side::Black => self.black_pieces.get(&position),
        }
        .unwrap();

        if piece.is_king() {
            return false;
        }

        match side {
            Side::White => position.row == Self::BLACK_FIRST_ROW,
            Side::Black => position.row == Self::WHITE_FIRST_ROW,
        }
    }

    /// Превращает шашку данной стороны на указанной позиции в дамку
    pub fn turn_man_to_king(&mut self, side: Side, position: Position) {
        let pieces = match side {
            Side::White => &mut self.white_pieces,
            Side::Black => &mut self.black_pieces,
        };
        pieces.insert(position, Piece::King);
    }

    /// Возвращает итератор по координатам ячеек игральной доски
    pub fn board_cell_coordinates(&self) -> impl Iterator<Item = (i8, i8)> {
        use itertools::Itertools;
        (0..Self::DEFAULT_SIZE.0).cartesian_product(0..Self::DEFAULT_SIZE.1)
    }

    /// Проверяет, расположена ли ячейка с указанными координатами в пределах игральной доски
    pub fn is_inside_board(&self, position: Position) -> bool {
        (0..Self::DEFAULT_SIZE.0).contains(&position.row)
            && (0..Self::DEFAULT_SIZE.1).contains(&position.column)
    }

    /// Передвигает фигуру из позиции from, в позицию to
    pub fn move_piece(&mut self, side: Side, from: Position, to: Position) {
        let pieces = match side {
            Side::White => &mut self.white_pieces,
            Side::Black => &mut self.black_pieces,
        };
        let piece = *pieces.get(&from).unwrap();
        pieces.remove(&from);
        pieces.insert(to, piece);
    }

    /// Передать ход противоположной стороне
    pub fn pass_the_move(&mut self) {
        self.current_move = self.current_move.opposite();
    }

    /// Проверяет, содержится ли ячейка доски в корректных путях
    pub fn routes_contains_position(&self, routes: &Vec<Route>, position: Position) -> bool {
        for route in routes {
            if match route {
                Route::Movement(pos) => position == *pos,
                Route::Taking(positions) => positions.contains(&position),
            } {
                return true;
            }
        }
        false
    }

    /// Просчитывает возможные пути для фигуры на указанной позиции
    pub fn get_available_routes(&self, position: Position, piece: Piece) -> Vec<Route> {
        let movement_routes: Vec<Route> =
            self.get_movement_routes(position, piece, self.current_move);

        movement_routes
    }

    /// Возвращает позиции, в которые можно перейти, находясь в текущей ячейке за определённую сторону
    fn get_movement_routes(&self, position: Position, piece: Piece, side: Side) -> Vec<Route> {
        match side {
            Side::White => match piece {
                Piece::Man => {
                    vec![
                        (position.row - 1, position.column - 1).into(),
                        (position.row - 1, position.column + 1).into(),
                    ]
                }
                Piece::King => {
                    todo!()
                }
            },
            Side::Black => match piece {
                Piece::Man => {
                    vec![
                        (position.row + 1, position.column - 1).into(),
                        (position.row + 1, position.column + 1).into(),
                    ]
                }
                Piece::King => {
                    todo!()
                }
            },
        }
        .into_iter()
        // Отсекаем ячейки, в которых находятся фигуры
        .filter(|position| self.is_cell_empty(*position))
        // Отсекаем ячейки за пределами доски
        .filter(|position| self.is_inside_board(*position))
        .map(Route::Movement)
        .collect()
    }

    // Проверяет, находится ли какая-нибудь фигура в ячейки с указанными координатами
    fn is_cell_empty(&self, position: Position) -> bool {
        !self.white_pieces.contains_key(&position) && !self.black_pieces.contains_key(&position)
    }

    fn get_piece_default_positions() -> HashMap<Side, Vec<Position>> {
        use itertools::Itertools;
        (0..Self::DEFAULT_SIZE.0)
            .cartesian_product(0..Self::DEFAULT_SIZE.1)
            // Выбираем координаты черных ячеек доски
            .filter(|(row, column)| {
                row % 2 == 0 && column % 2 == 1 || row % 2 == 1 && column % 2 == 0
            })
            .map(Position::from)
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
