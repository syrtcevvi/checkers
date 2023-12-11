use std::rc::Rc;

use crate::application::structs::GameData;
use derive_more::Display;

/// Снимок
#[derive(Debug, Display, Clone)]
#[display(fmt = "{}-{}", id, message)]
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
