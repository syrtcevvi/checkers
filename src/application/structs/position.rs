use derive_more::Display;

use crate::application::enums::Direction;

/// Положение фигуры на игральной доске
#[derive(Debug, Display, Hash, PartialEq, Eq, Clone, Copy)]
#[display(fmt = "({}, {})", row, column)]
pub struct Position {
    pub row: i8,
    pub column: i8,
}

impl Position {
    /// Возвращает позиции клеток-соседей, которые находятся сверху по диагонали на расстоянии 1
    pub fn top_diagonal_neighbours(&self) -> Vec<Position> {
        vec![
            (self.row - 1, self.column - 1).into(),
            (self.row - 1, self.column + 1).into(),
        ]
    }

    /// Возвращает позиции клеток-соседей, которые находятся снизу по диагонали на расстоянии 1
    pub fn bottom_diagonal_neighbours(&self) -> Vec<Position> {
        vec![
            (self.row + 1, self.column - 1).into(),
            (self.row + 1, self.column + 1).into(),
        ]
    }

    /// Возвращает позиции клеток-соседей, которые находятся на диагоналях, содержащих ячейку на данной позиции
    ///
    /// Параметры:
    /// step: на каком расстоянии находятся клетки-соседи
    pub fn diagonal_neighbours(&self, steps: i8) -> Vec<(Position, Direction)> {
        let mut neighbours = Vec::with_capacity(steps as usize * 4);
        for offset in 1..=steps {
            neighbours.extend([
                (
                    Position::from((self.row - offset, self.column - offset)),
                    Direction::LeftUp,
                ),
                (
                    Position::from((self.row - offset, self.column + offset)),
                    Direction::RightUp,
                ),
                (
                    Position::from((self.row + offset, self.column - offset)),
                    Direction::LeftDown,
                ),
                (
                    Position::from((self.row + offset, self.column + offset)),
                    Direction::RightDown,
                ),
            ]);
        }
        neighbours
    }

    // Возвращает ячейку, которая находится рядом с текущей по диагонали, в указанном направлении
    pub fn next_diagonal(&self, direction: Direction) -> Position {
        match direction {
            Direction::LeftUp => (self.row - 1, self.column - 1).into(),
            Direction::LeftDown => (self.row + 1, self.column - 1).into(),
            Direction::RightDown => (self.row + 1, self.column + 1).into(),
            Direction::RightUp => (self.row - 1, self.column + 1).into(),
        }
    }
}

impl From<(i8, i8)> for Position {
    fn from(value: (i8, i8)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}
