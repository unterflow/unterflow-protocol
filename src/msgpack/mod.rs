pub mod task;
pub mod topic;
pub mod topology;


use error::Error;
use rmp_serde::{Deserializer, Serializer};
use rmp_serde::encode::StructMapWriter;
use serde::{Deserialize, Serialize};

pub const EMPTY_MAP: u8 = 0x80;
pub const EMPTY_ARRAY: u8 = 0x90;
pub const NIL: u8 = 0xc0;

pub fn serialize<S: Serialize>(data: &S) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::new();
    data.serialize(
        &mut Serializer::with(&mut buffer, StructMapWriter),
    )?;
    Ok(buffer)
}

pub fn deserialize<'d, D: Deserialize<'d>>(data: &[u8]) -> Result<D, Error> {
    let mut de = Deserializer::new(data);
    let value = Deserialize::deserialize(&mut de)?;
    Ok(value)
}
