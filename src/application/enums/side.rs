#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Black,
    White,
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
