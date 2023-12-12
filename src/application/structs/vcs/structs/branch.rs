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
