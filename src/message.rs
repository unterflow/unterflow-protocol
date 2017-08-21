use serde_bytes::ByteBuf;
use std::collections::HashMap;

pub const NIL: &[u8] = &[0xc0];

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TopologyRequest {}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopologyResponse {
    pub topic_leaders: Vec<TopicLeader>,
    pub brokers: Vec<SocketAddress>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopicLeader {
    pub host: String,
    pub port: u16,
    pub topic_name: String,
    pub partition_id: u16,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct SocketAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskSubscription {
    pub topic_name: String,
    pub partition_id: u32,
    pub subscriber_key: u64,
    pub task_type: String,
    pub lock_duration: u64,
    pub lock_owner: String,
    pub credits: u32,
}

impl TaskSubscription {
    pub fn for_topic(topic_name: String, partition_id: u32) -> Self {
        TaskSubscription {
            topic_name,
            partition_id,
            ..Default::default()
        }
    }
}

pub const SUBSCRIBE_STATE: &'static str = "SUBSCRIBE";
pub const SUBSCRIBED_STATE: &'static str = "SUBSCRIBED";

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TopicSubscriber {
    pub start_position: u64,
    pub name: String,
    pub state: String,
    pub prefetch_capacity: u32,
    pub force_start: bool,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct CloseSubscription {
    pub topic_name: String,
    pub partition_id: u32,
    pub subscriber_key: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct TaskHeaders {
    pub workflow_instance_key: i64,
    pub bpmn_process_id: String,
    pub workflow_definition_version: i64,
    pub workflow_key: i64,
    pub activity_id: String,
    pub activity_instance_key: i64,
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

pub const CREATE_STATE: &'static str = "CREATE";
pub const CREATED_STATE: &'static str = "CREATED";
pub const LOCK_STATE: &'static str = "LOCK";
pub const LOCKED_STATE: &'static str = "LOCKED";
pub const LOCK_REJECTED_STATE: &'static str = "LOCK_REJECTED";
pub const COMPLETE_STATE: &'static str = "COMPLETE";
pub const COMPLETED_STATE: &'static str = "COMPLETED";
pub const COMPLETE_REJECTED_STATE: &'static str = "COMPLETE_REJECTED";
pub const EXPIRE_LOCK_STATE: &'static str = "EXPIRE_LOCK";
pub const LOCK_EXPIRED_STATE: &'static str = "LOCK_EXPIRED";
pub const LOCK_EXPIRATION_REJECTED_STATE: &'static str = "LOCK_EXPIRATION_REJECTED";
pub const FAIL_STATE: &'static str = "FAIL";
pub const FAILED_STATE: &'static str = "FAILED";
pub const FAIL_REJECTED_STATE: &'static str = "FAIL_REJECTED";
pub const UPDATE_RETRIES_STATE: &'static str = "UPDATE_RETRIES";
pub const RETRIES_UPDATED_STATE: &'static str = "RETRIES_UPDATED";
pub const UPDATE_RETRIES_REJECTED_STATE: &'static str = "UPDATE_RETRIES_REJECTED";
pub const CANCEL_STATE: &'static str = "CANCEL";
pub const CANCELED_STATE: &'static str = "CANCELED";
pub const CANCEL_REJECTED_STATE: &'static str = "CANCEL_REJECTED";

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

pub const CREATE_WORKFLOW_INSTANCE_STATE: &'static str = "CREATE_WORKFLOW_INSTANCE";
pub const WORKFLOW_INSTANCE_CREATED_STATE: &'static str = "WORKFLOW_INSTANCE_CREATED";
pub const WORKFLOW_INSTANCE_REJECTED_STATE: &'static str = "WORKFLOW_INSTANCE_REJECTED";
pub const START_EVENT_OCCURRED_STATE: &'static str = "START_EVENT_OCCURRED";
pub const END_EVENT_OCCURRED_STATE: &'static str = "END_EVENT_OCCURRED";
pub const SEQUENCE_FLOW_TAKEN_STATE: &'static str = "SEQUENCE_FLOW_TAKEN";
pub const ACTIVITY_READY_STATE: &'static str = "ACTIVITY_READY";
pub const ACTIVITY_ACTIVATED_STATE: &'static str = "ACTIVITY_ACTIVATED";
pub const ACTIVITY_COMPLETING_STATE: &'static str = "ACTIVITY_COMPLETING";
pub const ACTIVITY_COMPLETED_STATE: &'static str = "ACTIVITY_COMPLETED";
pub const ACTIVITY_TERMINATED_STATE: &'static str = "ACTIVITY_TERMINATED";
pub const WORKFLOW_INSTANCE_COMPLETED_STATE: &'static str = "WORKFLOW_INSTANCE_COMPLETED";
pub const CANCEL_WORKFLOW_INSTANCE_STATE: &'static str = "CANCEL_WORKFLOW_INSTANCE";
pub const WORKFLOW_INSTANCE_CANCELED_STATE: &'static str = "WORKFLOW_INSTANCE_CANCELED";
pub const CANCEL_WORKFLOW_INSTANCE_REJECTED_STATE: &'static str = "CANCEL_WORKFLOW_INSTANCE_REJECTED";
pub const UPDATE_PAYLOAD_STATE: &'static str = "UPDATE_PAYLOAD";
pub const PAYLOAD_UPDATED_STATE: &'static str = "PAYLOAD_UPDATED";
pub const UPDATE_PAYLOAD_REJECTED_STATE: &'static str = "UPDATE_PAYLOAD_REJECTED";


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
