use crate::application::structs::BoardMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Board(BoardMessage),
}
