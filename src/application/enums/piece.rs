use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Тип фигуры на шашечной доске
#[derive(Debug, Display, Serialize, Deserialize, Default, Clone, Copy)]
pub enum Piece {
    #[default]
    #[display(fmt = "Шашка")]
    Man,
    #[display(fmt = "Дамка")]
    King,
}

impl Piece {
    pub fn is_man(&self) -> bool {
        matches!(self, Piece::Man)
    }

    pub fn is_king(&self) -> bool {
        matches!(self, Piece::King)
    }
}
