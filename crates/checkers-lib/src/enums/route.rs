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

use crate::structs::Position;

/// Маршрут перемещения фигуры
#[derive(Debug, Clone)]
pub enum Route {
    /// Перемещение
    Movement(Position),
    /// Взятие фигур противника
    ///
    /// Конечная позиция и позиции вражеских фигур, которые были "съедены" во время взятия
    Taking(Position, Vec<Position>),
}

impl Route {
    /// Возвращает конечную позицию фигуры
    pub fn position(&self) -> Position {
        match self {
            Self::Movement(position) => *position,
            Self::Taking(position, ..) => *position,
        }
    }
}
