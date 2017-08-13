use io::{Data, FromBytes, HasBlockLength, HasData, HasMessageLength, Message, ToBytes, ToData};
use std;

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct MessageHeader {
    block_length: u16,
    template_id: u16,
    schema_id: u16,
    version: u16,
}

impl MessageHeader {
    pub fn new(block_length: u16, template_id: u16, schema_id: u16, version: u16) -> Self {
        MessageHeader {
            block_length,
            template_id,
            schema_id,
            version,
        }
    }
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
pub enum ControlMessageType {
    AddTaskSubscription,
    RemoveTaskSubscription,
    IncreaseTaskSubscriptionCredits,
    RemoveTopicSubscription,
    RequestTopology,
}

impl ControlMessageType {
    pub fn with<D: ToData>(self, data: D) -> Result<ControlMessageRequest, std::io::Error> {
        Ok(ControlMessageRequest {
               message_type: self,
               data: data.to_data()?,
           })
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "10", schema_id = "0", version = "1")]
pub struct ControlMessageRequest {
    message_type: ControlMessageType,
    data: Data,
}

impl ControlMessageRequest {
    pub fn new<T>(message_type: ControlMessageType, data: T) -> Self
        where T: Into<Data>
    {
        ControlMessageRequest {
            message_type,
            data: data.into(),
        }
    }

    pub fn message_type(&self) -> &ControlMessageType {
        &self.message_type
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "11", schema_id = "0", version = "1")]
pub struct ControlMessageResponse {
    data: Data,
}

impl ControlMessageResponse {
    pub fn new<T>(data: T) -> Self
        where T: Into<Data>
    {
        ControlMessageResponse { data: data.into() }
    }
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
    partition_id: u16,
    position: u64,
    key: u64,
    event_type: EventType,
    topic_name: String,
    command: Data,
}

impl ExecuteCommandRequest {
    pub fn new<T>(topic_name: String, partition_id: u16, position: u64, key: u64, event_type: EventType, command: T) -> Self
        where T: Into<Data>
    {
        ExecuteCommandRequest {
            topic_name,
            partition_id,
            position,
            key,
            event_type,
            command: command.into(),
        }
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "21", schema_id = "0", version = "1")]
#[data = "event"]
pub struct ExecuteCommandResponse {
    partition_id: u16,
    position: u64,
    key: u64,
    topic_name: String,
    event: Data,
}

impl ExecuteCommandResponse {
    pub fn new<T>(topic_name: String, partition_id: u16, position: u64, key: u64, event: T) -> Self
        where T: Into<Data>
    {
        ExecuteCommandResponse {
            topic_name,
            partition_id,
            position,
            key,
            event: event.into(),
        }
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, HasMessageLength)]
enum SubscriptionType {
    TaskSubscription,
    TopicSubscription,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "30", schema_id = "0", version = "1")]
#[data = "event"]
pub struct SubscribedEvent {
    partition_id: u16,
    position: u64,
    key: u64,
    subscriber_key: u64,
    subscription_type: SubscriptionType,
    event_type: EventType,
    topic_name: String,
    event: Data,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength, Message, HasData, HasMessageLength)]
#[message(template_id = "10", schema_id = "4", version = "1")]
pub struct AppendRequest {
    partition_id: u16,
    term: u16,
    previous_event_position: u64,
    previous_event_term: i32,
    commit_position: u64,
    port: u16,
    topic_name: String,
    host: String,
    data: Data,
}

impl AppendRequest {
    pub fn new<S: Into<String>, D: Into<Data>>(topic_name: S,
                                               partition_id: u16,
                                               term: u16,
                                               previous_event_position: u64,
                                               previous_event_term: i32,
                                               commit_position: u64,
                                               host: S,
                                               port: u16,
                                               data: D)
                                               -> Self {
        AppendRequest {
            partition_id,
            term,
            previous_event_position,
            previous_event_term,
            commit_position,
            port,
            topic_name: topic_name.into(),
            host: host.into(),
            data: data.into(),
        }
    }
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

        let header = MessageHeader::new(1, 2, 3, 4);

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

        let request = ControlMessageRequest::new(ControlMessageType::RemoveTaskSubscription, vec![12, 13]);

        let mut bytes = vec![];
        request.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(request,
                   ControlMessageRequest::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(MessageHeader::new(1, 10, 0, 1),
                   ControlMessageRequest::message_header());

        assert_eq!(&ControlMessageType::RemoveTaskSubscription,
                   request.message_type());
    }

    #[test]
    fn test_control_message_response() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(2).unwrap();
        buffer.write_all(&[12, 13]).unwrap();

        let response = ControlMessageResponse::new(vec![12, 13]);

        let mut bytes = vec![];
        response.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(response,
                   ControlMessageResponse::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(MessageHeader::new(0, 11, 0, 1),
                   ControlMessageResponse::message_header());
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

        let request = ExecuteCommandRequest::new("foo".to_string(),
                                                 1,
                                                 2,
                                                 3,
                                                 EventType::SubscriberEvent,
                                                 vec![1, 2, 3]);

        let mut bytes = vec![];
        request.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(request,
                   ExecuteCommandRequest::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(MessageHeader::new(19, 20, 0, 1),
                   ExecuteCommandRequest::message_header());
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

        let response = ExecuteCommandResponse::new("foo".to_string(), 1, 2, 3, vec![1, 2, 3]);

        let mut bytes = vec![];
        response.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);
        assert_eq!(response,
                   ExecuteCommandResponse::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(MessageHeader::new(18, 21, 0, 1),
                   ExecuteCommandResponse::message_header());
    }

}
