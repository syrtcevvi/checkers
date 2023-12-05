mod message;
mod structs;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::application::structs::GameData;
pub use message::Message;
use structs::{Branch, Commit};

/// Версионный контроль состояния доски игры "Шашки"
pub struct Vcs {
    current_branch: Rc<RefCell<Branch>>,
    current_commit: Option<Rc<Commit>>,

    /// Ветки, созданные пользователем. Имя ветки связано с объектом ветки
    branches: HashMap<String, Rc<RefCell<Branch>>>,
    /// Снимки, созданные пользователем. Hash снимка связан с объектом снимка
    commits: HashMap<isize, Rc<Commit>>,
    /// Идентификатор, который будет присвоен следующему снимку
    next_commit_id: isize,
}

impl Vcs {
    const DEFAULT_BRANCH_NAME: &str = "default";
    const DEFAULT_BRANCHES_CAPACITY: usize = 16;
    const DEFAULT_COMMITS_CAPACITY: usize = 64;

    /// Создаёт новую ветку
    pub fn create_branch(&mut self, name: &str) {
        let new_branch = Rc::new(RefCell::new(Branch {
            commit: self.current_commit.clone(),
            name: name.to_string(),
        }));
        self.branches.insert(name.to_string(), new_branch.clone());

        self.current_branch = new_branch;
    }

    /// Проверяет, разрешено ли создание нового снимка
    ///
    /// Создание снимка в центре истории снимков запрещено
    pub fn is_commit_creation_allowed(&self) -> bool {
        todo!()
    }

    /// Создаёт новый снимок
    ///
    /// Параметры:
    /// message - поясняющее сообщение, которое будет храниться вместе со снимком
    /// game_data - состояние игры, которые будут записаны в объект снимка
    pub fn create_commit(&mut self, message: &str, game_data: GameData) {
        let id = self.next_commit_id;
        let commit = Rc::new(Commit {
            id,
            parent_commit: self.current_commit.clone(),
            message: message.to_owned(),
            game_data,
        });

        // После создания нового снимка текущая ветка начинает на него указывать
        self.current_branch.borrow_mut().commit = Some(commit.clone());
        // Добавляем снимок в хранилище снимков
        self.commits.insert(id, commit.clone());
        // Фиксируем, что новый снимок становится текущим
        self.current_commit = Some(commit);

        self.next_commit_id += 1;
    }

    /// Возвращает имя выбранной ветки
    pub fn current_branch_name(&self) -> String {
        self.current_branch.borrow().name.clone()
    }

    /// Возвращает имена существующих веток
    pub fn branch_names(&self) -> Vec<String> {
        self.branches
            .keys()
            .map(|branch_name| branch_name.clone())
            .collect()
    }

    /// Возвращает заголовок текущего снимка
    pub fn current_commit_header(&self) -> Option<String> {
        self.current_commit
            .as_ref()
            .map(|commit| format!("{} - {}", commit.id, commit.message))
    }

    pub fn commits(&self) -> Vec<Commit> {
        todo!()
    }
}

impl Default for Vcs {
    fn default() -> Self {
        let current_branch = Rc::new(RefCell::new(Branch::new(Self::DEFAULT_BRANCH_NAME, None)));
        let mut branches = HashMap::with_capacity(Self::DEFAULT_BRANCHES_CAPACITY);
        branches.insert(current_branch.borrow().name.clone(), current_branch.clone());
        Self {
            current_branch,
            current_commit: None,
            branches,
            commits: HashMap::with_capacity(Self::DEFAULT_COMMITS_CAPACITY),
            next_commit_id: isize::default(),
        }
    }
}
