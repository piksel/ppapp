use std::collections::HashMap;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, State, TryData, AckSender};
use tracing::{debug, error, info};
use uuid::{Bytes, Uuid};
use crate::{event, pokemon, state};
use crate::event::{Event, MessageIn, Users, VoteIn};
use crate::id::{decode_id, encode_id};

use crate::state::{Message, RoomState, Round, Session, Sessions, User, Vote};
use crate::state::room::{Room, RoomDTO};
use crate::state::round::RoundDTO;
use crate::state::user::UserDTO;
use crate::state::vote::VoteDTO;

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
        "create room",
        |socket: SocketRef, Data::<String>(room_name), room_state: State<RoomState>, ack_sender: AckSender| async move {
            let room_id = encode_id(&Uuid::new_v4());
            let room_info = Room {
                room_id,
                name: room_name,
            };

            room_state.rooms.write().unwrap().insert(room_info.room_id.clone(), room_info.clone());

            ack_sender.send(RoomDTO::from(room_info)).ok();
        });

    s.on(
        "join",
        |socket: SocketRef, Data::<String>(room_id), room_state: State<RoomState>, State(state::Users(users_state))| async move {
            let user_id = socket.extensions.get::<Session>().unwrap().user_id.clone();

            info!(room_id, "Received join");
            let _ = socket.leave_all();
            let _ = socket.join(room_id.clone());
            let room_info: RoomDTO =
                room_state.rooms.write().unwrap().entry(room_id.clone())
                    .or_insert(Room { room_id: room_id.clone(), name: room_id.clone() }).clone().into();

            // let State((state::Users(users_state), room_state)) = state;
            // let State(room_state) = state;

            // debug!(id = debug(socket.id), "Getting user...");
            // store.get_user(&socket.id).await;

            let members = {
                let mut members = room_state.members.write().unwrap();
                let room_members = members.entry(room_id.clone()).or_default();
                room_members.insert(user_id.clone());
                room_members.clone()
            };


            let users: Vec<UserDTO> = {
                let users = users_state.read().unwrap();
                members.iter().map(|id| users.get(id).cloned().unwrap_or_else(|| User::new(pokemon::random_name())))
                    .map(|u| u.into()).collect()
            };
            debug!(count = users.len(), "Sending users...");
            let _ = socket.emit(Event::Users, event::Users { users });

            let rounds: Vec<_> = room_state.get_rounds(&room_id).await.into_iter().map(Into::into).collect();
            debug!(count = rounds.len(), "Sending rounds...");
            let _ = socket.emit(Event::Rounds, event::Rounds { rounds });

            let votes: Vec<_> = room_state.get_votes(&room_id).await.into_iter().map(Into::into).collect();
            debug!(count = votes.len(), "Sending votes...");
            let _ = socket.emit(Event::Votes, event::Votes { votes });

            // let room_info: RoomDTO = room_state.get_room_info(&room_id);
            debug!(room_info = room_info.name, "Sending room info...");
            let _ = socket.emit(Event::Room, room_info);
        },
    );

    s.on(
        "new round",
        |s: SocketRef, Data::<String>(room), room_state: State<RoomState>| async move {
            let votes = {
                let mut room_votes = room_state.votes.write().unwrap();
                let mut votes = room_votes.entry(room.clone()).or_default();
                let round_votes: Vec<Vote> = votes.values().cloned().collect();
                votes.clear();
                round_votes
            };


            let rounds: Vec<RoundDTO> = {
                let mut room_rounds = room_state.rounds.write().unwrap();
                let mut rounds = room_rounds.entry(room.clone()).or_default();
                let round = Round {
                    votes,
                    name: format!("Round #{}", rounds.len()),
                };
                rounds.push(round);
                rounds.iter().cloned().map(Into::into).collect()
            };

            s.within(room.clone())
                .emit("rounds", event::Rounds { rounds })
                .ok();

            s.within(room)
                .emit("new round", ())
                .ok();
        },
    );

    s.on(
        "message",
        |s: SocketRef, Data(MessageIn { room, content }), room_state: State<RoomState>| async move {
            let user_id = &s.extensions.get::<Session>().unwrap().user_id;
            let message = Message {
                from: user_id.clone(),
                content,
                date: chrono::Utc::now(),
            };
            room_state.messages.write().unwrap().entry(room.clone()).or_default().push(message.clone());
            s.within(room)
                .emit("message", message.into_dto())
                .ok();
        },
    );

    s.on(
        "vote",
        |s: SocketRef, Data(VoteIn { room, score }), room_state: State<RoomState>| async move {
            let user_id = &s.extensions.get::<Session>().unwrap().user_id;

            let Ok(score) = score.parse() else {
                return;
            };
            let vote = Vote { user_id: user_id.clone(), score };
            {
                room_state.votes.write().unwrap().entry(room.clone()).or_default().insert(user_id.clone(), vote.clone());
            }
            let dto: VoteDTO = vote.into();
            s.emit("vote", dto).ok();

            let votes: Vec<VoteDTO> = room_state.votes.read().unwrap().get(&room).unwrap()
                .values().cloned()
                .map(Into::into)
                .collect();
            s.within(room).emit("votes", event::Votes { votes }).ok();
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
    let user = {
        let mut sessions = session_state.write().unwrap();
        if let Some(session) = auth.session_id.and_then(|id| sessions.get_mut(&id)) {
            session.connected = true;
            s.extensions.insert(session.clone());
            users_state.read().unwrap().get(&session.user_id).unwrap().clone()
        } else {
            let user = {
                let mut users = users_state.write().unwrap();
                {
                    if let Some(user_id) = auth.user_id {
                        if let Some((_, user)) = users.iter().find(|(_, u)| u.user_id == user_id) {
                            Some(user.clone())
                        } else {
                            None
                        }
                    } else { None }
                }.unwrap_or_else(|| {
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

    let userDto: UserDTO = user.into();

    _ = s.emit("user", &userDto);

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

    let res = UserConnectedRes::new(&session, userDto);

    s.broadcast()
        .emit("user_connected", res)
        .map_err(ConnectError::BroadcastError)?;
    Ok(())
}