use std::{collections::HashMap, ops::RangeInclusive};

use crate::application::{
    enums::{Direction, Piece, Route, Side},
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
    const WHITE_FIRST_ROW: i8 = 7;

    /// Возвращает размер игральной доски в ячейках
    pub fn board_cells(&self) -> (i8, i8) {
        Self::DEFAULT_SIZE
    }

    /// Проверяет, закончена ли игра
    pub fn is_game_ended(&self) -> bool {
        self.white_pieces.len() == 0 || self.black_pieces.len() == 0
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

    /// Удаляет фигуры указанной стороны из указанных позиций
    pub fn remove_pieces(&mut self, positions: &[Position], side: Side) {
        let pieces = match side {
            Side::White => &mut self.white_pieces,
            Side::Black => &mut self.black_pieces,
        };

        for position in positions {
            pieces.remove(position);
        }
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

    /// Если да, возвращает путь, содержащий данную ячейку
    pub fn get_route_containing_position(
        &self,
        routes: &Vec<Route>,
        position: Position,
    ) -> Option<Route> {
        for route in routes {
            if route.position() == position {
                return Some(route.clone());
            }
        }
        None
    }

    /// Просчитывает возможные пути для фигуры на указанной позиции
    pub fn get_available_routes(&self, position: Position, piece: Piece) -> Vec<Route> {
        let movement_routes: Vec<Route> =
            self.get_movement_routes(position, piece, self.current_move);
        let taking_routes: Vec<Route> = self.get_taking_routes(position, piece, self.current_move);

        let mut total_routes = Vec::with_capacity(movement_routes.len() + taking_routes.len());
        total_routes.extend(movement_routes);
        total_routes.extend(taking_routes);
        total_routes
    }

    /// Возвращает позиции, в которые можно перейти, находясь в текущей ячейке за определённую сторону
    fn get_movement_routes(&self, position: Position, piece: Piece, side: Side) -> Vec<Route> {
        match piece {
            Piece::Man => match side {
                Side::White => position.top_diagonal_neighbours(),
                Side::Black => position.bottom_diagonal_neighbours(),
            },
            Piece::King => {
                let positions = position.diagonal_neighbours(8);
                // Надо отсечь ячейки, заблокированные фигурами
                let mut direction_is_blocked_at_position: HashMap<Direction, Position> =
                    HashMap::with_capacity(4);
                let mut non_blocked_positions = Vec::with_capacity(positions.len() / 2);
                for (position, direction) in &positions {
                    if direction_is_blocked_at_position.contains_key(direction) {
                        continue;
                    }
                    if !self.is_cell_empty(*position) {
                        direction_is_blocked_at_position.insert(*direction, *position);
                    } else {
                        non_blocked_positions.push(*position)
                    }
                }
                non_blocked_positions
            }
        }
        .into_iter()
        // Отсекаем ячейки, в которых находятся фигуры
        .filter(|position| self.is_cell_empty(*position))
        // Отсекаем ячейки за пределами доски
        .filter(|position| self.is_inside_board(*position))
        .map(Route::Movement)
        .collect()
    }

    fn get_taking_routes(&self, position: Position, piece: Piece, side: Side) -> Vec<Route> {
        let mut taking_routes: Vec<Route> = Vec::with_capacity(16);
        self.get_taking_routes_rec(position, piece, side, &vec![], &mut taking_routes);
        taking_routes
    }

    fn get_taking_routes_rec(
        &self,
        position: Position,
        piece: Piece,
        side: Side,
        taken_pieces_positions: &Vec<Position>,
        taking_routes: &mut Vec<Route>,
    ) {
        let positions: Vec<(Position, Direction)> = match piece {
            Piece::Man => match side {
                Side::White => position.diagonal_neighbours(1),
                Side::Black => position.diagonal_neighbours(1),
            },
            Piece::King => {
                let positions = position.diagonal_neighbours(8);
                // Надо отсечь ячейки, заблокированные фигурами
                let mut direction_is_blocked_at_position: HashMap<Direction, Position> =
                    HashMap::with_capacity(4);
                let mut non_blocked_positions = Vec::with_capacity(positions.len() / 2);
                for (position, direction) in &positions {
                    if direction_is_blocked_at_position.contains_key(direction) {
                        continue;
                    }
                    if !self.is_cell_empty(*position) {
                        if self.contains_takable_enemy_piece(*position, *direction, side) {
                            non_blocked_positions.push((*position, *direction))
                        }
                        direction_is_blocked_at_position.insert(*direction, *position);
                    } else {
                        non_blocked_positions.push((*position, *direction))
                    }
                }
                non_blocked_positions
            }
        }
        .into_iter()
        // Пропускаем ячейки, в которых фигура противника уже была взята за данный "проход"
        .filter(|(position, _)| !taken_pieces_positions.contains(position))
        // Пропускаем ячейки, в которых находится союзная фигура
        .filter(|(position, _)| !self.contains_ally_piece(*position, side))
        // Выбираем ячейки, содержащие фигуры противника, которые можно взять
        .filter(|(position, direction)| {
            self.contains_takable_enemy_piece(*position, *direction, side)
        })
        .collect();

        // Позиции вражеских фигур, которые можно взять
        for (enemy_piece_position, direction) in positions {
            let result_position = enemy_piece_position.next_diagonal(direction);
            let mut enemy_pieces_positions = taken_pieces_positions.clone();
            enemy_pieces_positions.push(enemy_piece_position);
            self.get_taking_routes_rec(
                result_position,
                piece,
                side,
                &enemy_pieces_positions,
                taking_routes,
            );

            taking_routes.push(Route::Taking(result_position, enemy_pieces_positions));
        }
    }

    /// Проверяет, находится ли какая-нибудь фигура в ячейки с указанными координатами
    fn is_cell_empty(&self, position: Position) -> bool {
        !self.white_pieces.contains_key(&position) && !self.black_pieces.contains_key(&position)
    }

    /// Проверяет, содержит ли ячейка союзную фигуру
    fn contains_ally_piece(&self, position: Position, current_side: Side) -> bool {
        let ally_pieces = self.pieces(current_side);
        ally_pieces.contains_key(&position)
    }

    /// Проверяет, содержит ли ячейка фигуру противника, которую можно взять по правилам шашек
    ///
    /// Фигуру можно взять по диагонали, если за ней находится свободная ячейка
    fn contains_takable_enemy_piece(
        &self,
        position: Position,
        direction: Direction,
        current_side: Side,
    ) -> bool {
        let enemy_pieces = self.pieces(current_side.opposite());
        enemy_pieces.contains_key(&position)
        // Проверяем, что следующая ячейка находится в пределах доски
            && self.is_inside_board(position.next_diagonal(direction))
        // И что она свободна
            && self.is_cell_empty(position.next_diagonal(direction))
    }

    fn pieces(&self, side: Side) -> &HashMap<Position, Piece> {
        match side {
            Side::White => &self.white_pieces,
            Side::Black => &self.black_pieces,
        }
    }

    fn pieces_mut(&mut self, side: Side) -> &mut HashMap<Position, Piece> {
        match side {
            Side::White => &mut self.white_pieces,
            Side::Black => &mut self.black_pieces,
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
