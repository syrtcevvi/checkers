use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    #[display(fmt = "Чёрные")]
    Black,
    #[default]
    #[display(fmt = "Белые")]
    White,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

impl From<bool> for Side {
    fn from(value: bool) -> Self {
        if value {
            Self::Black
        } else {
            Self::White
        }
    }
}
