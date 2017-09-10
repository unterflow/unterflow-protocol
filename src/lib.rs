extern crate byteorder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate rmp_serde;
#[macro_use]
extern crate unterflow_protocol_derive;

pub mod frame;
pub mod io;
pub mod message;
pub mod sbe;

use frame::*;
use io::*;
use sbe::{AppendRequest, ControlMessageRequest, ControlMessageResponse, ExecuteCommandRequest, ExecuteCommandResponse, MessageHeader,
          SubscribedEvent, ToMessageHeader};

use std::io::{Read, Write};

#[derive(Debug)]
pub enum RequestResponseMessage {
    ControlMessageRequest(ControlMessageRequest),
    ControlMessageResponse(ControlMessageResponse),
    ExecuteCommandRequest(ExecuteCommandRequest),
    ExecuteCommandResponse(ExecuteCommandResponse),
}

impl RequestResponseMessage {
    pub fn read<R: Read>(message_header: &MessageHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        if message_header == &ControlMessageRequest::message_header() {
            let message = ControlMessageRequest::from_bytes(reader)?;
            Ok(RequestResponseMessage::ControlMessageRequest(message))
        } else if message_header == &ControlMessageResponse::message_header() {
            let message = ControlMessageResponse::from_bytes(reader)?;
            Ok(RequestResponseMessage::ControlMessageResponse(message))
        } else if message_header == &ExecuteCommandRequest::message_header() {
            let message = ExecuteCommandRequest::from_bytes(reader)?;
            Ok(RequestResponseMessage::ExecuteCommandRequest(message))
        } else if message_header == &ExecuteCommandResponse::message_header() {
            let message = ExecuteCommandResponse::from_bytes(reader)?;
            Ok(RequestResponseMessage::ExecuteCommandResponse(message))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Unsupported request response message {:?}",
                    message_header
                ),
            ))
        }
    }
}

impl From<ControlMessageRequest> for RequestResponseMessage {
    fn from(message: ControlMessageRequest) -> Self {
        RequestResponseMessage::ControlMessageRequest(message)
    }
}

impl From<ControlMessageResponse> for RequestResponseMessage {
    fn from(message: ControlMessageResponse) -> Self {
        RequestResponseMessage::ControlMessageResponse(message)
    }
}

impl From<ExecuteCommandRequest> for RequestResponseMessage {
    fn from(message: ExecuteCommandRequest) -> Self {
        RequestResponseMessage::ExecuteCommandRequest(message)
    }
}

impl From<ExecuteCommandResponse> for RequestResponseMessage {
    fn from(message: ExecuteCommandResponse) -> Self {
        RequestResponseMessage::ExecuteCommandResponse(message)
    }
}

impl ToBytes for RequestResponseMessage {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), std::io::Error> {
        let message: &ToBytes = match *self {
            RequestResponseMessage::ControlMessageRequest(ref m) => m,
            RequestResponseMessage::ControlMessageResponse(ref m) => m,
            RequestResponseMessage::ExecuteCommandRequest(ref m) => m,
            RequestResponseMessage::ExecuteCommandResponse(ref m) => m,
        };

        message.to_bytes(writer)
    }
}


#[derive(Debug)]
pub struct RequestResponse {
    pub frame_header: DataFrameHeader,
    pub transport_header: TransportHeader,
    pub request_header: RequestResponseHeader,
    pub message_header: MessageHeader,
    pub message: RequestResponseMessage,
}

impl RequestResponse {
    pub fn read<R: Read>(frame_header: DataFrameHeader, transport_header: TransportHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        let request_header = RequestResponseHeader::from_bytes(reader)?;
        let message_header = MessageHeader::from_bytes(reader)?;
        let message = RequestResponseMessage::read(&message_header, reader)?;

        Ok(RequestResponse {
            frame_header,
            transport_header,
            request_header,
            message_header,
            message,
        })
    }

    pub fn message(&self) -> &RequestResponseMessage {
        &self.message
    }
}

impl ToBytes for RequestResponse {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), std::io::Error> {
        let mut buffer = vec![0u8; self.frame_header.aligned_length()];

        {
            let mut buffer = buffer.as_mut_slice();
            self.frame_header.to_bytes(&mut buffer)?;
            self.transport_header.to_bytes(&mut buffer)?;
            self.request_header.to_bytes(&mut buffer)?;
            self.message_header.to_bytes(&mut buffer)?;
            self.message.to_bytes(&mut buffer)?;
        }

        writer.write_all(buffer.as_slice())
    }
}

#[derive(Debug)]
pub enum SingleRequestMessage {
    SubscribedEvent(SubscribedEvent),
    AppendRequest(AppendRequest),
}

