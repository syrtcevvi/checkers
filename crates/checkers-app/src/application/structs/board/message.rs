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

use checkers_lib::{enums::Side, structs::Position};

#[derive(Debug, Clone)]
pub enum Message {
    // Переместить фигуру из позиции from в позицию to
    MovePiece {
        from: Position,
        to: Position,
        side: Side,
    },
    /// Взятие фигур фигурой, перемещая её из позиции from в позицию to
    TakePieces {
        from: Position,
        to: Position,
        side: Side,
        /// Позиции взятых фигур
        taken_pieces_positions: Vec<Position>,
    },
    Restart,
}
