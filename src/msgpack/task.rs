

use error::Error;

use msgpack::{NIL, deserialize};
use serde::Deserialize;

use serde_bytes::ByteBuf;
use std::collections::HashMap;

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
        deserialize(buffer)
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

enum_serialize!{
    TaskState => {
        TaskState::Create => "CREATE",
        TaskState::Created => "CREATED",

        TaskState::Lock => "LOCK",
        TaskState::Locked => "LOCKED",
        TaskState::LockRejected => "LOCK_REJECTED",

        TaskState::Complete => "COMPLETE",
        TaskState::Completed => "COMPLETED",
        TaskState::CompleteRejected => "COMPLETE_REJECTED",

        TaskState::ExpireLock => "EXPIRE_LOCK",
        TaskState::LockExpired => "LOCK_EXPIRED",
        TaskState::LockExpirationRejected => "LOCK_EXPIRATION_REJECTED",

        TaskState::Fail => "FAIL",
        TaskState::Failed => "FAILED",
        TaskState::FailRejected => "FAIL_REJECTED",

        TaskState::UpdateRetries => "UPDATE_RETRIES",
        TaskState::RetriesUpdated => "RETRIES_UPDATED",
        TaskState::UpdateRetriesRejected => "UPDATE_RETRIES_REJECTED",

        TaskState::Cancel => "CANCEL",
        TaskState::Canceled => "CANCELED",
        TaskState::CancelRejected => "CANCEL_REJECTED"
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