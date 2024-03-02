use ts_rs::TS;

#[derive(serde::Serialize, Clone, Debug, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "client/src/types/ppapi/")]
pub enum AckResult<T> {
    OK {
        #[serde()]
        content: T
    },
    Error { error: String },
}