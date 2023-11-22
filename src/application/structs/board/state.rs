use crate::application::{enums::Piece, structs::Position};

#[derive(Default)]
pub enum State {
    #[default]
    /// Пользователь не взаимодействует с игровым полем
    None,
    /// Пользователь перемещает выбранную фигуру
    MovingPiece {
        initial_position: Position,
        piece: Piece,
    },
}
