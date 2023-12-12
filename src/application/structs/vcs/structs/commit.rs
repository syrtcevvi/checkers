use serde::{Deserialize, Serialize};

use crate::application::structs::GameData;
use derive_more::Display;

/// Снимок версионного контроля
#[derive(Debug, Display, Clone, Deserialize, Serialize)]
#[display(fmt = "{}-{}", id, message)]
pub struct Commit {
    /// Идентификатор снимка
    pub id: isize,
    /// Идентификатор родительского снимка. Если данный снимок является первым в истории, у него нет
    pub parent_commit_id: Option<isize>,
    /// Сообщение, связанное с данным снимком
    pub message: String,
    /// Состояние игры, сохранённое в снимке
    pub game_data: GameData,
}
