use crate::application::{enums::Side, structs::Position};

#[derive(Debug, Clone)]
pub enum Message {
    // Переместить фигуру из позиции from в позицию to
    MovePiece {
        from: Position,
        to: Position,
        side: Side,
    },
    /// Взятие фигур фигурой, перемещая её из позиции from в позицию to
    TakePieces {
        from: Position,
        to: Position,
        side: Side,
        /// Позиции взятых фигур
        taken_pieces_positions: Vec<Position>,
    },
}
