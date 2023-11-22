use crate::application::{enums::Side, structs::Position};

#[derive(Debug, Clone)]
pub enum Message {
    // Переместить фигуру из позиции from в позицию to
    MovePiece {
        from: Position,
        to: Position,
        side: Side,
    },
}
