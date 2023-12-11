use serde::{Deserialize, Serialize};

use crate::application::structs::{vcs::Commit, GameData};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitSer {
    pub id: isize,
    pub parent_commit_id: Option<isize>,
    pub message: String,
    pub game_data: GameData,
}

impl From<Commit> for CommitSer {
    fn from(value: Commit) -> Self {
        CommitSer {
            id: value.id,
            parent_commit_id: value.parent_commit.map(|parent_commit| parent_commit.id),
            message: value.message,
            game_data: value.game_data,
        }
    }
}
