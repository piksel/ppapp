use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use uuid::Uuid;
use anyhow::{self, Result};

pub fn encode_id(uuid: &Uuid) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

pub fn decode_id<T: AsRef<[u8]>>(encoded: T) -> Result<Uuid> {
    let bytes = BASE64_URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(Uuid::try_from(bytes)?)
}