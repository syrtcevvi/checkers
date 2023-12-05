#[derive(Debug, Clone)]
pub enum Message {
    /// Выбрать снимок по идентификатору
    SwitchToCommit(isize),
    /// Выбрать ветку по имени
    SwitchToBranch(String),
}
