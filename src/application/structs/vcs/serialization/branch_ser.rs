use serde::{Deserialize, Serialize};

use crate::application::structs::vcs::Branch;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchSer {
    pub commit_id: Option<isize>,
    pub name: String,
}

impl From<Branch> for BranchSer {
    fn from(value: Branch) -> Self {
        BranchSer {
            commit_id: value.commit.map(|commit| commit.id),
            name: value.name,
        }
    }
}
