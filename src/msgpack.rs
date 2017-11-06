pub const EMPTY_MAP: u8 = 0x80;
pub const EMPTY_ARRAY: u8 = 0x90;
pub const EMPTY_NIL: u8 = 0xc0;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyResponse {
    pub topic_leaders: Vec<TopicLeader>,
    pub brokers: Vec<Broker>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicLeader {
    pub host: String,
    pub port: u32,
    pub topic_name: String,
    pub partition_id: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Broker {
    pub host: String,
    pub port: u32,
}
