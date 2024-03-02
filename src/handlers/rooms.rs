use socketioxide::extract::{SocketRef, State};
use tokio::sync::RwLock;
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;
use crate::{handlers, pokemon};
use crate::event::ServerEvent;
use crate::id::encode_id;
use crate::state::{Room, RoomState, Session, User};
use crate::state::game::Game;
use crate::state::room::RoomDTO;
use crate::state::round::CurrentRoundDTO;
use crate::state::user::UserDTO;

pub async fn handle_create(_: &SocketRef, room_name: String, game_name: String,
                           room_state: State<RoomState>) -> Result<RoomDTO, String> {
    let room_id = encode_id(&Uuid::new_v4());


    let game: Game = {
        match (&game_name).parse() {
            Err(e) => {
                return Err(format!("{e:?}"));
            }
            Ok(game) => game
        }
    };

    let room_info = Room {
        room_id,
        name: room_name,
        game,
    };


    room_state.rooms.write().await.insert(room_info.room_id.clone(), room_info.clone());

    Ok(RoomDTO::from(room_info))
}

pub async fn handle_join(socket: &SocketRef, room_id: String, room_state: State<RoomState>,
                         users_state: &RwLock<HashMap<String, User>>) -> Result<(), String> {
    let user_id = socket.extensions.get::<Session>().unwrap().user_id.clone();

    let room_info: RoomDTO = {
        let rooms = room_state.rooms.write().await;
        let Some(room) = rooms.get(&room_id) else {
            return Err(format!("room with ID \"{room_id}\" could not be found"));
        };
        room.clone().into()
    };

    let _ = socket.leave_all();
    let _ = socket.join(room_id.clone());

    let members = {
        let mut members = room_state.members.write().await;
        let room_members = members.entry(room_id.clone()).or_default();
        room_members.insert(user_id.clone());
        room_members.clone()
    };

    let users: Vec<UserDTO> = {
        let users = users_state.read().await;
        members
            .iter()
            .map(|id| {
                users
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| User::new(pokemon::random_name()))
            })
            .map(|u| u.into())
            .collect()
    };

    let rounds: Vec<_> = room_state
        .get_rounds(&room_id)
        .await
        .into_iter()
        .map(Into::into)
        .collect();

    if let Some(current_round) = room_state.current_round.read().await.get(&room_id.clone())
    {
        let current_round: CurrentRoundDTO = current_round.clone().into();
        debug!(name = &current_round.name, "Sending current round...");
        socket
            .emit("current round", current_round)
            .unwrap_or_else(handlers::log_emit_err);
    } else {
        socket
            .emit("current round", ())
            .unwrap_or_else(handlers::log_emit_err);
    }

    debug!(count = rounds.len(), "Sending rounds...");
    handlers::emit_reply(&socket, ServerEvent::Rounds(&rounds));

    let votes: Vec<_> = room_state
        .get_votes(&room_id)
        .await
        .into_iter()
        .map(Into::into)
        .collect();
    debug!(count = votes.len(), "Sending votes...");
    handlers::emit_reply(&socket, ServerEvent::Votes(&votes));

    debug!(room_info = room_info.name, "Sending room info...");
    handlers::emit_reply(&socket, ServerEvent::Room(&room_info));

    debug!(count = users.len(), "Sending users...");
    handlers::emit_within(&socket, room_id, ServerEvent::Users(&users));

    Ok(())
}
