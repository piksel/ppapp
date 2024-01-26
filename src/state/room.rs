use typescript_type_def::TypeDef;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Room {
    pub room_id: String,
    pub name: String,
}

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
pub struct RoomDTO {
    #[serde(rename = "roomID")]
    pub room_id: String,
    pub name: String,
}

impl From<Room> for RoomDTO {
    fn from(value: Room) -> Self {
        Self {
            room_id: value.room_id, //.as_simple().to_string(),
            name: value.name
        }
    }
}