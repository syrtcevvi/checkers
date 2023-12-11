use serde::{Deserialize, Serialize};

use super::{branch_ser::BranchSer, commit_ser::CommitSer};
use crate::application::structs::Vcs;

#[derive(Debug, Deserialize, Serialize)]
pub struct VcsSer {
    pub commits: Vec<CommitSer>,
    pub branches: Vec<BranchSer>,

    pub current_branch_name: String,
    pub current_commit_id: Option<isize>,
    pub next_commit_id: isize,
}

impl From<Vcs> for VcsSer {
    fn from(value: Vcs) -> Self {
        Self {
            current_branch_name: value.current_branch_name(),
            current_commit_id: value.current_commit.map(|value| value.id),
            next_commit_id: value.next_commit_id,

            commits: value
                .commits
                .into_iter()
                .map(|(_commit_id, commit)| CommitSer::from(commit.as_ref().clone()))
                .collect(),
            branches: value
                .branches
                .into_iter()
                .map(|(_branch_name, branch)| BranchSer::from(branch.borrow().clone()))
                .collect(),
        }
    }
}

impl From<VcsSer> for Vcs {
    fn from(value: VcsSer) -> Self {
        Vcs::default()
        // todo!()
    }
}
