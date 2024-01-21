use chrono::{DateTime, Utc};
use typescript_type_def::{TypeDef};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub from: Uuid,
    pub date: DateTime<Utc>,
}

#[derive(serde::Serialize, Clone, Debug, TypeDef)]
pub struct MessageDTO {
    pub content: String,
    pub from: String,
    pub date: String,
}

impl Message {
    pub fn into_dto(self) -> MessageDTO {
        MessageDTO {
            content: self.content,
            date: self.date.to_string(),
            from: self.from.as_simple().to_string()
        }
    }

    pub fn as_dto(&self) -> MessageDTO {
        MessageDTO {
            content: self.content.clone(),
            date: self.date.to_string(),
            from: self.from.as_simple().to_string()
        }
    }
}