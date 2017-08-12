extern crate unterflow_protocol;

use unterflow_protocol::*;
use unterflow_protocol::io::*;

macro_rules! dump {
    ($reader:ident, $file:expr) => (
        let data = include_bytes!(concat!("dumps/", $file)).to_vec();
        let mut $reader: &[u8] = &data[..];
    )
}

#[test]
fn topology_request() {
    dump!(reader, "topology-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(22, 0, 0, 0, 0), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(256), request_response_header);
}

#[test]
fn topology_response() {
    dump!(reader, "topology-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(125, 0, 0, 0, 0), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(256), request_response_header);
}

#[test]
fn create_task_request() {
    dump!(reader, "create-task-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(158, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);
}

#[test]
fn create_task_response() {
    dump!(reader, "create-task-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(278, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);
}

#[test]
fn open_task_subscription_request() {
    dump!(reader, "open-task-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(129, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);
}

#[test]
fn open_task_subscription_response() {
    dump!(reader, "open-task-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(128, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);
}

#[test]
fn close_task_subscription_request() {
    dump!(reader, "close-task-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(97, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);
}

#[test]
fn close_task_subscription_response() {
    dump!(reader, "close-task-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(124, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(259), request_response_header);
}

#[test]
fn open_topic_subscription_request() {
    dump!(reader, "open-topic-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(125, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);
}

#[test]
fn open_topic_subscription_response() {
    dump!(reader, "open-topic-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(133, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(257), request_response_header);
}

#[test]
fn close_topic_subscription_request() {
    dump!(reader, "close-topic-subscription-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(82, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);
}

#[test]
fn close_topic_subscription_response() {
    dump!(reader, "close-topic-subscription-response.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(81, 0, 0, 0, 1), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::RequestResponse),
               transport_header);

    let request_response_header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(RequestResponseHeader::new(258), request_response_header);
}

#[test]
fn keep_alive() {
    dump!(reader, "keep-alive.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(6, 0, 0, 0, 0), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::ControlMessage),
               transport_header);
}

#[test]
fn append_request() {
    dump!(reader, "append-request.bin");

    let dump_length = reader.len();

    let data_frame_header = DataFrameHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(DataFrameHeader::new(218, 0, 0, 0, 0), data_frame_header);
    assert_eq!(dump_length, data_frame_header.aligned_length() as usize);

    let transport_header = TransportHeader::from_bytes(&mut reader).unwrap();
    assert_eq!(TransportHeader::new(TransportProtocol::FullDuplexSingleMessage),
               transport_header);

}
