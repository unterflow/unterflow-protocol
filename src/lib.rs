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
use msgpack::{deserialize, serialize};
use serde::Serialize;

use std::collections::HashMap;

pub fn create_topic_request(request_id: u64, partition_id: u16, topic: &str, partitions: u32) -> Result<Vec<u8>, Error> {
    let topic_event = serialize(&msgpack::TopicEvent {
        state: msgpack::TopicState::Create,
        name: topic.into(),
        partitions,
    })?;

    let buffer = sbe::encode_execute_command_request(partition_id, sbe::EventType::TOPIC_EVENT, &topic_event)?;

    frame::encode_request_response(0, 0, 0, request_id, &buffer)
}

pub fn read_topic_response(data: &[u8]) -> Result<TopicEvent, Error> {
    let frame = frame::decode_request_response(data)?;
    let request_id = frame.request_response_header.request_id;

    let (metadata, event) = sbe::decode_execute_command_response(frame.message)?;
    let event = deserialize(event)?;
    Ok(TopicEvent {
        request_id,
        metadata,
        event,
    })
}

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
    deserialize(data)
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

    pub fn retries(&mut self, retries: i32) -> &mut Self {
        self.task_event.retries = retries;
        self
    }

    pub fn custom_headers(&mut self, custom_headers: HashMap<String, String>) -> &mut Self {
        self.task_event.custom_headers = custom_headers;
        self
    }

    pub fn custom_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.task_event.custom_headers.insert(
            key.into(),
            value.into(),
        );
        self
    }

    pub fn payload<S: Serialize>(&mut self, payload: S) -> Result<&mut Self, Error> {
        self.task_event.payload = serialize(&payload)?.into();
        Ok(self)
    }

    pub fn build(&self) -> Result<Vec<u8>, Error> {
        let buffer = serialize(&self.task_event)?;

        let buffer = sbe::encode_execute_command_request(self.partition_id, sbe::EventType::TASK_EVENT, &buffer)?;

        frame::encode_request_response(0, 0, 0, self.request_id, &buffer)
    }
}

#[derive(Debug, PartialEq)]
pub struct EventMetadata {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
}

#[derive(Debug, PartialEq)]
pub struct TaskEvent {
    pub metadata: EventMetadata,
    pub event: msgpack::TaskEvent,
}

#[derive(Debug, PartialEq)]
pub struct TopicEvent {
    pub request_id: u64,
    pub metadata: EventMetadata,
    pub event: msgpack::TopicEvent,
}

pub fn create_task_response(data: &[u8]) -> Result<TaskEvent, Error> {
    let frame = frame::decode_request_response(data)?;
    let (metadata, event) = sbe::decode_execute_command_response(frame.message)?;
    let event = deserialize(event)?;
    Ok(TaskEvent { metadata, event })
}
