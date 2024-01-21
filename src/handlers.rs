use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, State, TryData};
use tracing::{debug, error, info};
use uuid::Uuid;
use crate::{event, state};
use crate::event::{Event, MessageIn, Users};

use crate::state::{Message, RoomState, Session, Sessions, User};
use crate::state::user::UserDTO;

#[derive(Debug, Deserialize)]
pub struct Auth {
    #[serde(rename = "sessionID")]
    session_id: Option<Uuid>,
    username: Option<String>,
}

/// Request/Response Types
#[derive(Debug, Serialize, Clone)]
struct UserConnectedRes {
    #[serde(rename = "userID")]
    user_id: Uuid,
    username: String,
    connected: bool,
    // messages: Vec<Message>,
}

impl UserConnectedRes {
    fn new(session: &Session) -> Self {
        Self {
            user_id: session.user_id,
            username: session.username.clone(),
            connected: session.connected,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct PrivateMessageReq {
    to: Uuid,
    content: String,
}

pub fn on_connection(
    s: SocketRef,
    TryData(auth): TryData<Auth>,
    sessions: State<Sessions>,
    users: State<state::Users>,
    room_state: State<RoomState>,
) {
    if let Err(e) = session_connect(&s, auth, sessions.0, users.0, room_state.0) {
        error!("Failed to connect: {:?}", e);
        s.disconnect().ok();
        return;
    }

    s.on(
        "join",
        |socket: SocketRef, Data::<String>(room), room_state: State<RoomState>, State(state::Users(users_state))| async move {
            let user_id =  socket.extensions.get::<Session>().unwrap().user_id;

            info!(room, "Received join");
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());

            // let State((state::Users(users_state), room_state)) = state;
            // let State(room_state) = state;

            // debug!(id = debug(socket.id), "Getting user...");
            // store.get_user(&socket.id).await;

            let members = {

                let mut members = room_state.members.write().unwrap();
                let room_members = members.entry(room.clone()).or_default();
                room_members.push(user_id);
                room_members.clone()
            };


            let users: Vec<UserDTO> = {
                let users = users_state.read().unwrap();
                members.iter().map(|id| users.get(id).cloned().unwrap_or_else(|| User::from(id)))
                    .map(|u| u.into()).collect()
            };
            debug!(count = users.len(), "Sending users...");
            let _ = socket.emit(Event::Users, event::Users{ users });

            let messages: Vec<_> = room_state.get_messages(&room).await.iter().map(|m| m.as_dto()).collect();
            debug!(count = messages.len(), "Sending messages...");
            let _ = socket.emit(Event::Messages, event::Messages { messages });
        },
    );

    s.on(
        "message",
        |s: SocketRef, Data(MessageIn { room, content }), room_state: State<RoomState>| async move {
            let user_id = s.extensions.get::<Session>().unwrap().user_id;
            let message = Message {
                from: user_id,
                content,
                date: chrono::Utc::now(),
            };
            room_state.messages.write().unwrap().entry(room.clone()).or_default().push(message.clone());
            s.within(room)
                .emit("message", message.into_dto())
                .ok();
        },
    );

    s.on_disconnect(|s: SocketRef, State(Sessions(sessions))| async move {
        let mut session = s.extensions.get::<Session>().unwrap().clone();
        session.connected = false;

        sessions
            .write()
            .unwrap()
            .get_mut(&session.session_id)
            .unwrap()
            .connected = false;

        s.broadcast().emit("user disconnected", session).ok();
    });
}

#[derive(Debug)]
enum ConnectError {
    InvalidUsername,
    EncodeError(serde_json::Error),
    SocketError(socketioxide::SendError),
    BroadcastError(socketioxide::BroadcastError),
}

/// Handles the connection of a new user
fn session_connect(
    s: &SocketRef,
    auth: Result<Auth, serde_json::Error>,
    Sessions(session_state): &Sessions,
    state::Users(users_state): &state::Users,
    room_state: &RoomState,
) -> Result<(), ConnectError> {
    let auth = auth.map_err(ConnectError::EncodeError)?;
    {
        let mut sessions = session_state.write().unwrap();
        if let Some(session) = auth.session_id.and_then(|id| sessions.get_mut(&id)) {
            session.connected = true;
            s.extensions.insert(session.clone());
        } else {
            let username = auth.username.ok_or(ConnectError::InvalidUsername)?;

            let user = {
                let mut users = users_state.write().unwrap();
                match users.iter().find(|(_, u)| u.username == username) {
                    Some((_, user)) => user.clone(),
                       None => {
                           let user = User::new(username);
                           users.insert(user.user_id, user.clone());
                           user
                       }
                }
            };

            let session = Session::new(&user);
            s.extensions.insert(session.clone());

            sessions.insert(session.session_id, session);
        };
    }

    let session = s.extensions.get::<Session>().unwrap();

    s.join(session.user_id.to_string()).ok();
    s.emit("session", session.clone())
        .map_err(ConnectError::SocketError)?;

    // let users = session_states
    //     .read()
    //     .unwrap()
    //     .iter()
    //     .filter(|(id, _)| id != &&session.session_id)
    //     .map(|(_, session)| {
    //         // let messages = room_state.messages
    //         //     .read()
    //         //     .await
    //         //
    //         //     .iter()
    //         //     .filter(|message| message.to == session.user_id || message.from == session.user_id)
    //         //     .cloned()
    //         //     .collect();
    //
    //         UserConnectedRes::new(session)
    //     })
    //     .collect::<Vec<_>>();

    // s.emit("users", [users])
    //     .map_err(ConnectError::SocketError)?;

    let res = UserConnectedRes::new(&session);

    s.broadcast()
        .emit("user_connected", res)
        .map_err(ConnectError::BroadcastError)?;
    Ok(())
}