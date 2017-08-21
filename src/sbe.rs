use io::{Data, FromBytes, HasBlockLength, HasData, HasMessageLength, Message, ToBytes, ToData};
use message::{COMPLETE_STATE, NIL, TaskEvent};
use std;

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct MessageHeader {
    pub block_length: u16,
    pub template_id: u16,
    pub schema_id: u16,
    pub version: u16,
}

pub trait ToMessageHeader {
    fn message_header() -> MessageHeader;
}

impl<'a, T: Message + HasBlockLength> From<&'a T> for MessageHeader {
    fn from(_: &'a T) -> Self {
        T::message_header()
    }
}

impl<T: Message + HasBlockLength> ToMessageHeader for T {
    fn message_header() -> MessageHeader {
        MessageHeader {
            block_length: T::block_length(),
            template_id: T::template_id(),
            schema_id: T::schema_id(),
            version: T::version(),
        }
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, HasMessageLength)]
pub enum ErrorCode {
    MessageNotSupported,
    TopicNotFound,
    RequestWriteFailure,
    InvalidClientVersion,
    RequestTimeout,
    RequestProcessingFailure,
    InvalidMessage,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "0", schema_id = "0", version = "1")]
#[data = "error_data"]
pub struct ErrorResponse {
    pub error_code: ErrorCode,
    pub error_data: Data,
    pub failed_request: Data,
}


#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, HasMessageLength)]
pub enum ControlMessageType {
    AddTaskSubscription,
    RemoveTaskSubscription,
    IncreaseTaskSubscriptionCredits,
    RemoveTopicSubscription,
    RequestTopology,
}

impl ControlMessageType {
    pub fn with<D: ToData>(self, data: &D) -> Result<ControlMessageRequest, std::io::Error> {
        Ok(ControlMessageRequest {
            message_type: self,
            data: data.to_data()?,
        })
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "10", schema_id = "0", version = "1")]
pub struct ControlMessageRequest {
    pub message_type: ControlMessageType,
    pub data: Data,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "11", schema_id = "0", version = "1")]
pub struct ControlMessageResponse {
    pub data: Data,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, HasMessageLength)]
pub enum EventType {
    TaskEvent,
    RaftEvent,
    SubscriptionEvent,
    SubscriberEvent,
    DeploymentEvent,
    WorkflowInstanceEvent,
    IncidentEvent,
    WorkflowEvent,
    NoopEvent,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "20", schema_id = "0", version = "1")]
#[data = "command"]
pub struct ExecuteCommandRequest {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
    pub event_type: EventType,
    pub topic_name: String,
    pub command: Data,
}

impl ExecuteCommandRequest {
    pub fn complete_task(message: &SubscribedEvent, mut event: TaskEvent) -> Result<Self, std::io::Error> {
        event.state = COMPLETE_STATE.into();
        if event.payload.is_empty() {
            event.payload = NIL.to_vec().into();
        }
        let command = event.to_data()?;
        Ok(ExecuteCommandRequest {
            topic_name: message.topic_name.clone(),
            partition_id: message.partition_id,
            position: message.position,
            key: message.key,
            event_type: EventType::TaskEvent,
            command,
        })
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "21", schema_id = "0", version = "1")]
#[data = "event"]
pub struct ExecuteCommandResponse {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
    pub topic_name: String,
    pub event: Data,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, HasMessageLength)]
pub enum SubscriptionType {
    TaskSubscription,
    TopicSubscription,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "30", schema_id = "0", version = "1")]
#[data = "event"]
pub struct SubscribedEvent {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
    pub subscriber_key: u64,
    pub subscription_type: SubscriptionType,
    pub event_type: EventType,
    pub topic_name: String,
    pub event: Data,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "10", schema_id = "4", version = "1")]
pub struct AppendRequest {
    pub partition_id: u16,
    pub term: u16,
    pub previous_event_position: u64,
    pub previous_event_term: i32,
    pub commit_position: u64,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
    pub data: Data,
}

#[cfg(test)]
mod test {
    use super::*;
    use byteorder::{LittleEndian, WriteBytesExt};
    use std::io::Write;

    #[test]
    fn test_message_header() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u16::<LittleEndian>(2).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_u16::<LittleEndian>(4).unwrap();

        let header = MessageHeader {
            block_length: 1,
            template_id: 2,
            schema_id: 3,
            version: 4,
        };

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(header, MessageHeader::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(8, MessageHeader::block_length());
    }

