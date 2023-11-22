use derive_more::Display;

#[derive(Debug, Display, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    #[display(fmt = "Чёрные")]
    Black,
    #[default]
    #[display(fmt = "Белые")]
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
