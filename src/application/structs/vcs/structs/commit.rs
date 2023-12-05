use crate::application::structs::GameData;
use derive_more::Display;
use std::rc::Rc;

/// Снимок
#[derive(Debug, Display, Clone)]
#[display(fmt = "{} {}", message, id)]
pub struct Commit {
    /// Идентификатор снимка
    pub id: isize,
    /// Идентификатор родительского снимка. Если данный снимок является первым в истории, у него нет
    pub parent_commit: Option<Rc<Commit>>,
    /// Сообщение, связанное с данным снимком
    pub message: String,
    /// Состояние игры, сохранённое в снимке
    pub game_data: GameData,
}
