use std::collections::{BTreeSet, HashMap};
use serde::Serialize;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod message;
pub mod user;
pub mod room;
pub mod round;
pub mod vote;
pub mod game;

pub use message::Message;
pub use user::User;
pub use room::Room;
pub use round::Round;
pub use vote::Vote;
use crate::state::round::CurrentRound;

pub type MessagesStore = HashMap<String, Vec<Message>>;
pub type MembersStore = HashMap<String, BTreeSet<String>>;
pub type RoomsStore = HashMap<String, Room>;
pub type RoundsStore = HashMap<String, Vec<Round>>;
pub type CurrentRoundStore = HashMap<String, CurrentRound>;
pub type VotesStore = HashMap<String, HashMap<String, Vote>>;


#[derive(Default)]
pub struct RoomState {
    pub rooms: RwLock<RoomsStore>,
    pub messages: RwLock<MessagesStore>,
    pub members: RwLock<MembersStore>,
    pub rounds: RwLock<RoundsStore>,
    pub votes: RwLock<VotesStore>,
    pub current_round: RwLock<CurrentRoundStore>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    #[serde(rename = "sessionID")]
    pub session_id: Uuid,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub connected: bool,
}

impl Session {
    pub fn new(user: &User) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            user_id: user.user_id.clone(),
            connected: true,
        }
    }
}

#[derive(Default)]
pub struct Sessions(pub RwLock<HashMap<Uuid, Session>>);

#[derive(Default)]
pub struct Users(pub RwLock<HashMap<String, User>>);

impl RoomState {
    pub async fn insert_message(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_owned()).or_default();
        messages.push(message);
    }

    pub async fn get_rounds(&self, room: &str) -> Vec<Round> {
        let rounds = self.rounds.read().await.get(room).cloned();
        rounds.unwrap_or_default().into_iter().collect()
    }

    pub async fn get_votes(&self, room: &str) -> Vec<Vote> {
        let votes = self.votes.read().await.get(room).cloned();
        votes.unwrap_or_default().values().cloned().collect()
    }

    pub async fn get_room_info(&self, room_id: &str) -> Room {
        self.rooms.read().await.get(room_id).unwrap().clone()
    }

    pub async fn get_members(&self, room: &str) -> Vec<String> {
        let members = self.members.read().await.get(room).cloned();
        members.unwrap_or_default().into_iter().rev().collect()
    }
}
