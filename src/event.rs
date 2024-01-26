use std::borrow::Cow;
use std::collections::HashMap;
use socketioxide::socket::Sid;
use uuid::Uuid;
use crate::state::user::UserDTO;
use super::state::{message, user, User, vote, round};

#[derive(Clone)]
pub enum Event {
    Message,
    Messages,
    Join,
    UpdateUser,
    Users,
    UserUpdated,
    Room,
    Votes,
    Rounds,
}

impl Into<Cow<'static, str>> for Event {
    fn into(self) -> Cow<'static, str> {
        Cow::from(match self {
            Event::Message => "message",
            Event::Messages => "messages",
            Event::UpdateUser => "user",
            Event::Users => "users",
            Event::UserUpdated => "user_update",
            Event::Join => "join",
            Event::Room => "room",
            Event::Votes => "votes",
            Event::Rounds => "rounds",
        })
    }
}


#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    pub room: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct VoteIn {
    pub room: String,
    pub score: String,
}

#[derive(serde::Serialize)]
pub struct Messages {
    pub messages: Vec<message::MessageDTO>,
}

#[derive(serde::Serialize)]
pub struct Votes {
    pub votes: Vec<vote::VoteDTO>,
}

#[derive(serde::Serialize)]
pub struct Rounds {
    pub rounds: Vec<round::RoundDTO>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserIn {
    pub room: String,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct Users {
    pub users: Vec<UserDTO>,
}

#[derive(serde::Serialize)]
pub(crate) struct EntityUpdate {
    pub(crate) id: String,
    #[serde(rename(serialize = "type"))]
    pub(crate) entity_type: &'static str,
    pub(crate) update: UserDTO,
}