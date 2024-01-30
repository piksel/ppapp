use std::fmt::{Debug, Display};
use std::hash::Hash;
use base64::Engine;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, State, AckSender, TryData};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{pokemon, state};
use crate::event::{ServerEvent, MessageIn, UserIn, VoteIn};
use crate::id::encode_id;
use crate::state::{Message, RoomState, Round, Session, Sessions, User, Vote};
use crate::state::room::{Room, RoomDTO};
use crate::state::round::{CurrentRound, CurrentRoundDTO, RoundDTO};
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

// async fn on_connect(socket: SocketRef) {
pub async fn on_connection(
    s: SocketRef,
    TryData(auth): TryData<Auth>,
    State(sessions): State<Sessions>,
    State(users): State<state::Users>,
) {
    if let Err(e) = session_connect(&s, auth, sessions, users).await {
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

            room_state.rooms.write().await.insert(room_info.room_id.clone(), room_info.clone());

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
                room_state.rooms.write().await.entry(room_id.clone())
                    .or_insert(Room { room_id: room_id.clone(), name: room_id.clone() }).clone().into();

            let members = {
                let mut members = room_state.members.write().await;
                let room_members = members.entry(room_id.clone()).or_default();
                room_members.insert(user_id.clone());
                room_members.clone()
            };


            let users: Vec<UserDTO> = {
                let users = users_state.read().await;
                members.iter().map(|id| users.get(id).cloned().unwrap_or_else(|| User::new(pokemon::random_name())))
                    .map(|u| u.into()).collect()
            };


            let rounds: Vec<_> = room_state.get_rounds(&room_id).await.into_iter().map(Into::into).collect();


            let current_round: CurrentRoundDTO =
                room_state.current_round.write().await.entry(room_id.clone())
                    .or_insert(CurrentRound::new(rounds.len())).clone().into();
            debug!(name = &current_round.name, "Sending current round...");
            socket.emit("current round", current_round).unwrap_or_else(log_emit_err);

            debug!(count = rounds.len(), "Sending rounds...");
            emit_reply(&socket, ServerEvent::Rounds(&rounds));

            let votes: Vec<_> = room_state.get_votes(&room_id).await.into_iter().map(Into::into).collect();
            debug!(count = votes.len(), "Sending votes...");
            emit_reply(&socket, ServerEvent::Votes(&votes));

            debug!(room_info = room_info.name, "Sending room info...");
            emit_reply(&socket, ServerEvent::Room(&room_info));

            debug!(count = users.len(), "Sending users...");
            emit_within(&socket, room_id, ServerEvent::Users(&users));
        },
    );

    s.on(
        "new round",
        |s: SocketRef, Data::<String>(room), room_state: State<RoomState>, ack_sender: AckSender| async move {
            if !room_state.current_round.read().await.get(&room).unwrap().flipped {
                ack_error(ack_sender, "the current round is not done");
                return;
            }

            let votes = {
                let mut room_votes = room_state.votes.write().await;
                let mut votes = room_votes.entry(room.clone()).or_default();
                let round_votes: Vec<Vote> = votes.values().cloned().collect();
                votes.clear();
                round_votes
            };


            let (rounds, current_round): (Vec<RoundDTO>, CurrentRoundDTO) = {
                let mut room_rounds = room_state.rounds.write().await;
                let mut rounds = room_rounds.entry(room.clone()).or_default();

                let mut current_rounds = room_state.current_round.write().await;
                let round_count = rounds.len() + match current_rounds.get(&room) {
                    Some(_) => 1,
                    None => 0,
                };
                let current_round = CurrentRound::new(round_count);
                if let Some(prev_round) = current_rounds.insert(room.clone(), current_round.clone()) {
                    rounds.push(Round {
                        votes,
                        name: prev_round.name,
                    });
                }

                (rounds.iter().cloned().map(Into::into).collect(), current_round.clone().into())
            };

            emit_within(&s, room.clone(), ServerEvent::Rounds(&rounds));
            emit_within(&s, room.clone(), ServerEvent::Votes(&vec![]));
            emit_within(&s, room.clone(), ServerEvent::CurrentRound(&current_round));
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
        "update user",
        |s: SocketRef, Data(UserIn { name, email }), users_state: State<state::Users>| async move {
            let user_id = &s.extensions.get::<Session>().unwrap().user_id.clone();
            let user = {
                let mut users = users_state.0.0.write().await;
                let user = users.get_mut(user_id).unwrap();
                user.name = name;
                user.avatar = hash_email(&email);
                user.email = email;
                user.clone()
            };
            let dto: UserDTO = user.into();
            emit_reply(&s, ServerEvent::User(&dto));
            let dto = {
                let mut dto = dto;
                dto.email = "".to_string();
                dto
            };
            for room in s.rooms().unwrap().iter() {
                emit_within(&s, room.clone(), ServerEvent::UserUpdated(&dto));
            }
        },
    );

    s.on(
        "vote",
        |s: SocketRef, Data(VoteIn { room, score }), room_state: State<RoomState>, ack_sender: AckSender| async move {
            let user_id = s.extensions.get::<Session>().unwrap().user_id.clone();

            let score = match score.parse() {
                Ok(score) => score,
                Err(error) => {
                    ack_error(ack_sender, &format!("{error:?}"));
                    return;
                }
            };

            let vote = Vote { user_id: user_id.clone(), score };
            {
                room_state.votes.write().await.entry(room.clone()).or_default().insert(user_id.clone(), vote.clone());
            }
            let dto: VoteDTO = vote.into();
            emit_reply(&s, ServerEvent::Vote(&dto));

            let votes: Vec<VoteDTO> = room_state.votes.read().await.get(&room).unwrap()
                .values().cloned()
                .map(Into::into)
                .collect();
            emit_within(&s, room, ServerEvent::Votes(&votes));
        },
    );

    s.on(
        "end vote",
        |socket: SocketRef, Data::<String>(room), room_state: State<RoomState>, ack_sender: AckSender| async move {
            {
                let votes_state = room_state.votes.read().await;
                let Some(votes) = votes_state.get(&room) else {
                    ack_error(ack_sender, "no votes for round");
                    return;
                };
                let members: Vec<_> = room_state.members.read().await.get(&room).unwrap().iter().cloned().collect();
                if members.iter().any(|m| votes.get(m).is_none()) {
                    ack_sender.send(AckResult::error("not every member has voted")).ok();
                    return;
                }
            }

            let current_round: CurrentRoundDTO = {
                let mut current_round_state = room_state.current_round.write().await;
                let Some(current_round) = current_round_state.get_mut(&room) else {
                    ack_error(ack_sender, "no votes in round");
                    return;
                };
                current_round.flipped = true;
                current_round.clone().into()
            };

            ack_sender.send(AckResult::OK).unwrap_or_else(log_emit_err);
            emit_within(&socket, room, ServerEvent::CurrentRound(&current_round));
        },
    );

    s.on_disconnect(|s: SocketRef, State(Sessions(sessions))| async move {
        let mut session = s.extensions.get::<Session>().unwrap().clone();
        session.connected = false;

        sessions
            .write()
            .await
            .get_mut(&session.session_id)
            .unwrap()
            .connected = false;

        s.broadcast().emit("user disconnected", session).ok();
    });
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

fn ack_error(ack_sender: AckSender, message: &str) {
    if let Err(error) = ack_sender.send(AckResult::error(message)) {
        error!(error = debug(error), "Failed to send ack error")
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
    state::Users(users_state): &state::Users,
) -> Result<(), ConnectError> {
    let auth = auth.map_err(ConnectError::EncodeError)?;
    let user = {
        let mut sessions = session_state.write().await;
        if let Some(session) = auth.session_id.and_then(|id| sessions.get_mut(&id)) {
            session.connected = true;
            s.extensions.insert(session.clone());
            users_state.read().await.get(&session.user_id).unwrap().clone()
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

use sha2::{Sha256, Digest};
use socketioxide::operators::RoomParam;
use crate::dto::AckResult;

fn hash_email(email: &str) -> String {
    let hash = Sha256::digest(email);
    base16ct::lower::encode_string(&hash)
}