#![allow(non_snake_case)]

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TopologyRequest {}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct TopologyResponse {
    topicLeaders: Vec<TopicLeader>,
    brokers: Vec<SocketAddress>,
}

impl TopologyResponse {
    pub fn topic_leaders(&self) -> &Vec<TopicLeader> {
        &self.topicLeaders
    }

    pub fn brokers(&self) -> &Vec<SocketAddress> {
        &self.brokers
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct TopicLeader {
    host: String,
    port: u16,
    topicName: String,
    partitionId: u16,
}

impl TopicLeader {
    pub fn new(host: String, port: u16, topic_name: String, partition_id: u16) -> Self {
        TopicLeader {
            host,
            port,
            topicName: topic_name,
            partitionId: partition_id,
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
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
pub struct TaskSubscription {
    topicName: String,
    partitionId: u32,
    subscriberKey: u64,
    taskType: String,
    lockDuration: u64,
    lockOwner: String,
    credits: u32,
}

impl TaskSubscription {
    pub fn for_topic(topicName: String, partitionId: u32) -> Self {
        TaskSubscription {
            topicName,
            partitionId,
            ..Default::default()
        }
    }

    pub fn new(topicName: String,
               partitionId: u32,
               taskType: String,
               lockOwner: String,
               lockDuration: u64,
               subscriberKey: u64,
               credits: u32)
               -> Self {
        TaskSubscription {
            topicName,
            partitionId,
            subscriberKey,
            taskType,
            lockDuration,
            lockOwner,
            credits,
        }
    }
}


#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct TopicSubscriber {
    startPosition: u64,
    name: String,
    prefetchCapacity: u32,
    forceStart: bool,
}

impl TopicSubscriber {
    pub fn new(startPosition: u64, name: String, prefetchCapacity: u32, forceStart: bool) -> Self {
        TopicSubscriber {
            startPosition,
            name,
            prefetchCapacity,
            forceStart,
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct CloseSubscription {
    topicName: String,
    partitionId: u32,
    subscriberKey: u64,
}

impl CloseSubscription {
    pub fn new(topicName: String, partitionId: u32, subscriberKey: u64) -> Self {
        CloseSubscription {
            topicName,
            partitionId,
            subscriberKey,
        }
    }
}
