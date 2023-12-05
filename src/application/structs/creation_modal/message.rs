use super::ModalType;

#[derive(Debug, Clone)]
pub enum Message {
    Open(ModalType),
    Close,
    TextInputChanged(String),
    Finished(String),
}
