use serde::ser::SerializeTuple;
use serde::Serializer;
use crate::state::message::MessageDTO;
use crate::state::room::RoomDTO;
use crate::state::round::{CurrentRoundDTO, RoundDTO};
use crate::state::user::UserDTO;
use crate::state::vote::VoteDTO;

pub enum ServerEvent<'a> {
    Message(&'a MessageDTO),
    Messages(&'a Vec<MessageDTO>),
    Users(&'a Vec<UserDTO>),
    User(&'a UserDTO),
    UserUpdated(&'a UserDTO),
    Room(&'a RoomDTO),
    Votes(&'a Vec<VoteDTO>),
    Vote(&'a VoteDTO),
    Rounds(&'a Vec<RoundDTO>),
    CurrentRound(&'a CurrentRoundDTO),
}

#[derive(Clone)]
pub enum ClientEvent {
    Join(String),
    UpdateUser(UserDTO),
    Vote(String),
    Reveal(String),
    NewRound(String),
}

impl<'a> serde::Serialize for ServerEvent<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut tup = serializer.serialize_tuple(1).unwrap();
        match self {
            ServerEvent::Message(p) => tup.serialize_element(p),
            ServerEvent::Messages(p) => tup.serialize_element(p),
            ServerEvent::Users(p) => tup.serialize_element(p),
            ServerEvent::UserUpdated(p) => tup.serialize_element(p),
            ServerEvent::CurrentRound(p) => tup.serialize_element(p),
            ServerEvent::Room(p) => tup.serialize_element(p),
            ServerEvent::Votes(p) => tup.serialize_element(p),
            ServerEvent::Rounds(p) => tup.serialize_element(p),
            ServerEvent::User(p) => tup.serialize_element(p),
            ServerEvent::Vote(p) => tup.serialize_element(p),
        }?;
        tup.end()
    }
}

impl<'a> ServerEvent<'a> {
    pub fn event_id(&self) -> &'static str {
        match self {
            ServerEvent::Message(_) => "message",
            ServerEvent::Messages(_) => "messages",
            ServerEvent::Users(_) => "users",
            ServerEvent::UserUpdated(_) => "user updated",
            ServerEvent::CurrentRound(_) => "current round",
            ServerEvent::Room(_) => "room",
            ServerEvent::Votes(_) => "votes",
            ServerEvent::Rounds(_) => "rounds",
            ServerEvent::User(_) => "user",
            ServerEvent::Vote(_) => "vote",
        }
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
    pub messages: Vec<MessageDTO>,
}

#[derive(serde::Serialize)]
pub struct Votes {
    pub votes: Vec<VoteDTO>,
}

#[derive(serde::Serialize)]
pub struct Rounds {
    pub rounds: Vec<RoundDTO>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserIn {
    pub email: String,
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