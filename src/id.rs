use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use uuid::{Bytes, Uuid};
use anyhow::{self, Context, Result};

pub fn encode_id(uuid: &Uuid) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

pub fn decode_id<T: AsRef<[u8]>>(encoded: T) -> Result<Uuid> {
    let bytes = BASE64_URL_SAFE_NO_PAD.decode(encoded)?;
    // let bytes = Bytes::try_from(byte_vec).with_context(|| "Vec is the wrong size")?;
    Ok(Uuid::try_from(bytes)?)
}