extern crate unterflow_protocol;

use unterflow_protocol::*;
use unterflow_protocol::msgpack::*;

macro_rules! dump_vec {
        ($v:ident, $file:expr) => (
            let $v = include_bytes!(concat!("dumps/", $file)).to_vec();
        )
    }

#[test]
fn test_topology_request() {
    dump_vec!(expected, "topology-request.bin");
    let buffer = topology_request(256).unwrap();

    assert_eq!(expected, buffer);
}

#[test]
fn test_topology_response() {
    dump_vec!(response, "topology-response.bin");

    let expected = TopologyResponse {
        topic_leaders: vec![
            TopicLeader {
                topic_name: "internal-system".to_string(),
                partition_id: 0,
                host: "0.0.0.0".to_string(),
                port: 51_015,
            },
        ],
        brokers: vec![
            Broker {
                host: "0.0.0.0".to_string(),
                port: 51_015,
            },
        ],
    };

    let topology = topology_response(&response).unwrap();
    assert_eq!(expected, topology);
}
