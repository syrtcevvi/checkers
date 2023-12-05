use crate::application::structs::{BoardMessage, CreationModalMessage, VcsMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Board(BoardMessage),
    Vcs(VcsMessage),
    CreationModal(CreationModalMessage),
    None,
}
