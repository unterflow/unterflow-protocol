extern crate unterflow_protocol;

use std::env;
use std::net::TcpStream;
use unterflow_protocol::{RequestResponseMessage, TransportMessage};
use unterflow_protocol::io::{FromBytes, FromData, ToBytes};
use unterflow_protocol::message::{TopologyRequest, TopologyResponse};
use unterflow_protocol::sbe::ControlMessageType;

fn main() {
    let broker_address = env::args().nth(1).unwrap_or_else(
        || "localhost:51015".to_string(),
    );

    let mut stream = TcpStream::connect(&broker_address).expect(&format!("Failed to connect to broker {}", broker_address));
    println!("Connected to broker {}", broker_address);

    let message = ControlMessageType::RequestTopology
        .with(&TopologyRequest {})
        .expect("Failed to create message");

    let request = TransportMessage::request(1, message);
    request.to_bytes(&mut stream).expect(
        "Failed to send request",
    );

    let response = TransportMessage::from_bytes(&mut stream).expect("Failed to read response");

    if let TransportMessage::RequestResponse(response) = response {
        if let RequestResponseMessage::ControlMessageResponse(ref message) = *response.message() {
            let topology = TopologyResponse::from_data(message).expect("Failed to read topology");
            println!("{:#?}", topology);
        } else {
            panic!("Unexpected response message {:?}", response.message());
        }
    } else {
        panic!("Unexpected response {:?}", response);
    }
}
