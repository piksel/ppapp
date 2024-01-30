use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use ts_rs::TS;
use uuid::Uuid;

fn encode_id(uuid: &Uuid) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub struct UserDTO {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub avatar: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub avatar: String,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            email: value.email,
            avatar: value.avatar,
        }
    }
}

impl User {
    pub(crate) fn new(name: String) -> User {
        Self {
            user_id: encode_id(&Uuid::new_v4()),
            name,
            email: "".to_string(),
            avatar: "".to_string(),
        }
    }
}