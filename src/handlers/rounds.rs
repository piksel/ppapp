use socketioxide::extract::{SocketRef, State};
use crate::event::ServerEvent;
use crate::handlers;
use crate::state::{RoomState, Round, Vote};
use crate::state::round::{CurrentRound, CurrentRoundDTO, RoundDTO, RoundOpts};

pub async fn handle_new(s: &SocketRef, room: String, round_opts: RoundOpts, room_state: State<RoomState>) -> Result<(), String> {
    if !room_state
        .current_round
        .read()
        .await
        .get(&room)
        .map_or(true, |r| r.flipped)
    {
        return Err("the current round is not done".into());
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
        let round_count = rounds.len()
            + match current_rounds.get(&room) {
            Some(_) => 1,
            None => 0,
        };
        let current_round = CurrentRound::new(round_count, round_opts);
        if let Some(prev_round) = current_rounds.insert(room.clone(), current_round.clone())
        {
            rounds.push(Round {
                votes,
                name: prev_round.name,
            });
        }

        (
            rounds.iter().cloned().map(Into::into).collect(),
            current_round.clone().into(),
        )
    };

    handlers::emit_within(&s, room.clone(), ServerEvent::Rounds(&rounds));
    handlers::emit_within(&s, room.clone(), ServerEvent::Votes(&vec![]));
    handlers::emit_within(&s, room.clone(), ServerEvent::CurrentRound(&current_round));
    Ok(())
}
