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

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Тип фигуры на шашечной доске
#[derive(Debug, Display, Default, Clone, Copy, Deserialize, Serialize)]
pub enum Piece {
    #[default]
    #[display(fmt = "Шашка")]
    Man,
    #[display(fmt = "Дамка")]
    King,
}

impl Piece {
    pub fn is_man(&self) -> bool {
        matches!(self, Piece::Man)
    }

    pub fn is_king(&self) -> bool {
        matches!(self, Piece::King)
    }
}
