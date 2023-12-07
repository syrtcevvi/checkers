use crate::application::structs::Position;

/// Маршрут перемещения фигуры
#[derive(Debug, Clone)]
pub enum Route {
    /// Перемещение
    Movement(Position),
    /// Взятие фигур противника
    ///
    /// Конечная позиция и позиция вражеских фигур, которые были "съедены" во время взятия
    Taking(Position, Vec<Position>),
}

impl Route {
    /// Возвращает конечную позицию фигуры
    pub fn position(&self) -> Position {
        match self {
            Self::Movement(position) => position.clone(),
            Self::Taking(position, ..) => position.clone(),
        }
    }
}
