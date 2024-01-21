use typescript_type_def::TypeDef;
use uuid::Uuid;

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
pub struct UserDTO {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id.as_simple().to_string(),
            username: value.username
        }
    }
}

impl User {
    pub(crate) fn new(username: String) -> User {
        Self {
            user_id: Uuid::new_v4(),
            username,
        }
    }
}

impl From<&Uuid> for User {
    fn from(value: &Uuid) -> Self {
        Self {
            user_id: value.clone(),
            username: format!("anon-{}", value.as_simple())
        }
    }
}