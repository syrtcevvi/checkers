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

mod message;
mod structs;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::structs::GameData;
pub use message::Message;
use structs::{Branch, Commit};

/// Версионный контроль состояния доски игры "Шашки"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vcs {
    current_branch_name: String,
    current_commit_id: Option<isize>,

    /// Ветки, созданные пользователем. Имя ветки связано с объектом ветки
    branches: HashMap<String, Branch>,
    /// Снимки, созданные пользователем. Hash снимка связан с объектом снимка
    commits: HashMap<isize, Commit>,
    /// Идентификатор, который будет присвоен следующему снимку
    next_commit_id: isize,
}

impl Vcs {
    const DEFAULT_BRANCH_NAME: &str = "default";
    const DEFAULT_BRANCHES_CAPACITY: usize = 16;
    const DEFAULT_COMMITS_CAPACITY: usize = 64;

    /// Возвращает состояние игры для текущего снимка, если такой был создан
    pub fn get_current_state(&self) -> Option<GameData> {
        self.get_current_commit()
            .map(|commit| commit.game_data.clone())
    }

    /// Создаёт новую ветку
    pub fn create_branch(&mut self, name: String) {
        let new_branch = Branch {
            commit_id: self.current_commit_id,
            name: name.clone(),
        };
        self.branches.insert(name.clone(), new_branch);
        self.current_branch_name = name;
    }

    /// Проверяет, разрешено ли создание нового снимка
    ///
    /// Создание снимка в центре истории снимков запрещено
    pub fn is_commit_creation_allowed(&self) -> bool {
        /*
           Создание снимков разрешено только если на текущий снимок указывает какая-либо из созданных
           веток или если еще не было создано никаких снимков
        */
        self.branches
            .values()
            .any(|branch| branch.commit_id == self.current_commit_id)
    }

    /// Создаёт новый снимок
    ///
    /// Параметры:
    /// message - поясняющее сообщение, которое будет храниться вместе со снимком
    /// game_data - состояние игры, которые будут записаны в объект снимка
    pub fn create_commit(&mut self, message: String, game_data: GameData) {
        let id = self.next_commit_id;
        let commit = Commit {
            id,
            parent_commit_id: self.current_commit_id,
            message,
            game_data,
        };
        self.get_current_branch_mut().commit_id = Some(commit.id);
        self.current_commit_id = Some(commit.id);
        self.commits.insert(commit.id, commit);

        self.next_commit_id += 1;
    }

    /// Возвращает имя выбранной ветки
    pub fn current_branch_name(&self) -> String {
        self.get_current_branch().name.clone()
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
        self.current_commit_id = Some(id);
        self.get_current_commit().unwrap().game_data.clone()
    }

    /// Переключается на выбранную ветку
    ///
    /// Возвращает состояние игры. Состояние может отсутстовать, если пользователь не сделал ни одного снимка
    pub fn switch_to_branch(&mut self, name: String) -> Option<GameData> {
        self.current_branch_name = name;
        self.current_commit_id = self.get_current_branch().commit_id;
        self.get_current_commit()
            .map(|commit| commit.game_data.clone())
    }

    /// Возвращает заголовок текущего снимка
    pub fn current_commit_header(&self) -> Option<String> {
        self.get_current_commit().map(|commit| commit.to_string())
    }

    pub fn commit_headers(&self) -> Vec<String> {
        let mut commit_headers = Vec::with_capacity(Self::DEFAULT_COMMITS_CAPACITY);

        // Строим цепочку коммитов, начиная с выбранного, заканчивая самым первым
        if let Some(commit_id) = self.get_current_branch().commit_id {
            self.get_commit_chain(&mut commit_headers, commit_id);
        }
        commit_headers
    }

    fn get_commit_chain(&self, commit_headers: &mut Vec<String>, commit_id: isize) {
        let commit = self.get_commit(commit_id);
        commit_headers.push(commit.to_string());
        if let Some(parent_commit_id) = commit.parent_commit_id {
            self.get_commit_chain(commit_headers, parent_commit_id);
        }
    }

    /// Возвращает ссылку на неизменяемый объект снимка
    fn get_commit(&self, commit_id: isize) -> &Commit {
        self.commits.get(&commit_id).unwrap()
    }

    /// Возвращает ссылку на неизменяемый объект текущего снимка
    fn get_current_commit(&self) -> Option<&Commit> {
        self.current_commit_id
            .map(|commit_id| self.commits.get(&commit_id).unwrap())
    }

    /// Возвращает ссылку на неизменяемый объект текущей ветки
    fn get_current_branch(&self) -> &Branch {
        self.branches.get(&self.current_branch_name).unwrap()
    }

    /// Возвращает ссылку на изменяемый объект текущей ветки
    fn get_current_branch_mut(&mut self) -> &mut Branch {
        self.branches.get_mut(&self.current_branch_name).unwrap()
    }
}

impl Default for Vcs {
    fn default() -> Self {
        let current_branch = Branch::new(Self::DEFAULT_BRANCH_NAME, None);
        let mut branches = HashMap::with_capacity(Self::DEFAULT_BRANCHES_CAPACITY);
        branches.insert(current_branch.name.clone(), current_branch.clone());
        Self {
            current_branch_name: Self::DEFAULT_BRANCH_NAME.to_string(),
            current_commit_id: None,
            branches,
            commits: HashMap::with_capacity(Self::DEFAULT_COMMITS_CAPACITY),
            next_commit_id: isize::default(),
        }
    }
}
