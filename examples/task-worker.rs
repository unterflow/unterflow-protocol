extern crate unterflow_protocol;

use std::env;
use std::net::TcpStream;
use unterflow_protocol::{RequestResponseMessage, SingleRequestMessage, TransportMessage};
use unterflow_protocol::io::{FromBytes, FromData, ToBytes};
use unterflow_protocol::message::{TaskEvent, TaskSubscription};
use unterflow_protocol::sbe::{ControlMessageType, ExecuteCommandRequest};

fn main() {
    let task_type = env::args().nth(1).unwrap_or_else(|| "foo".to_string());
    let lock_owner = env::args().nth(2).unwrap_or_else(
        || "unterflow".to_string(),
    );
    let broker_address = env::args().nth(3).unwrap_or_else(
        || "localhost:51015".to_string(),
    );

    let mut request_id = 1;

    let mut stream = TcpStream::connect(&broker_address).expect(&format!("Failed to connect to broker {}", broker_address));
    println!("Connected to broker {}", broker_address);

    let mut credits = 32;

    let mut subscription = TaskSubscription::new(
        "default-topic",
        0,
        &task_type,
        &lock_owner,
        1_000,
        0,
        credits,
    );

    let message = ControlMessageType::AddTaskSubscription
        .with(&subscription)
        .expect("Failed to create message");

    let request = TransportMessage::request(request_id, message);
    request.to_bytes(&mut stream).expect(
        "Failed to send request",
    );


    loop {
        let request = TransportMessage::from_bytes(&mut stream).expect("Failed to read response");
        match request {
            TransportMessage::SingleRequest(request) => {
                let message = request.message();
                if let SingleRequestMessage::SubscribedEvent(ref message) = *message {
                    credits -= 1;
                    let event = TaskEvent::from_data(message).expect("Failed to read task event");
                    println!("Event {:?}", event);

                    let message = ExecuteCommandRequest::complete_task(message, event).expect("Failed to create complete task message");
                    request_id += 1;
                    let request = TransportMessage::request(request_id, message);

                    request.to_bytes(&mut stream).expect(
                        "Failed to send request",
                    );

                    if credits == 0 {
                        // increase credits again
                        credits = 32;

                        let message = ControlMessageType::IncreaseTaskSubscriptionCredits
                            .with(&subscription)
                            .expect("Failed to create message");

                        let request = TransportMessage::request(request_id, message);
                        request.to_bytes(&mut stream).expect(
                            "Failed to send request",
                        );
                    }
                } else {
                    eprintln!("Received {:?} instead of subscribed event", message);
                }
            }
            TransportMessage::RequestResponse(response) => {
                let message = response.message();
                if let RequestResponseMessage::ExecuteCommandResponse(ref message) = *message {
                    let event = TaskEvent::from_data(message).expect("Failed to read task event");
                    println!("Event {:?}", event);
                } else if let RequestResponseMessage::ControlMessageResponse(ref message) = *message {
                    let response = TaskSubscription::from_data(message).expect("Failed to read task subcription");
                    println!("Subscription {:?}", response);
                    subscription.set_subscriber_key(response.subscriber_key());
                }
            }
            _ => eprintln!("Unsupported message {:?}", request),
        }

    }

}
