use std::fmt::{Display, Formatter, Octal};
use std::num::ParseIntError;
use std::str::FromStr;
use typescript_type_def::TypeDef;

#[derive(Clone, Debug)]
pub enum Score {
    Infinite,
    Coffee,
    Unknown,
    Number(u8),
}

impl FromStr for Score {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "infinite" => Ok(Score::Infinite),
            "coffee" => Ok(Score::Coffee),
            "unknown" => Ok(Score::Unknown),
            _ => s.parse::<u8>().map(|v| Score::Number(v))
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
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vote {
    pub user_id: String,
    pub score: Score,
}

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
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