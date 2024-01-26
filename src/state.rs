use std::collections::{BTreeSet, HashMap, VecDeque};
use std::collections::hash_map::Entry;
use serde::Serialize;
use socketioxide::socket::Sid;
use std::sync::RwLock;
use tracing::debug;
use uuid::Uuid;

pub mod message;
pub mod user;
pub mod room;
pub mod round;
pub mod vote;

pub use message::Message;
pub use user::User;
pub use room::Room;
pub use round::Round;
pub use vote::Vote;

pub type MessagesStore = HashMap<String, Vec<Message>>;
pub type MembersStore = HashMap<String, BTreeSet<String>>;
pub type RoomsStore = HashMap<String, Room>;
pub type RoundsStore = HashMap<String, Vec<Round>>;
pub type VotesStore = HashMap<String, HashMap<String, Vote>>;

#[derive(Default)]
pub struct RoomState {
    pub rooms: RwLock<RoomsStore>,
    pub messages: RwLock<MessagesStore>,
    pub members: RwLock<MembersStore>,
    pub rounds: RwLock<RoundsStore>,
    pub votes: RwLock<VotesStore>,
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
        let mut binding = self.messages.write().unwrap();
        let messages = binding.entry(room.to_owned()).or_default();
        messages.push(message);
        // messages.truncate(20);
    }

    pub async fn get_rounds(&self, room: &str) -> Vec<Round> {
        let rounds = self.rounds.read().unwrap().get(room).cloned();
        rounds.unwrap_or_default().into_iter().collect()
    }

    pub async fn get_votes(&self, room: &str) -> Vec<Vote> {
        let votes = self.votes.read().unwrap().get(room).cloned();
        votes.unwrap_or_default().values().cloned().collect()
    }

    pub async fn get_room_info(&self, room_id: &str) -> Room {
        self.rooms.read().unwrap().get(room_id).unwrap().clone()
    }

    pub async fn get_members(&self, room: &str) -> Vec<String> {
        let members = self.members.read().unwrap().get(room).cloned();
        members.unwrap_or_default().into_iter().rev().collect()
    }

    // pub async fn insert_user(&self, room: &str, message: Message) {
    //     let mut binding = self.messages.write().await;
    //     let messages = binding.entry(room.to_owned()).or_default();
    //     messages.push_front(message);
    //     messages.truncate(20);
    // }

    // pub async fn update_user(&self, sid: &Sid, user: &User) {
    //     self.users.write().await.insert(sid.clone(), user.clone());
    // }
    //
    // pub async fn get_users(&self) -> HashMap<Sid, User> {
    //     self.users.read().await.clone()
    // }
    //
    // pub async fn get_user(&self, sid: &Sid) -> User {
    //     if let Some(user) = self.users.read().await.get(sid) {
    //             debug!("Found existing user");
    //             return user.clone()
    //     }
    //
    //     debug!("Creating new user...");
    //     return self.users.write().await.entry(sid.clone()).or_insert_with(|| User::from(sid)).clone()
    // }
}