impl SingleRequestMessage {
    pub fn read<R: Read>(message_header: &MessageHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        if message_header == &SubscribedEvent::message_header() {
            let message = SubscribedEvent::from_bytes(reader)?;
            Ok(SingleRequestMessage::SubscribedEvent(message))
        } else if message_header == &AppendRequest::message_header() {
            let message = AppendRequest::from_bytes(reader)?;
            Ok(SingleRequestMessage::AppendRequest(message))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Unsupported single request message {:?}",
                    message_header
                ),
            ))
        }
    }
}

#[derive(Debug)]
pub struct SingleRequest {
    pub frame_header: DataFrameHeader,
    pub transport_header: TransportHeader,
    pub message_header: MessageHeader,
    pub message: SingleRequestMessage,
}

impl SingleRequest {
    pub fn read<R: Read>(frame_header: DataFrameHeader, transport_header: TransportHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        let message_header = MessageHeader::from_bytes(reader)?;
        let message = SingleRequestMessage::read(&message_header, reader)?;

        Ok(SingleRequest {
            frame_header,
            transport_header,
            message_header,
            message,
        })
    }

    pub fn message(&self) -> &SingleRequestMessage {
        &self.message
    }
}

#[derive(Debug)]
pub struct ControlRequest {
    pub frame_header: DataFrameHeader,
    pub transport_header: TransportHeader,
    pub message: ControlMessage,
}

impl ControlRequest {
    pub fn read<R: Read>(frame_header: DataFrameHeader, transport_header: TransportHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        let message = ControlMessage::from_bytes(reader)?;

        Ok(ControlRequest {
            frame_header,
            transport_header,
            message,
        })
    }

    pub fn message(&self) -> &ControlMessage {
        &self.message
    }
}


#[derive(Debug)]
pub enum TransportMessage {
    RequestResponse(RequestResponse),
    SingleRequest(SingleRequest),
    ControlRequest(ControlRequest),
}

impl TransportMessage {
    pub fn request<M: Into<RequestResponseMessage> + ToMessageHeader + HasMessageLength>(request_id: u64, message: M) -> Self {
        let length = u32::from(TransportHeader::block_length()) + u32::from(RequestResponseHeader::block_length()) +
            u32::from(MessageHeader::block_length()) + message.message_length();

        let request_response = RequestResponse {
            frame_header: DataFrameHeader {
                length,
                version: 0,
                flags: 0,
                frame_type: DataFrameType::Message,
                stream_id: 0,
            },
            transport_header: TransportHeader { protocol: TransportProtocol::RequestResponse },
            request_header: RequestResponseHeader { request_id: request_id },
            message_header: M::message_header(),
            message: message.into(),
        };

        TransportMessage::RequestResponse(request_response)
    }

    pub fn read<R: Read>(frame_header: DataFrameHeader, reader: &mut R) -> Result<Self, std::io::Error> {
        let transport_header = TransportHeader::from_bytes(reader)?;
        match transport_header.protocol {
            TransportProtocol::RequestResponse => {
                let message = RequestResponse::read(frame_header, transport_header, reader)?;
                Ok(TransportMessage::RequestResponse(message))
            }
            TransportProtocol::FullDuplexSingleMessage => {
                let message = SingleRequest::read(frame_header, transport_header, reader)?;
                Ok(TransportMessage::SingleRequest(message))
            }
            TransportProtocol::ControlMessage => {
                let message = ControlRequest::read(frame_header, transport_header, reader)?;
                Ok(TransportMessage::ControlRequest(message))
            }
        }
    }

    pub fn length(&self) -> usize {
        match *self {
            TransportMessage::RequestResponse(ref r) => r.frame_header.aligned_length(),
            TransportMessage::ControlRequest(ref r) => r.frame_header.aligned_length(),
            TransportMessage::SingleRequest(ref r) => r.frame_header.aligned_length(),
        }
    }
}

impl ToBytes for TransportMessage {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), std::io::Error> {
        let message: &ToBytes = match *self {
            TransportMessage::RequestResponse(ref m) => m,
            _ => unimplemented!(),
        };

        message.to_bytes(writer)
    }
}

impl FromBytes for TransportMessage {
    fn from_bytes(reader: &mut Read) -> Result<Self, std::io::Error> {
        let frame_header = DataFrameHeader::from_bytes(reader)?;
        match frame_header.frame_type {
            DataFrameType::Message => {
                let length = frame_header.aligned_length() - DataFrameHeader::block_length() as usize;
                let mut buffer = vec![0; length];
                {
                    let mut buffer = buffer.as_mut_slice();
                    reader.read_exact(&mut buffer)?;
                }

                let mut buffer = buffer.as_slice();
                TransportMessage::read(frame_header, &mut buffer)
            }
            _ => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Expected message but received {:?}", frame_header),
                ))
            }
        }
    }
}
