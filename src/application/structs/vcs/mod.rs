mod message;
mod serialization;
mod structs;

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::application::structs::GameData;
pub use message::Message;
use serialization::VcsSer;
use structs::{Branch, Commit};

/// Версионный контроль состояния доски игры "Шашки"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "VcsSer")]
#[serde(from = "VcsSer")]
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
    pub fn create_branch(&mut self, name: String) {
        let new_branch = Rc::new(RefCell::new(Branch {
            commit: self.current_commit.clone(),
            name: name.clone(),
        }));
        self.branches.insert(name, new_branch.clone());

        self.current_branch = new_branch;
    }

    /// Проверяет, разрешено ли создание нового снимка
    ///
    /// Создание снимка в центре истории снимков запрещено
    pub fn is_commit_creation_allowed(&self) -> bool {
        /*
           Создание снимков разрешено только если на текущий снимок указывает какая-либо из созданных
           веток или если еще не было создано никаких снимков
        */
        let current_commit = self.current_commit.clone();
        if let Some(current_commit) = current_commit {
            self.branches.values().any(|branch| {
                if let Some(commit) = &branch.borrow().commit {
                    commit.id == current_commit.id
                } else {
                    false
                }
            })
        } else {
            true
        }
    }

    /// Создаёт новый снимок
    ///
    /// Параметры:
    /// message - поясняющее сообщение, которое будет храниться вместе со снимком
    /// game_data - состояние игры, которые будут записаны в объект снимка
    pub fn create_commit(&mut self, message: String, game_data: GameData) {
        let id = self.next_commit_id;
        let commit = Rc::new(Commit {
            id,
            parent_commit: self.current_commit.clone(),
            message,
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

    /// Переключается на выбранный снимок
    ///
    /// Возвращает состояние игры на момент создания данного снимка
    pub fn switch_to_commit(&mut self, id: isize) -> GameData {
        self.current_commit = self.commits.get(&id).cloned();

        self.current_commit.as_ref().unwrap().game_data.clone()
    }

    /// Переключается на выбранную ветку
    ///
    /// Возвращает состояние игры. Состояние может отсутстовать, если пользователь не сделал ни одного снимка
    pub fn switch_to_branch(&mut self, name: String) -> Option<GameData> {
        self.current_branch = self.branches.get(&name).unwrap().clone();
        self.current_commit = self.current_branch.borrow().commit.clone();

        self.current_commit
            .as_ref()
            .map(|commit| commit.game_data.clone())
    }

    /// Возвращает заголовок текущего снимка
    pub fn current_commit_header(&self) -> Option<String> {
        self.current_commit
            .as_ref()
            .map(|commit| commit.to_string())
    }

    pub fn commit_headers(&self) -> Vec<String> {
        let mut commit_headers = Vec::with_capacity(Self::DEFAULT_COMMITS_CAPACITY);

        // Строим цепочку коммитов, начиная с выбранного, заканчивая самым первым
        if let Some(commit) = &self.current_branch.borrow().commit {
            self.get_commit_chain(&mut commit_headers, commit)
        }
        commit_headers
    }

    fn get_commit_chain(&self, commit_headers: &mut Vec<String>, commit: &Rc<Commit>) {
        commit_headers.push(commit.to_string());
        if let Some(commit) = &commit.parent_commit.clone() {
            self.get_commit_chain(commit_headers, commit);
        }
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
