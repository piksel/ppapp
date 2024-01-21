use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use serde::Serialize;
use socketioxide::socket::Sid;
use std::sync::RwLock;
use tracing::debug;
use uuid::Uuid;

pub mod message;
pub mod user;

pub use message::Message;
pub use user::User;

pub type MessagesStore = HashMap<String, Vec<Message>>;
pub type MembersStore = HashMap<String, Vec<Uuid>>;

#[derive(Default)]
pub struct RoomState {
    pub messages: RwLock<MessagesStore>,
    pub members: RwLock<MembersStore>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    #[serde(rename = "sessionID")]
    pub session_id: Uuid,
    #[serde(rename = "userID")]
    pub user_id: Uuid,
    pub username: String,
    pub connected: bool,
}
impl Session {
    pub fn new(user: &User) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            user_id: user.user_id,
            username: user.username.clone(),
            connected: true,
        }
    }
}

#[derive(Default)]
pub struct Sessions(pub RwLock<HashMap<Uuid, Session>>);
#[derive(Default)]
pub struct Users(pub RwLock<HashMap<Uuid, User>>);

impl RoomState {
    pub async fn insert_message(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().unwrap();
        let messages = binding.entry(room.to_owned()).or_default();
        messages.push(message);
        // messages.truncate(20);
    }

    pub async fn get_messages(&self, room: &str) -> Vec<Message> {
        let messages = self.messages.read().unwrap().get(room).cloned();
        messages.unwrap_or_default().into_iter().rev().collect()
    }

    pub async fn get_members(&self, room: &str) -> Vec<Uuid> {
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
