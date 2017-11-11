#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicEvent {
    pub state: TopicState,
    pub name: String,
    pub partitions: u32,
}

#[derive(Debug, PartialEq)]
pub enum TopicState {
    Create,
    Created,
    CreateRejected,
}

enum_serialize! {
    TopicState => {
        TopicState::Create => "CREATE",
        TopicState::Created => "CREATED",
        TopicState::CreateRejected => "CREATE_REJECTED"
    }
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicSubscriberEvent {
    pub state: TopicSubscriberState,
    pub name: String,
    pub prefetch_capacity: u32,
    pub start_position: i64,
    pub force_start: bool,
}

impl Default for TopicSubscriberEvent {
    fn default() -> Self {
        TopicSubscriberEvent {
            state: TopicSubscriberState::Subscribe,
            name: String::new(),
            prefetch_capacity: 0,
            start_position: -1,
            force_start: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TopicSubscriberState {
    Subscribe,
    Subscribed,
}

enum_serialize! {
    TopicSubscriberState => {
        TopicSubscriberState::Subscribe => "SUBSCRIBE",
        TopicSubscriberState::Subscribed => "SUBSCRIBED"
    }
}
