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
    pub candidates: Vec<String>,
    pub max_votes: u8,
    pub anonymous: bool,
    pub round_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct RoundOpts {
    pub candidates: Vec<String>,
    pub max_votes: u8,
    pub anonymous: bool,
    pub round_type: String,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct CurrentRoundDTO {
    pub name: String,
    pub flipped: bool,
    pub candidates: Vec<String>,
    pub max_votes: u8,
    pub anonymous: bool,
    pub round_type: String,
}

impl From<CurrentRound> for CurrentRoundDTO {
    fn from(value: CurrentRound) -> Self {
        Self {
            name: value.name,
            flipped: value.flipped,
            candidates: value.candidates,
            max_votes: value.max_votes,
            anonymous: value.anonymous,
            round_type: value.round_type,
        }
    }
}

impl CurrentRound {
    pub fn new(prior_rounds: usize, round_opts: RoundOpts) -> Self {
        let RoundOpts { candidates, max_votes, anonymous, round_type } = round_opts;
        Self {
            flipped: false,
            name: format!("Round #{}", prior_rounds + 1),
            candidates,
            max_votes,
            anonymous,
            round_type,
        }
    }
}