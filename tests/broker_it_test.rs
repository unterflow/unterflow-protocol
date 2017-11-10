#[macro_use]
extern crate maplit;
extern crate unterflow_protocol;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use unterflow_protocol::*;

const BROKER_ADDRESS: &str = "127.0.0.1:51015";

fn send_request(request: &[u8]) -> Vec<u8> {
    let mut stream = TcpStream::connect(BROKER_ADDRESS).unwrap();
    stream.write_all(request).unwrap();

    let mut buffer = vec![0; 4096];
    stream.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
    let bytes = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes);

    buffer
}

fn try_create_topic(topic: &str, partitions: u32) {
    let request = create_topic_request(0, 0, topic, partitions).unwrap();
    let _ = send_request(&request);
}

// assumes there is only one partition for the topic
fn get_partition(topic: &str) -> Option<u16> {
    let request = topology_request(1).unwrap();

    let buffer = send_request(&request);

    let topology = topology_response(&buffer).unwrap();

    for topic_leader in topology.topic_leaders {
        if topic_leader.topic_name == topic {
            return Some(topic_leader.partition_id);
        }
    }

    None
}

#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn should_create_topic() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now = now.as_secs() * 1000 + u64::from(now.subsec_nanos()) / 1_000_000;
    let topic = format!("test-{}", now);
    let request = create_topic_request(123, 0, &topic, 2).unwrap();
    let buffer = send_request(&request);
    let TopicEvent {
        request_id,
        metadata,
        event,
    } = read_topic_response(&buffer).unwrap();
    assert_eq!(123, request_id);

    assert_eq!(0, metadata.partition_id);

    assert_eq!(msgpack::TopicState::Created, event.state);
    assert_eq!(topic, event.name);
    assert_eq!(2, event.partitions);
}


#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn test_topology_request() {
    let request = topology_request(1).unwrap();

    let buffer = send_request(&request);

    let response = topology_response(&buffer).unwrap();
    assert!(response.brokers.len() >= 1);
    assert!(response.topic_leaders.len() >= 1);

    let topic_leader = &response.topic_leaders[0];
    assert_eq!("internal-system", topic_leader.topic_name);
    assert_eq!(0, topic_leader.partition_id);
}

#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn test_create_task() {
    try_create_topic("default-topic", 1);
    let partition = get_partition("default-topic").unwrap();

    let payload =
        hashmap!{
        "foo" => "bar"
    };

    let request = CreateTaskBuilder::new(0, partition, "foo")
        .retries(4)
        .custom_header("a", "1")
        .custom_header("b", "2")
        .payload(payload)
        .unwrap()
        .build()
        .unwrap();

    let buffer = send_request(&request);

    let TaskEvent { metadata, event } = create_task_response(&buffer).unwrap();
    assert_eq!(partition, metadata.partition_id);

    assert_eq!(msgpack::TaskState::Created, event.state);
    assert_eq!("foo", event.task_type);
    assert_eq!(4, event.retries);
    assert_eq!(Some(&"1".to_string()), event.custom_headers.get("a"));
    assert_eq!(Some(&"2".to_string()), event.custom_headers.get("b"));

    let payload = event.payload_as::<HashMap<String, String>>().unwrap();
    assert_eq!(Some(&"bar".to_string()), payload.get("foo"));

}
