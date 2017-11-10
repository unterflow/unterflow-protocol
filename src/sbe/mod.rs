pub mod zb_protocol;


pub use self::zb_protocol::ControlMessageType;
pub use self::zb_protocol::EventType;

use super::EventMetadata;

use error::Error;
use std;

pub const ANY_PARTITION: u16 = std::u16::MAX;

pub fn decode_control_message_response(data: &[u8]) -> Result<&[u8], Error> {
    let decoder = zb_protocol::start_decoding_control_message_response(data);
    let (header, decoder) = decoder.header()?;

    let expected_header = zb_protocol::ControlMessageResponseMessageHeader::default().message_header;

    if *header == expected_header {
        let (data, _) = decoder.data()?;
        Ok(data)
    } else {
        Err(Error::DecodeError(format!(
            "Expected SBE message {:?} but got {:?}",
            expected_header,
            header
        )))
    }
}

pub fn encode_control_message_request(partition_id: u16, message_type: ControlMessageType, data: &[u8]) -> Result<Vec<u8>, Error> {
    // +2 for the data length field (u16)
    let size = size_of!(
        zb_protocol::MessageHeader,
        zb_protocol::ControlMessageRequestFields
    ) + data.len() + 2;
    let mut buffer = vec![0u8; size];
    let size = {
        let encoder = zb_protocol::start_encoding_control_message_request(&mut buffer);

        let encoder = encoder.header_copy(
            &zb_protocol::ControlMessageRequestMessageHeader::default().message_header,
        )?;

        let (fields, encoder) = encoder.control_message_request_fields()?;
        fields.message_type = message_type;
        fields.partition_id = partition_id;

        let done = encoder.data(data)?;

        done.unwrap()
    };

    buffer.truncate(size);

    Ok(buffer)
}

impl<'d> From<&'d zb_protocol::ExecuteCommandResponseFields> for EventMetadata {
    fn from(fields: &zb_protocol::ExecuteCommandResponseFields) -> Self {
        EventMetadata {
            partition_id: fields.partition_id,
            position: fields.position,
            key: fields.key,
        }
    }
}

pub fn decode_execute_command_response(data: &[u8]) -> Result<(EventMetadata, &[u8]), Error> {
    let decoder = zb_protocol::start_decoding_execute_command_response(data);
    let (header, decoder) = decoder.header()?;

    let expected_header = zb_protocol::ExecuteCommandResponseMessageHeader::default().message_header;

    if *header == expected_header {
        let (fields, decoder) = decoder.execute_command_response_fields()?;
        let (data, _) = decoder.event()?;
        Ok((fields.into(), data))
    } else {
        Err(Error::DecodeError(format!(
            "Expected SBE message {:?} but got {:?}",
            expected_header,
            header
        )))
    }
}

pub fn encode_execute_command_request(partition_id: u16, event_type: EventType, command: &[u8]) -> Result<Vec<u8>, Error> {
    // +2 for the command length field (u16)
    let size = size_of!(
        zb_protocol::MessageHeader,
        zb_protocol::ExecuteCommandRequestFields
    ) + command.len() + 2;
    let mut buffer = vec![0u8; size];
    let size = {
        let encoder = zb_protocol::start_encoding_execute_command_request(&mut buffer);

        let encoder = encoder.header_copy(
            &zb_protocol::ExecuteCommandRequestMessageHeader::default().message_header,
        )?;

        let (fields, encoder) = encoder.execute_command_request_fields()?;
        fields.partition_id = partition_id;
        fields.position = ::std::u64::MAX;
        fields.key = ::std::u64::MAX;
        fields.event_type = event_type;

        let done = encoder.command(command)?;

        done.unwrap()
    };

    buffer.truncate(size);

    Ok(buffer)
}
