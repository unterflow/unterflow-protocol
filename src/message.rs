use serde_bytes::ByteBuf;
use std::collections::HashMap;

pub const NIL: &[u8] = &[0xc0];

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TopologyRequest {}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopologyResponse {
    topic_leaders: Vec<TopicLeader>,
    brokers: Vec<SocketAddress>,
}

impl TopologyResponse {
    pub fn topic_leaders(&self) -> &Vec<TopicLeader> {
        &self.topic_leaders
    }

    pub fn brokers(&self) -> &Vec<SocketAddress> {
        &self.brokers
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopicLeader {
    host: String,
    port: u16,
    topic_name: String,
    partition_id: u16,
}

impl TopicLeader {
    pub fn new(host: String, port: u16, topic_name: String, partition_id: u16) -> Self {
        TopicLeader {
            host,
            port,
            topic_name,
            partition_id: partition_id,
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct SocketAddress {
    host: String,
    port: u16,
}

impl SocketAddress {
    pub fn new(host: String, port: u16) -> Self {
        SocketAddress { host, port }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskSubscription {
    topic_name: String,
    partition_id: u32,
    subscriber_key: u64,
    task_type: String,
    lock_duration: u64,
    lock_owner: String,
    credits: u32,
}

impl TaskSubscription {
    pub fn for_topic(topic_name: String, partition_id: u32) -> Self {
        TaskSubscription {
            topic_name,
            partition_id,
            ..Default::default()
        }
    }

    pub fn new<S: Into<String>>(
        topic_name: S,
        partition_id: u32,
        task_type: S,
        lock_owner: S,
        lock_duration: u64,
        subscriber_key: u64,
        credits: u32,
    ) -> Self {
        TaskSubscription {
            topic_name: topic_name.into(),
            partition_id,
            subscriber_key,
            task_type: task_type.into(),
            lock_duration,
            lock_owner: lock_owner.into(),
            credits,
        }
    }

    pub fn subscriber_key(&self) -> u64 {
        self.subscriber_key
    }

    pub fn set_subscriber_key(&mut self, subscriber_key: u64) {
        self.subscriber_key = subscriber_key;
    }
}


#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopicSubscriber {
    start_position: u64,
    name: String,
    state: String,
    prefetch_capacity: u32,
    force_start: bool,
}

impl TopicSubscriber {
    pub fn new<S: Into<String>>(start_position: u64, name: S, state: S, prefetch_capacity: u32, force_start: bool) -> Self {
        TopicSubscriber {
            start_position,
            name: name.into(),
            state: state.into(),
            prefetch_capacity,
            force_start,
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct CloseSubscription {
    topic_name: String,
    partition_id: u32,
    subscriber_key: u64,
}

impl CloseSubscription {
    pub fn new(topic_name: String, partition_id: u32, subscriber_key: u64) -> Self {
        CloseSubscription {
            topic_name,
            partition_id,
            subscriber_key,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskHeaders {
    workflow_instance_key: i64,
    bpmn_process_id: String,
    workflow_definition_version: i64,
    workflow_key: i64,
    activity_id: String,
    activity_instance_key: i64,
}

impl Default for TaskHeaders {
    fn default() -> Self {
        TaskHeaders {
            workflow_instance_key: -1,
            bpmn_process_id: String::new(),
            workflow_definition_version: -1,
            workflow_key: -1,
            activity_id: String::new(),
            activity_instance_key: -1,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
    pub state: String,
    pub lock_time: i64,
    pub lock_owner: String,
    pub retries: i32,
    #[serde(rename = "type")]
    pub task_type: String,
    pub headers: TaskHeaders,
    // TODO(menski): this is probably not a string/string hash map
    pub custom_headers: HashMap<String, String>,
    pub payload: ByteBuf,
}

impl Default for TaskEvent {
    fn default() -> Self {
        TaskEvent {
            state: String::new(),
            lock_time: i64::min_value(),
            lock_owner: String::new(),
            retries: -1,
            task_type: String::new(),
            headers: TaskHeaders::default(),
            custom_headers: Default::default(),
            payload: NIL.to_vec().into(),
        }
    }
}

impl TaskEvent {
    pub fn new<S: Into<String>>(state: S, task_type: S, retries: i32) -> Self {
        TaskEvent {
            state: state.into(),
            task_type: task_type.into(),
            retries,
            ..Default::default()
        }
    }

    pub fn set_state<S: Into<String>>(&mut self, state: S) {
        self.state = state.into();
    }

    pub fn set_lock_owner<S: Into<String>>(&mut self, lock_owner: S) {
        self.lock_owner = lock_owner.into();
    }

    pub fn set_lock_time(&mut self, lock_time: i64) {
        self.lock_time = lock_time;
    }

    pub fn payload(&self) -> &ByteBuf {
        &self.payload
    }

    pub fn set_payload<B: Into<ByteBuf>>(&mut self, payload: B) {
        self.payload = payload.into();
    }

    pub fn add_custom_header<S: Into<String>>(&mut self, key: S, value: S) {
        self.custom_headers.insert(key.into(), value.into());
    }
}

pub const CREATE_DEPLOYMENT_STATE: &'static str = "CREATE_DEPLOYMENT";
pub const DEPLOYMENT_CREATED_STATE: &'static str = "DEPLOYMENT_CREATED";
pub const DEPLOYMENT_REJECTED_STATE: &'static str = "DEPLOYMENT_REJECTED";

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentEvent {
    pub state: String,
    pub deployed_workflows: Vec<DeployedWorkflow>,
    pub bpmn_xml: ByteBuf,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct DeployedWorkflow {
    pub bpmn_process_id: String,
    pub version: i32,
}


pub const CREATE_WORKFLOW_INSTANCE: &'static str = "CREATE_WORKFLOW_INSTANCE";
pub const WORKFLOW_INSTANCE_CREATED: &'static str = "WORKFLOW_INSTANCE_CREATED";
pub const WORKFLOW_INSTANCE_REJECTED: &'static str = "WORKFLOW_INSTANCE_REJECTED";
pub const START_EVENT_OCCURRED: &'static str = "START_EVENT_OCCURRED";
pub const END_EVENT_OCCURRED: &'static str = "END_EVENT_OCCURRED";
pub const SEQUENCE_FLOW_TAKEN: &'static str = "SEQUENCE_FLOW_TAKEN";
pub const ACTIVITY_READY: &'static str = "ACTIVITY_READY";
pub const ACTIVITY_ACTIVATED: &'static str = "ACTIVITY_ACTIVATED";
pub const ACTIVITY_COMPLETING: &'static str = "ACTIVITY_COMPLETING";
pub const ACTIVITY_COMPLETED: &'static str = "ACTIVITY_COMPLETED";
pub const ACTIVITY_TERMINATED: &'static str = "ACTIVITY_TERMINATED";
pub const WORKFLOW_INSTANCE_COMPLETED: &'static str = "WORKFLOW_INSTANCE_COMPLETED";
pub const CANCEL_WORKFLOW_INSTANCE: &'static str = "CANCEL_WORKFLOW_INSTANCE";
pub const WORKFLOW_INSTANCE_CANCELED: &'static str = "WORKFLOW_INSTANCE_CANCELED";
pub const CANCEL_WORKFLOW_INSTANCE_REJECTED: &'static str = "CANCEL_WORKFLOW_INSTANCE_REJECTED";
pub const UPDATE_PAYLOAD: &'static str = "UPDATE_PAYLOAD";
pub const PAYLOAD_UPDATED: &'static str = "PAYLOAD_UPDATED";
pub const UPDATE_PAYLOAD_REJECTED: &'static str = "UPDATE_PAYLOAD_REJECTED";

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct WorkInstanceEvent {
    pub state: String,
    pub bpmn_process_id: String,
    pub version: i32,
    pub workflow_key: i64,
    pub workflow_instance_key: i64,
    pub activity_id: String,
    pub payload: ByteBuf,
}
