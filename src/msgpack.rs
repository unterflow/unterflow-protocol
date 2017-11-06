
use error::Error;
use rmp_serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};

use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::fmt;

pub const EMPTY_MAP: u8 = 0x80;
pub const EMPTY_ARRAY: u8 = 0x90;
pub const NIL: u8 = 0xc0;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyResponse {
    pub topic_leaders: Vec<TopicLeader>,
    pub brokers: Vec<Broker>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicLeader {
    pub host: String,
    pub port: u32,
    pub topic_name: String,
    pub partition_id: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Broker {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
    pub state: TaskState,
    pub lock_time: i64,
    pub retries: i32,
    #[serde(rename = "type")]
    pub task_type: String,
    pub headers: TaskHeaders,
    pub custom_headers: HashMap<String, String>,
    pub payload: ByteBuf,
}

impl Default for TaskEvent {
    fn default() -> Self {
        TaskEvent {
            state: TaskState::default(),
            lock_time: ::std::i64::MIN,
            retries: 3,
            task_type: "".to_string(),
            headers: TaskHeaders::default(),
            custom_headers: HashMap::new(),
            payload: vec![NIL].into(),
        }
    }
}

impl TaskEvent {
    pub fn payload_as<'de, D: Deserialize<'de>>(&self) -> Result<D, Error> {
        let buffer: &[u8] = self.payload.as_ref();
        let mut de = rmp_serde::Deserializer::new(buffer);
        let payload = Deserialize::deserialize(&mut de)?;
        Ok(payload)
    }
}

#[derive(Debug, PartialEq)]
pub enum TaskState {
    Create,
    Created,

    Lock,
    Locked,
    LockRejected,

    Complete,
    Completed,
    CompleteRejected,

    ExpireLock,
    LockExpired,
    LockExpirationRejected,

    Fail,
    Failed,
    FailRejected,

    UpdateRetries,
    RetriesUpdated,
    UpdateRetriesRejected,

    Cancel,
    Canceled,
    CancelRejected,
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState::Create
    }
}

impl Serialize for TaskState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match *self {
            TaskState::Create => "CREATE",
            _ => unimplemented!(),
        };

        serializer.serialize_str(value)
    }
}

impl<'d> Deserialize<'d> for TaskState {
    fn deserialize<D>(deserializer: D) -> Result<TaskState, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_str(TaskStateVisitor)
    }
}

struct TaskStateVisitor;

impl<'d> Visitor<'d> for TaskStateVisitor {
    type Value = TaskState;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string represtentation of task state")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "CREATED" => Ok(TaskState::Created),
            _ => Err(E::custom(format!("Unsupported task state: {}", value))),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskHeaders {
    workflow_instance_key: i64,
    bpmn_process_id: String,
    workflow_definition_version: i32,
    workflow_key: i64,
    activity_id: String,
    activity_instance_key: i64,
}

impl Default for TaskHeaders {
    fn default() -> Self {
        TaskHeaders {
            workflow_instance_key: -1,
            bpmn_process_id: "".to_string(),
            workflow_definition_version: -1,
            workflow_key: -1,
            activity_id: "".to_string(),
            activity_instance_key: -1,
        }
    }
}
