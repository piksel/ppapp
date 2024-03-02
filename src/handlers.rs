use std::collections::HashMap;
use base64::Engine;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};
use std::fmt::{Debug, Display};
use std::future::Future;
use std::hash::Hash;
use tracing::{debug, error, info, info_span, warn};
use uuid::Uuid;

use crate::event::{ClientEvent, MessageIn, ServerEvent, UserIn, VoteIn};
use crate::id::encode_id;
use crate::state::room::{Room, RoomDTO};
use crate::state::round::{CurrentRound, CurrentRoundDTO, RoundDTO, RoundOpts};
use crate::state::user::UserDTO;
use crate::state::vote::VoteDTO;
use crate::state::{Message, RoomState, Round, Session, Sessions, User, Users, Vote};
use crate::{pokemon, state};

pub type EventResult = Result<(), String>;

#[derive(Debug, Deserialize)]
pub struct Auth {
    #[serde(rename = "sessionID")]
    session_id: Option<Uuid>,
    #[serde(rename = "userID")]
    user_id: Option<String>,
}

/// Request/Response Types
#[derive(Debug, Serialize, Clone)]
struct UserConnectedRes {
    #[serde(rename = "userID")]
    user_id: String,
    connected: bool,
    user: UserDTO,
}

impl UserConnectedRes {
    fn new(session: &Session, user: UserDTO) -> Self {
        Self {
            user_id: session.user_id.clone(),
            connected: session.connected,
            user,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PrivateMessageReq {
    to: Uuid,
    content: String,
}

pub async fn on_connection(
    s: SocketRef,
    TryData(auth): TryData<Auth>,
    State(sessions): State<Sessions>,
    State(users_state): State<Users>,
) {
    let span = info_span!("Socket Connection", socket = %s.id);
    let _guard = span.enter();

    if let Err(e) = session_connect(&s, auth, sessions, users_state).await {
        error!("Failed to connect: {:?}", e);
        s.disconnect().ok();
        return;
    }

    s.on(
        ClientEvent::CreateRoom,
        |socket: SocketRef,
         Data::<(String, String)>((room_name, game_name)),
         room_state: State<RoomState>,
         ack_sender: AckSender| async move {
            info!(socket = %socket.id, event = %ClientEvent::CreateRoom, room_name, "Received event");
            ack_result(ack_sender, rooms::handle_create(&socket, room_name, game_name, room_state).await)
        });

    s.on(
        ClientEvent::Join,
        |socket: SocketRef,
         Data::<String>(room_id),
         room_state: State<RoomState>,
         State(Users(users_state)),
         ack_sender: AckSender| async move {
            info!(socket = %socket.id, event = %ClientEvent::Join, room_id, "Received event");
            ack_result(ack_sender, rooms::handle_join(&socket, room_id, room_state, users_state).await);
        },
    );

    s.on(
        ClientEvent::NewRound,
        |s: SocketRef,
         Data::<(String, RoundOpts)>((room, round_opts)),
         room_state: State<RoomState>,
         ack_sender: AckSender| async move {
            info!(socket = %s.id, event = %ClientEvent::NewRound, "Received event");
            ack_result(ack_sender, rounds::handle_new(&s, room, round_opts, room_state).await);
        },
    );

    s.on(
        "message",
        |socket: SocketRef, Data(MessageIn { room, content }), room_state: State<RoomState>| async move {
            let user_id = socket.extensions.get::<Session>().unwrap().user_id.clone();
            let message = Message {
                from: user_id.clone(),
                content,
                date: chrono::Utc::now(),
            };
            room_state.messages.write().await.entry(room.clone()).or_default().push(message.clone());
            emit_within(&socket, room, ServerEvent::Message(&message.into_dto()));
        },
    );

    s.on(
        ClientEvent::UpdateUser,
        |socket: SocketRef, Data(UserIn { name, email }), users_state: State<state::Users>| async move {
            info!(socket = %socket.id, event = %ClientEvent::UpdateUser, name, email, "Received event");
            users::handle_update_user(&socket, name, email, users_state).await;
        },
    );

    s.on(
        ClientEvent::Vote,
        |socket: SocketRef,
         Data(VoteIn { room, score }),
         room_state: State<RoomState>,
         ack_sender: AckSender| async move {
            info!(socket = %socket.id, event = %ClientEvent::Vote, room, score, "Received event");
            ack_result(ack_sender, votes::handle_vote(&socket, room, score, room_state).await);
        },
    );

    s.on(
        ClientEvent::EndVote,
        |socket: SocketRef,
         Data::<String>(room),
         room_state: State<RoomState>,
         ack_sender: AckSender| async move {
            info!(socket = %socket.id, event = %ClientEvent::EndVote, room, "Received event");
            ack_result(ack_sender, votes::handle_end_vote(&socket, room, room_state).await);
        },
    );

    s.on_disconnect(|s: SocketRef, State(Sessions(sessions))| async move {
        handle_disconnect(s, sessions).await;
    });
}

async fn handle_disconnect(s: SocketRef, sessions: &RwLock<HashMap<Uuid, Session>>) {
    let mut session = s.extensions.get::<Session>().unwrap().clone();
    session.connected = false;

    sessions
        .write()
        .await
        .get_mut(&session.session_id)
        .unwrap()
        .connected = false;

    s.broadcast().emit("user disconnected", session).ok();
}

fn emit_reply(socket_ref: &SocketRef, server_event: ServerEvent) {
    let event_id = server_event.event_id();
    if let Err(error) = socket_ref.emit(event_id, server_event) {
        error!(error = debug(error), event_id, "failed to emit reply")
    }
}

fn emit_within(socket_ref: &SocketRef, rooms: impl RoomParam, server_event: ServerEvent) {
    let event_id = server_event.event_id();
    if let Err(error) = socket_ref.within(rooms).emit(event_id, server_event) {
        error!(error = debug(error), event_id, "failed to emit to rooms")
    }
}

fn ack_result<'a, M: AsRef<str>, T: Serialize>(ack_sender: AckSender, result: Result<T, M>) {
    let ack_result = match result {
        Ok(content) => {
            debug!("Sending ack OK");
            AckResult::OK { content }
        }
        Err(message) => {
            let message = message.as_ref();
            warn!(message, "Sending ack error");
            AckResult::Error { error: message.to_string() }
        }
    };
    if let Err(error) = ack_sender.send(ack_result) {
        error!(error = ?error, "Failed to send ack")
    }
}

#[derive(Debug)]
enum ConnectError {
    InvalidUsername,
    EncodeError(serde_json::Error),
    SocketError(socketioxide::SendError),
    BroadcastError(socketioxide::BroadcastError),
}

/// Handles the connection of a new user
async fn session_connect(
    s: &SocketRef,
    auth: Result<Auth, serde_json::Error>,
    Sessions(session_state): &Sessions,
    Users(users_state): &Users,
) -> Result<(), ConnectError> {
    let auth = auth.map_err(ConnectError::EncodeError)?;
    let user = {
        let mut sessions = session_state.write().await;
        if let Some(session) = auth.session_id.and_then(|id| sessions.get_mut(&id)) {
            session.connected = true;
            s.extensions.insert(session.clone());
            users_state
                .read()
                .await
                .get(&session.user_id)
                .unwrap()
                .clone()
        } else {
            let user = {
                let mut users = users_state.write().await;
                {
                    if let Some(user_id) = auth.user_id {
                        if let Some((_, user)) = users.iter().find(|(_, u)| u.user_id == user_id) {
                            Some(user.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                    .unwrap_or_else(|| {
                        let user = User::new(pokemon::random_name());
                        users.insert(user.user_id.clone(), user.clone());
                        user
                    })
            };

            let session = Session::new(&user);
            s.extensions.insert(session.clone());

            sessions.insert(session.session_id, session);
            user
        }
    };

    let session = s.extensions.get::<Session>().unwrap();

    s.join(session.user_id.to_string()).ok();
    s.emit("session", session.clone())
        .map_err(ConnectError::SocketError)?;

    let user_dto: UserDTO = user.into();

    _ = s.emit("user", &user_dto);
    emit_reply(s, ServerEvent::User(&user_dto));
    let res = UserConnectedRes::new(&session, user_dto);

    s.broadcast()
        .emit("user_connected", res)
        .map_err(ConnectError::BroadcastError)?;
    Ok(())
}

fn log_emit_err<E: Debug>(error: E) {
    error!(error = debug(error), "Failed to send emit");
}

use crate::dto::AckResult;
use crate::state::game::Game;
use sha2::{Digest, Sha256};
use socketioxide::operators::RoomParam;
use tokio::sync::RwLock;

mod rounds;
mod votes;
mod rooms;
mod users;

fn hash_email(email: &str) -> String {
    let hash = Sha256::digest(email);
    base16ct::lower::encode_string(&hash)
}
