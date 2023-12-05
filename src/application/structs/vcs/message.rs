#[derive(Debug, Clone)]
pub enum Message {
    /// Выбрать снимок по идентификатору
    SelectCommit(String),
    /// Выбрать ветку по имени
    SelectBranch(String),
}
