/// Тип фигуры на шашечной доске
#[derive(Default)]
pub enum Piece {
    #[default]
    Man,
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
