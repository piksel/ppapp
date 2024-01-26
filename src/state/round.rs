// Round


use typescript_type_def::TypeDef;
use uuid::Uuid;
use super::vote::{Vote, VoteDTO};

#[derive(Clone, Debug)]
pub struct Round {
    pub name: String,
    pub votes: Vec<Vote>,
}

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
pub struct RoundDTO {
    pub name: String,
    pub votes: Vec<VoteDTO>,
}

impl From<Round> for RoundDTO {
    fn from(value: Round) -> Self {
        Self {
            votes: value.votes.iter().cloned().map(Into::into).collect(),
            name: value.name,
        }
    }
}