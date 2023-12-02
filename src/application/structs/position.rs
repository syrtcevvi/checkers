use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Положение фигуры на игральной доске
#[derive(Debug, Display, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
#[display(fmt = "({}, {})", row, column)]
pub struct Position {
    pub row: i8,
    pub column: i8,
}

impl From<(i8, i8)> for Position {
    fn from(value: (i8, i8)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}
