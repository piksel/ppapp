use socketioxide::extract::{SocketRef, State};
use crate::event::ServerEvent;
use crate::handlers;
use crate::handlers::EventResult;
use crate::state::{RoomState, Session, Vote};
use crate::state::round::CurrentRoundDTO;
use crate::state::vote::VoteDTO;

pub async fn handle_end_vote(socket: &SocketRef, room: String, room_state: State<RoomState>) -> EventResult {
    {
        let votes_state = room_state.votes.read().await;
        let Some(votes) = votes_state.get(&room) else {
            return Err("no votes for round".into());
        };
        let members: Vec<_> = room_state
            .members
            .read()
            .await
            .get(&room)
            .unwrap()
            .iter()
            .cloned()
            .collect();
        if members.iter().any(|m| votes.get(m).is_none()) {
            return Err("not every member has voted".into());
        }
    }

    let current_round: CurrentRoundDTO = {
        let mut current_round_state = room_state.current_round.write().await;
        let Some(current_round) = current_round_state.get_mut(&room) else {
            return Err("no current round".into());
        };
        current_round.flipped = true;
        current_round.clone().into()
    };

    handlers::emit_within(&socket, room, ServerEvent::CurrentRound(&current_round));
    Ok(())
}

pub async fn handle_vote(s: &SocketRef, room: String, score: String,
                         room_state: State<RoomState>) -> EventResult {
    let user_id = s.extensions.get::<Session>().unwrap().user_id.clone();

    let score = match score.parse() {
        Ok(score) => score,
        Err(error) => {
            return Err(format!("{error:?}"));
        }
    };

    let vote = Vote {
        user_id: user_id.clone(),
        score,
    };
    {
        room_state
            .votes
            .write()
            .await
            .entry(room.clone())
            .or_default()
            .insert(user_id.clone(), vote.clone());
    }
    let dto: VoteDTO = vote.into();
    handlers::emit_reply(&s, ServerEvent::Vote(&dto));

    let votes: Vec<VoteDTO> = room_state
        .votes
        .read()
        .await
        .get(&room)
        .unwrap()
        .values()
        .cloned()
        .map(Into::into)
        .collect();
    handlers::emit_within(&s, room, ServerEvent::Votes(&votes));

    Ok(())
}
