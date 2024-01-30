use ts_rs::TS;

#[derive(serde::Serialize, Clone, Debug, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub enum AckResult {
    OK,
    Error { error: String },
}

impl AckResult {
    pub(crate) fn error<T: Into<String>>(message: T) -> Self {
        AckResult::Error { error: message.into() }
    }
}