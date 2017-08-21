extern crate unterflow_protocol;

use std::env;
use std::net::TcpStream;
use unterflow_protocol::{RequestResponseMessage, TransportMessage};
use unterflow_protocol::io::{FromBytes, FromData, ToBytes, ToData};
use unterflow_protocol::message::{CREATE_STATE, TaskEvent};
use unterflow_protocol::sbe::{EventType, ExecuteCommandRequest};

fn main() {
    let broker_address = env::args().nth(1).unwrap_or_else(
        || "localhost:51015".to_string(),
    );

    let mut stream = TcpStream::connect(&broker_address).expect(&format!("Failed to connect to broker {}", broker_address));
    println!("Connected to broker {}", broker_address);

    let event = TaskEvent {
        state: CREATE_STATE.into(),
        lock_owner: "foo".into(),
        retries: 3,
        ..Default::default()
    };
    let command = event.to_data().expect("Failed to convert event");
    let message = ExecuteCommandRequest {
        topic_name: "default_topic".into(),
        partition_id: 0,
        position: 0,
        key: 0,
        event_type: EventType::TaskEvent,
        command,
    };

    let request = TransportMessage::request(1, message);
    request.to_bytes(&mut stream).expect(
        "Failed to send request",
    );

    let response = TransportMessage::from_bytes(&mut stream).expect("Failed to read response");

    if let TransportMessage::RequestResponse(response) = response {
        if let RequestResponseMessage::ExecuteCommandResponse(ref message) = *response.message() {
            let task = TaskEvent::from_data(message).expect("Failed to read task event");
            println!("{:#?}", task);
        } else {
            panic!("Unexpected response message {:?}", response.message());
        }
    } else {
        panic!("Unexpected response {:?}", response);
    }
}
