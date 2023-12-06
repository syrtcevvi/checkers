use derive_more::Display;

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
    pub fn diagonal_neighbours(&self, steps: i8) -> Vec<Position> {
        let mut neighbours = Vec::with_capacity(steps as usize * 4);
        for offset in 1..=steps {
            neighbours.extend([
                Position::from((self.row - offset, self.column - offset)),
                Position::from((self.row - offset, self.column + offset)),
                Position::from((self.row + offset, self.column - offset)),
                Position::from((self.row + offset, self.column + offset)),
            ]);
        }
        neighbours
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
