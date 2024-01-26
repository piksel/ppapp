use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use typescript_type_def::TypeDef;
use uuid::Uuid;

fn encode_id(uuid: &Uuid) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
pub struct UserDTO {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            email: value.email,
        }
    }
}

impl User {
    pub(crate) fn new(name: String) -> User {
        Self {
            user_id: encode_id(&Uuid::new_v4()),
            name,
            email: "".to_string(),
        }
    }
}

// impl From<&Uuid> for User {
//     fn from(value: &Uuid) -> Self {
//         Self {
//             user_id: value.clone(),
//             name: format!("Rocket Grunt {}", BASE64_URL_SAFE_NO_PAD.encode(value.as_bytes())),
//             email: "".to_string(),
//         }
//     }
// }