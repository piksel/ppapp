use std::borrow::Cow;
use std::collections::HashMap;
use socketioxide::socket::Sid;
use uuid::Uuid;
use crate::state::user::UserDTO;
use super::state::{message, user, User};

#[derive(Clone)]
pub enum Event {
    Message, Messages, Join,
    UpdateUser, Users, UserUpdated
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
        })
    }
}


#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    pub room: String,
    pub content: String,
}

#[derive(serde::Serialize)]
pub struct Messages {
    pub messages: Vec<message::MessageDTO>,
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
    pub(crate) update: UserDTO
}