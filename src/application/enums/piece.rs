use derive_more::Display;

/// Тип фигуры на шашечной доске
#[derive(Debug, Display, Default, Clone, Copy)]
pub enum Piece {
    #[default]
    #[display(fmt = "Шашка")]
    Man,
    #[display(fmt = "Дамка")]
    King,
}

impl Piece {
    pub fn is_man(&self) -> bool {
        if let Piece::Man = self {
            true
        } else {
            false
        }
    }

    pub fn is_king(&self) -> bool {
        if let Piece::King = self {
            true
        } else {
            false
        }
    }
}
