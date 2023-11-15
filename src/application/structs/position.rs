/// Положение фигуры на игральной доске
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub row: u8,
    pub column: u8,
}

impl From<(u8, u8)> for Position {
    fn from(value: (u8, u8)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}
