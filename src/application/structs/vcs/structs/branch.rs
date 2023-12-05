use crate::application::structs::vcs::Commit;
use derive_more::Display;
use std::rc::Rc;

#[derive(Debug, Display, Clone)]
#[display(fmt = "{}", name)]
pub struct Branch {
    /// Снимок, на который указывает данная ветка
    ///
    /// Если коммит отсутствует, значит пользователь еще не сделал ни одного
    pub commit: Option<Rc<Commit>>,
    /// Название ветки
    pub name: String,
}

impl Branch {
    pub fn new(name: &str, commit: Option<Rc<Commit>>) -> Self {
        Self {
            commit,
            name: name.to_string(),
        }
    }
}
