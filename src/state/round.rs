use ts_rs::TS;
use super::vote::{Vote, VoteDTO};

#[derive(Clone, Debug)]
pub struct Round {
    pub name: String,
    pub votes: Vec<Vote>,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
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

#[derive(Clone, Debug)]
pub struct CurrentRound {
    pub name: String,
    pub flipped: bool,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct CurrentRoundDTO {
    pub name: String,
    pub flipped: bool,
}

impl From<CurrentRound> for CurrentRoundDTO {
    fn from(value: CurrentRound) -> Self {
        Self {
            name: value.name,
            flipped: value.flipped,
        }
    }
}

impl CurrentRound {
    pub fn new(prior_rounds: usize) -> Self {
        Self {
            flipped: false,
            name: format!("Round #{}", prior_rounds + 1),
        }
    }
}