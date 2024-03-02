use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;
use ts_rs::TS;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown variant \"{0}\"")]
    UnknownVariant(String)
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub enum Game {
    Effort,
    Retro,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "effort" => Ok(Game::Effort),
            "retro" => Ok(Game::Retro),
            _ => Err(ParseError::UnknownVariant(s.into()))
        }
    }
}