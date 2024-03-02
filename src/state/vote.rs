use std::fmt::{Display, Formatter, Octal, Pointer, Write, write};
use std::num::ParseIntError;
use std::str::FromStr;
use ts_rs::TS;
use crate::state::game::ParseError;

#[derive(Clone, Debug)]
pub enum Score {
    Infinite,
    Coffee,
    Unknown,
    Number(u8),
    StartIdea(String),
    StopIdea(String),
    ContinueIdea(String),
}

impl FromStr for Score {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u8>() {
            Ok(Score::Number(n))
        } else {
            match (s, s.split_once(": ")) {
                ("infinite", _) => Ok(Score::Infinite),
                ("coffee", _) => Ok(Score::Coffee),
                ("unknown", _) => Ok(Score::Unknown),
                (_, Some(("continue", m))) => Ok(Score::ContinueIdea(m.to_owned())),
                (_, Some(("stop", m))) => Ok(Score::StopIdea(m.to_owned())),
                (_, Some(("start", m))) => Ok(Score::StartIdea(m.to_owned())),
                _ => Err(ParseError::UnknownVariant(s.to_owned())),
            }
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Infinite => f.write_str("infinite"),
            Score::Coffee => f.write_str("coffee"),
            Score::Unknown => f.write_str("unknown"),
            Score::Number(n) => std::fmt::Display::fmt(n, f),
            Score::ContinueIdea(s) => f.write_fmt(format_args!("continue: {s}")),
            Score::StartIdea(s) => f.write_fmt(format_args!("start: {s}")),
            Score::StopIdea(s) => f.write_fmt(format_args!("stop: {s}")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vote {
    pub user_id: String,
    pub score: Score,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct VoteDTO {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub score: String,
}

impl From<Vote> for VoteDTO {
    fn from(value: Vote) -> Self {
        Self {
            score: value.score.to_string(),
            user_id: value.user_id,
        }
    }
}