use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use typescript_type_def::{TypeDef};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub from: String,
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
            from: BASE64_URL_SAFE_NO_PAD.encode(self.from.as_bytes()),
        }
    }

    pub fn as_dto(&self) -> MessageDTO {
        MessageDTO {
            content: self.content.clone(),
            date: self.date.to_string(),
            from: BASE64_URL_SAFE_NO_PAD.encode(self.from.as_bytes()),
        }
    }
}