use crate::application::structs::Position;

/// Маршрут перемещения фигуры
#[derive(Debug)]
pub enum Route {
    /// Перемещение
    Movement(Position),
    /// Взятие фигур противника
    Taking(Vec<Position>),
}
