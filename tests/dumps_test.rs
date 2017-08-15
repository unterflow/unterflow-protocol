extern crate unterflow_protocol;

use unterflow_protocol::*;
use unterflow_protocol::frame::*;
use unterflow_protocol::io::*;
use unterflow_protocol::message::*;
use unterflow_protocol::sbe::*;

macro_rules! dump_vec {
    ($v:ident, $file:expr) => (
        let $v = include_bytes!(concat!("dumps/", $file)).to_vec();
    )
}

macro_rules! dump {
    ($reader:ident, $file:expr) => (
        dump_vec!(data, $file);
        let mut $reader: &[u8] = &data[..];
    )
}

#[test]
fn topology_request_manual() {
    dump!(reader, "topology-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(22, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(256), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageRequest::message_header(), message_header);

    let request = ControlMessageRequest::from_bytes(&mut reader).unwrap();
    assert_eq!(&ControlMessageType::RequestTopology, request.message_type());
    assert!(TopologyRequest::from_data(&request).is_ok());

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn topology_request_read() {
    dump!(reader, "topology-request.bin");

    let request = TransportMessage::from_bytes(&mut reader).unwrap();

    if let TransportMessage::RequestResponse(request) = request {
        let message = request.message();
        if let RequestResponseMessage::ControlMessageRequest(ref message) = *message {
            let topology = TopologyRequest::from_data(message).unwrap();
            assert_eq!(TopologyRequest {}, topology);
            assert_eq!(0, reader.len());
        } else {
            panic!("Expected control message request, got {:?}", message);
        }
    } else {
        panic!("Expected request response, got {:?}", request);
    }
}

#[test]
fn topology_request_write() {
    dump_vec!(expected, "topology-request.bin");

    let mut buffer = vec![];

    let message = ControlMessageType::RequestTopology
        .with(&TopologyRequest {})
        .unwrap();
    let request = TransportMessage::request(256, message);

    request.to_bytes(&mut buffer).unwrap();

    assert_eq!(expected, buffer);
}

#[test]
fn topology_response_manual() {
    dump!(reader, "topology-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(125, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(256), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageResponse::message_header(), message_header);

    let response = ControlMessageResponse::from_bytes(&mut reader).unwrap();
    let topology = TopologyResponse::from_data(&response).unwrap();

    assert_eq!(
        &vec![
            TopicLeader::new(
                "0.0.0.0".to_string(),
                51_015,
                "default-topic".to_string(),
                0
            ),
        ],
        topology.topic_leaders()
    );
    assert_eq!(
        &vec![SocketAddress::new("0.0.0.0".to_string(), 51_015)],
        topology.brokers()
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn topology_response_read() {
    dump!(reader, "topology-response.bin");

    let response = TransportMessage::from_bytes(&mut reader).unwrap();

    if let TransportMessage::RequestResponse(response) = response {
        let message = response.message();
        if let RequestResponseMessage::ControlMessageResponse(ref message) = *message {
            let topology = TopologyResponse::from_data(message).unwrap();

            assert_eq!(
                &vec![
                    TopicLeader::new(
                        "0.0.0.0".to_string(),
                        51_015,
                        "default-topic".to_string(),
                        0
                    ),
                ],
                topology.topic_leaders()
            );
            assert_eq!(
                &vec![SocketAddress::new("0.0.0.0".to_string(), 51_015)],
                topology.brokers()
            );

            assert_eq!(0, reader.len());
        } else {
            panic!("Expected control message response, got {:?}", message);
        }
    } else {
        panic!("Expected request response, got {:?}", response);
    }
}

#[test]
fn create_task_request() {
    dump!(reader, "create-task-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(158, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ExecuteCommandRequest::message_header(), message_header);

    let request = ExecuteCommandRequest::from_bytes(&mut reader).unwrap();
    let task = TaskEvent::from_data(&request).unwrap();
    let mut expected = TaskEvent::new("CREATE", "foo", 3);
    expected.add_custom_header("k1", "a");
    expected.add_custom_header("k2", "b");
    expected.set_payload(vec![129, 167, 112, 97, 121, 108, 111, 97, 100, 123]);
    assert_eq!(expected, task);

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn create_task_response() {
    dump!(reader, "create-task-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(278, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ExecuteCommandResponse::message_header(), message_header);

    let response = ExecuteCommandResponse::from_bytes(&mut reader).unwrap();
    let task = TaskEvent::from_data(&response).unwrap();
    let mut expected = TaskEvent::new("CREATED", "foo", 3);
    expected.add_custom_header("k1", "a");
    expected.add_custom_header("k2", "b");
    expected.set_payload(vec![129, 167, 112, 97, 121, 108, 111, 97, 100, 123]);
    assert_eq!(expected, task);

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn open_task_subscription_request() {
    dump!(reader, "open-task-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(129, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageRequest::message_header(), message_header);

    let request = ControlMessageRequest::from_bytes(&mut reader).unwrap();
    assert_eq!(
        &ControlMessageType::AddTaskSubscription,
        request.message_type()
    );

    let subscription = TaskSubscription::from_data(&request).unwrap();

    assert_eq!(
        TaskSubscription::new("default-topic", 0, "foo", "test", 300_000, 0, 32),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn open_task_subscription_response() {
    dump!(reader, "open-task-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(128, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageResponse::message_header(), message_header);

    let response = ControlMessageResponse::from_bytes(&mut reader).unwrap();
    let subscription = TaskSubscription::from_data(&response).unwrap();

    assert_eq!(
        TaskSubscription::new("default-topic", 0, "foo", "test", 300_000, 0, 32),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn task_subscription_locked_task() {
    dump!(reader, "task-subscription-locked-task.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(264, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::FullDuplexSingleMessage),
        transport_header
    );

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(SubscribedEvent::message_header(), message_header);

    let response = SubscribedEvent::from_bytes(&mut reader).unwrap();
    let task = TaskEvent::from_data(&response).unwrap();

    let mut expected = TaskEvent::new("LOCKED", "foo", 3);
    expected.set_lock_owner("test");
    expected.set_lock_time(1_502_612_949_248);
    expected.set_payload(vec![192]);
    assert_eq!(expected, task);

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn close_task_subscription_request() {
    dump!(reader, "close-task-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(97, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageRequest::message_header(), message_header);

    let request = ControlMessageRequest::from_bytes(&mut reader).unwrap();
    assert_eq!(
        &ControlMessageType::RemoveTaskSubscription,
        request.message_type()
    );

    let subscription = TaskSubscription::from_data(&request).unwrap();

    assert_eq!(
        TaskSubscription::for_topic("default-topic".to_string(), 0),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn close_task_subscription_response() {
    dump!(reader, "close-task-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(124, 0, 0, DataFrameType::Message, 1),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageResponse::message_header(), message_header);

    let response = ControlMessageResponse::from_bytes(&mut reader).unwrap();
    let subscription = TaskSubscription::from_data(&response).unwrap();

    assert_eq!(
        TaskSubscription::new("default-topic", 0, "", "default", 0, 0, 0),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn open_topic_subscription_request() {
    dump!(reader, "open-topic-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(125, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ExecuteCommandRequest::message_header(), message_header);

    let request = ExecuteCommandRequest::from_bytes(&mut reader).unwrap();
    let subscriber = TopicSubscriber::from_data(&request).unwrap();
    assert_eq!(
        TopicSubscriber::new(0, "foo", "SUBSCRIBE", 32, false),
        subscriber
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn open_topic_subscription_response() {
    dump!(reader, "open-topic-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(125, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ExecuteCommandResponse::message_header(), message_header);

    let response = ExecuteCommandResponse::from_bytes(&mut reader).unwrap();
    let subscriber = TopicSubscriber::from_data(&response).unwrap();
    assert_eq!(
        TopicSubscriber::new(0, "foo", "SUBSCRIBED", 32, false),
        subscriber
    );


    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn close_topic_subscription_request() {
    dump!(reader, "close-topic-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(74, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageRequest::message_header(), message_header);

    let request = ControlMessageRequest::from_bytes(&mut reader).unwrap();
    assert_eq!(
        &ControlMessageType::RemoveTopicSubscription,
        request.message_type()
    );

    let subscription = CloseSubscription::from_data(&request).unwrap();

    assert_eq!(
        CloseSubscription::new("default-topic".to_string(), 0, 123),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn close_topic_subscription_response() {
    dump!(reader, "close-topic-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(73, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessageResponse::message_header(), message_header);

    let response = ControlMessageResponse::from_bytes(&mut reader).unwrap();
    let subscription = CloseSubscription::from_data(&response).unwrap();

    assert_eq!(
        CloseSubscription::new("default-topic".to_string(), 0, 123),
        subscription
    );

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn error_topic_not_found() {
    dump!(reader, "error-topic-not-found.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(356, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::RequestResponse),
        transport_header
    );

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(1), request_response_header);

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(ErrorResponse::message_header(), message_header);

    let response = ErrorResponse::from_bytes(&mut reader).unwrap();
    assert_eq!(ErrorResponse::new(ErrorCode::TopicNotFound, vec![67, 97, 110, 110, 111, 116, 32, 101, 120, 101, 99, 117, 116, 101, 32, 99, 111, 109, 109, 97, 110, 100, 46, 32, 84, 111, 112, 105, 99, 32, 119, 105, 116, 104, 32, 110, 97, 109, 101, 32, 39, 100, 101, 102, 97, 117, 108, 116, 45, 116, 111, 105, 99, 39, 32, 97, 110, 100, 32, 112, 97, 114, 116, 105, 116, 105, 111, 110, 32, 105, 100, 32, 39, 48, 39, 32, 110, 111, 116,32, 102, 111, 117, 110, 100], vec![19, 0, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 100, 101, 102, 97, 117, 108, 116, 45, 116, 111, 105, 99, 205, 0, 136, 165, 115, 116, 97, 116, 101, 166, 67, 82, 69, 65, 84, 69, 168, 108, 111, 99, 107, 84, 105, 109, 101, 211, 128, 0, 0, 0, 0, 0, 0, 0, 169, 108, 111, 99, 107, 79, 119, 110, 101, 114, 160, 167, 114, 101, 116, 114, 105, 101, 115, 3, 164, 116, 121, 112, 101, 163, 102, 111, 111, 167, 104, 101, 97, 100, 101, 114, 115, 134, 179, 119, 111, 114, 107, 102, 108, 111, 119, 73, 110, 115, 116, 97, 110, 99, 101, 75, 101, 121, 255, 173, 98, 112, 109, 110, 80, 114, 111, 99, 101, 115, 115, 73, 100, 160, 185, 119, 111, 114, 107, 102, 108, 111, 119, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 86, 101, 114, 115, 105, 111, 110, 255, 171, 119, 111, 114, 107, 102, 108, 111, 119, 75, 101, 121, 255, 170, 97, 99, 116, 105, 118, 105, 116, 121, 73, 100, 160, 179, 97, 99, 116, 105, 118, 105, 116, 121, 73, 110, 115, 116, 97, 110, 99, 101, 75, 101, 121, 255, 173, 99, 117, 115, 116, 111, 109, 72, 101, 97, 100, 101, 114, 115, 128, 167, 112, 97, 121, 108, 111, 97, 100, 196, 1, 192]), response);

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn keep_alive() {
    dump!(reader, "keep-alive.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(6, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::ControlMessage),
        transport_header
    );

    let control_message = ControlMessage::from_bytes(&mut reader).unwrap();
    assert_eq!(ControlMessage::KeepAlive, control_message);

    assert_eq!(data_frame_header.padding(), reader.len());
}

#[test]
fn append_request() {
    dump!(reader, "append-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        DataFrameHeader::new(218, 0, 0, DataFrameType::Message, 0),
        data_frame_header
    );
    assert_eq!(dump_length, data_frame_header.aligned_length());

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(
        TransportHeader::new(TransportProtocol::FullDuplexSingleMessage),
        transport_header
    );

    let message_header = MessageHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(AppendRequest::message_header(), message_header);

    let response = AppendRequest::from_bytes(&mut reader).unwrap();
    let data = vec![
        141,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        96,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        43,
        0,
        35,
        0,
        200,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        128,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        1,
        0,
        0,
        0,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        1,
        0,
        1,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        167,
        109,
        101,
        109,
        98,
        101,
        114,
        115,
        146,
        130,
        164,
        104,
        111,
        115,
        116,
        169,
        108,
        111,
        99,
        97,
        108,
        104,
        111,
        115,
        116,
        164,
        112,
        111,
        114,
        116,
        205,
        31,
        65,
        130,
        164,
        104,
        111,
        115,
        116,
        169,
        108,
        111,
        99,
        97,
        108,
        104,
        111,
        115,
        116,
        164,
        112,
        111,
        114,
        116,
        205,
        31,
        66,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    assert_eq!(
        AppendRequest::new(
            "default",
            0,
            1,
            4_294_967_296,
            1,
            4_294_967_392,
            "localhost",
            8001,
            data,
        ),
        response
    );

    assert_eq!(data_frame_header.padding(), reader.len());

}
