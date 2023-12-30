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

use derive_more::Display;

#[derive(Debug, Display, Clone, Deserialize, Serialize)]
#[display(fmt = "{}", name)]
pub struct Branch {
    /// Идентификатор снимка, на который указывает данная ветка
    ///
    /// Если коммит отсутствует, значит пользователь еще не сделал ни одного
    pub commit_id: Option<isize>,
    /// Название ветки
    pub name: String,
}

impl Branch {
    pub fn new(name: &str, commit_id: Option<isize>) -> Self {
        Self {
            name: name.to_string(),
            commit_id,
        }
    }
}