    #[test]
    fn test_control_message_request() {
        let mut buffer = vec![];

        buffer.write_u8(1).unwrap();
        buffer.write_u16::<LittleEndian>(2).unwrap();
        buffer.write_all(&[12, 13]).unwrap();

        let request = ControlMessageRequest {
            message_type: ControlMessageType::RemoveTaskSubscription,
            data: vec![12, 13].into(),
        };

        let mut bytes = vec![];
        request.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(
            request,
            ControlMessageRequest::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(
            MessageHeader {
                block_length: 1,
                template_id: 10,
                schema_id: 0,
                version: 1,
            },
            ControlMessageRequest::message_header()
        );

        assert_eq!(
            ControlMessageType::RemoveTaskSubscription,
            request.message_type
        );
    }

    #[test]
    fn test_control_message_response() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(2).unwrap();
        buffer.write_all(&[12, 13]).unwrap();

        let response = ControlMessageResponse { data: vec![12, 13].into() };

        let mut bytes = vec![];
        response.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(
            response,
            ControlMessageResponse::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(
            MessageHeader {
                block_length: 0,
                template_id: 11,
                schema_id: 0,
                version: 1,
            },
            ControlMessageResponse::message_header()
        );
    }

    #[test]
    fn test_execute_command_request() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u64::<LittleEndian>(2).unwrap();
        buffer.write_u64::<LittleEndian>(3).unwrap();
        buffer.write_u8(3).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all("foo".as_bytes()).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all(&[1, 2, 3]).unwrap();

        let request = ExecuteCommandRequest {
            topic_name: "foo".into(),
            partition_id: 1,
            position: 2,
            key: 3,
            event_type: EventType::SubscriberEvent,
            command: vec![1, 2, 3].into(),
        };

        let mut bytes = vec![];
        request.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(
            request,
            ExecuteCommandRequest::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(
            MessageHeader {
                block_length: 19,
                template_id: 20,
                schema_id: 0,
                version: 1,
            },
            ExecuteCommandRequest::message_header()
        );
    }

    #[test]
    fn test_execute_command_response() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u64::<LittleEndian>(2).unwrap();
        buffer.write_u64::<LittleEndian>(3).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all("foo".as_bytes()).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all(&[1, 2, 3]).unwrap();

        let response = ExecuteCommandResponse {
            topic_name: "foo".into(),
            partition_id: 1,
            position: 2,
            key: 3,
            event: vec![1, 2, 3].into(),
        };

        let mut bytes = vec![];
        response.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(
            response,
            ExecuteCommandResponse::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(
            MessageHeader {
                block_length: 18,
                template_id: 21,
                schema_id: 0,
                version: 1,
            },
            ExecuteCommandResponse::message_header()
        );
    }

    #[test]
    fn test_subscribed_event() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u64::<LittleEndian>(2).unwrap();
        buffer.write_u64::<LittleEndian>(3).unwrap();
        buffer.write_u64::<LittleEndian>(4).unwrap();
        buffer.write_u8(1).unwrap();
        buffer.write_u8(6).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all("foo".as_bytes()).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all(&[1, 2, 3]).unwrap();

        let response = SubscribedEvent {
            partition_id: 1,
            position: 2,
            key: 3,
            subscriber_key: 4,
            subscription_type: SubscriptionType::TopicSubscription,
            event_type: EventType::IncidentEvent,
            topic_name: "foo".into(),
            event: vec![1, 2, 3].into(),
        };

        let mut bytes = vec![];
        response.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(
            response,
            SubscribedEvent::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(
            MessageHeader {
                block_length: 28,
                template_id: 30,
                schema_id: 0,
                version: 1,
            },
            SubscribedEvent::message_header()
        );
    }

    #[test]
    fn test_append_request() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u16::<LittleEndian>(2).unwrap();
        buffer.write_u64::<LittleEndian>(3).unwrap();
        buffer.write_i32::<LittleEndian>(4).unwrap();
        buffer.write_u64::<LittleEndian>(5).unwrap();
        buffer.write_u16::<LittleEndian>(6).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all("foo".as_bytes()).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all("bar".as_bytes()).unwrap();
        buffer.write_u16::<LittleEndian>(3).unwrap();
        buffer.write_all(&[1, 2, 3]).unwrap();

        let event = AppendRequest {
            partition_id: 1,
            term: 2,
            previous_event_position: 3,
            previous_event_term: 4,
            commit_position: 5,
            port: 6,
            topic_name: "foo".into(),
            host: "bar".into(),
            data: vec![1, 2, 3].into(),
        };

        let mut bytes = vec![];
        event.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(event, AppendRequest::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(
            MessageHeader {
                block_length: 26,
                template_id: 10,
                schema_id: 4,
                version: 1,
            },
            AppendRequest::message_header()
        );
    }
}
