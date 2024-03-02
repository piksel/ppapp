use ts_rs::TS;
use crate::state::game::Game;

#[derive(Clone, Debug)]
pub struct Room {
    pub room_id: String,
    pub name: String,
    pub game: Game,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct RoomDTO {
    #[serde(rename = "roomID")]
    pub room_id: String,
    pub name: String,
    pub game: Game,
}

impl From<Room> for RoomDTO {
    fn from(value: Room) -> Self {
        Self {
            room_id: value.room_id, //.as_simple().to_string(),
            name: value.name,
            game: value.game,
        }
    }
}
