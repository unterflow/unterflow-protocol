
extern crate unterflow_protocol;

use std::io::prelude::*;
use std::net::TcpStream;

use unterflow_protocol::*;

const BROKER_ADDERSS: &'static str = "127.0.0.1:51015";

#[test]
#[cfg_attr(not(feature = "broker-it"), ignore)]
fn test_topology_request() {
    let mut stream = TcpStream::connect(BROKER_ADDERSS).unwrap();

    let request = topology_request(1).unwrap();
    stream.write(&request).unwrap();

    let mut buffer = vec![0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = topology_response(&buffer).unwrap();

    assert_eq!(1, response.brokers.len());
    assert_eq!(1, response.topic_leaders.len());

    let topic_leader = &response.topic_leaders[0];
    assert_eq!("internal-system", topic_leader.topic_name);
    assert_eq!(0, topic_leader.partition_id);
}
