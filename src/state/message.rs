use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Utc};
use ts_rs::TS;

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub from: String,
    pub date: DateTime<Utc>,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "client/src/types/ppapi/")]
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
