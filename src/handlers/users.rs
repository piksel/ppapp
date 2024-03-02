use socketioxide::extract::{SocketRef, State};
use crate::event::ServerEvent;
use crate::handlers;
use crate::state::{Session, Users};
use crate::state::user::UserDTO;

pub async fn handle_update_user(s: &SocketRef, name: String, email: String, users_state: State<Users>) {
    let user_id = &s.extensions.get::<Session>().unwrap().user_id.clone();
    let user = {
        let mut users = users_state.0.0.write().await;
        let user = users.get_mut(user_id).unwrap();
        user.name = name;
        user.avatar = handlers::hash_email(&email);
        user.email = email;
        user.clone()
    };
    let dto: UserDTO = user.into();
    handlers::emit_reply(&s, ServerEvent::User(&dto));
    let dto = {
        let mut dto = dto;
        dto.email = "".to_string();
        dto
    };
    for room in s.rooms().unwrap().iter() {
        handlers::emit_within(&s, room.clone(), ServerEvent::UserUpdated(&dto));
    }
}
