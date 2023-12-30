/*
Copyright 2023 Сырцев Вадим Игоревич

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

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
