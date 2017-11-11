#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
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
    pub partition_id: u16,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Broker {
    pub host: String,
    pub port: u32,
}
