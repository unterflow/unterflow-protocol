#[macro_use]
extern crate maplit;
extern crate unterflow_protocol;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

use unterflow_protocol::*;


const BROKER_ADDRESS: &str = "127.0.0.1:51015";

#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn test_topology_request() {
    let mut stream = TcpStream::connect(BROKER_ADDRESS).unwrap();

    let request = topology_request(1).unwrap();
    stream.write_all(&request).unwrap();

    let mut buffer = vec![0; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes);

    let response = topology_response(&buffer).unwrap();

    assert!(1 <= response.brokers.len());
    assert!(1 <= response.topic_leaders.len());

    let topic_leader = &response.topic_leaders[0];
    assert_eq!("internal-system", topic_leader.topic_name);
    assert_eq!(0, topic_leader.partition_id);
}

#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn test_create_task() {
    let mut stream = TcpStream::connect(BROKER_ADDRESS).unwrap();

    let payload =
        hashmap!{
        "foo" => "bar"
    };

    let request = CreateTaskBuilder::new(0, 1, "foo")
        .retries(4)
        .custom_header("a", "1")
        .custom_header("b", "2")
        .payload(payload)
        .unwrap()
        .build()
        .unwrap();

    stream.write_all(&request).unwrap();

    let mut buffer = vec![0; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes);

    let TaskEvent { metadata, event } = create_task_response(&buffer).unwrap();
    assert_eq!(1, metadata.partition_id);
    assert!(metadata.position > 0);
    assert!(metadata.key > 0);

    assert_eq!(msgpack::TaskState::Created, event.state);
    assert_eq!("foo", event.task_type);
    assert_eq!(4, event.retries);
    assert_eq!(Some(&"1".to_string()), event.custom_headers.get("a"));
    assert_eq!(Some(&"2".to_string()), event.custom_headers.get("b"));

    let payload = event.payload_as::<HashMap<String, String>>().unwrap();
    assert_eq!(Some(&"bar".to_string()), payload.get("foo"));

}
