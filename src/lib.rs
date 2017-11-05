extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

#[macro_use]
mod macros;
pub mod error;
pub mod frame;
pub mod msgpack;
pub mod sbe;

use error::Error;
use rmp_serde::{Deserializer, Serializer};

use serde::Deserialize;
use serde::Serialize;

pub fn topology_request(request_id: u64) -> Result<Vec<u8>, Error> {
    let request = msgpack::TopologyRequest::new();

    let mut buffer = Vec::new();
    request.serialize(&mut Serializer::new(&mut buffer))?;

    let buffer = sbe::encode_control_message(
        sbe::ANY_PARTITION,
        sbe::ControlMessageType::REQUEST_TOPOLOGY,
        &buffer,
    )?;
    frame::encode_request_response(0, 0, 0, request_id, &buffer)
}

pub fn topology_response(data: &[u8]) -> Result<msgpack::TopologyResponse, Error> {
    let frame = frame::decode_request_response(data)?;
    let data = sbe::decode_control_message(frame.message)?;
    let mut de = Deserializer::new(data);
    let topology = Deserialize::deserialize(&mut de)?;
    Ok(topology)
}
