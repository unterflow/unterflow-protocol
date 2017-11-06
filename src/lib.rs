extern crate serde;
extern crate serde_bytes;
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
use rmp_serde::encode::StructMapWriter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn topology_request(request_id: u64) -> Result<Vec<u8>, Error> {
    let buffer = [msgpack::EMPTY_MAP];

    let buffer = sbe::encode_control_message_request(
        sbe::ANY_PARTITION,
        sbe::ControlMessageType::REQUEST_TOPOLOGY,
        &buffer,
    )?;

    frame::encode_request_response(0, 0, 0, request_id, &buffer)
}

pub fn topology_response(data: &[u8]) -> Result<msgpack::TopologyResponse, Error> {
    let frame = frame::decode_request_response(data)?;
    let data = sbe::decode_control_message_response(frame.message)?;
    let mut de = Deserializer::new(data);
    let topology = Deserialize::deserialize(&mut de)?;
    Ok(topology)
}

pub struct CreateTaskBuilder {
    request_id: u64,
    partition_id: u16,
    task_event: msgpack::TaskEvent,
}

impl CreateTaskBuilder {
    pub fn new(request_id: u64, partition_id: u16, task_type: &str) -> Self {
        CreateTaskBuilder {
            request_id,
            partition_id,
            task_event: msgpack::TaskEvent {
                task_type: task_type.into(),
                ..Default::default()
            },
        }
    }

    pub fn retries<'a>(&'a mut self, retries: i32) -> &'a mut Self {
        self.task_event.retries = retries;
        self
    }

    pub fn custom_headers<'a>(&'a mut self, custom_headers: HashMap<String, String>) -> &'a mut Self {
        self.task_event.custom_headers = custom_headers;
        self
    }

    pub fn custom_header<'a>(&'a mut self, key: &str, value: &str) -> &'a mut Self {
        self.task_event.custom_headers.insert(
            key.into(),
            value.into(),
        );
        self
    }

    pub fn payload<'a, S: Serialize>(&'a mut self, payload: S) -> Result<&'a mut Self, Error> {
        self.task_event.payload = serialize(&payload)?.into();
        Ok(self)
    }

    pub fn build(&self) -> Result<Vec<u8>, Error> {
        let buffer = serialize(&self.task_event)?;

        let buffer = sbe::encode_execute_command_request(self.partition_id, sbe::EventType::TASK_EVENT, &buffer)?;

        frame::encode_request_response(0, 0, 0, self.request_id, &buffer)
    }
}

pub fn create_task_response(data: &[u8]) -> Result<msgpack::TaskEvent, Error> {
    let frame = frame::decode_request_response(data)?;
    let event = sbe::decode_execute_command_response(frame.message)?;
    let mut de = Deserializer::new(event.data);
    let event = Deserialize::deserialize(&mut de)?;
    Ok(event)
}

fn serialize<S: Serialize>(data: &S) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::new();
    data.serialize(
        &mut Serializer::with(&mut buffer, StructMapWriter),
    )?;
    Ok(buffer)
}
