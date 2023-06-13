use std::collections::HashMap;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, Deserialize, Serialize)]
pub enum ConsumerResponse {
    Messages(ConsumerMessages),
    Error(ErrorMessage),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumerMessage {
    pub message_id: String,
    #[serde(alias = "MessageBodyMD5")]
    pub message_body_md5: Option<String>,
    pub message_body: String,
    pub receipt_handle: String,
    pub publish_time: String,
    pub first_consume_time: String,
    pub next_consume_time: String,
    pub consumed_times: String,
    pub message_tag: String,
    #[serde(deserialize_with = "parse_properties")]
    pub properties: Properties,
}

type Properties = std::collections::HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "Messages"))]
pub struct ConsumerMessages {
    #[serde(alias = "Message")]
    pub messages: Vec<ConsumerMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename(serialize = "Error"))]
pub struct ErrorMessage {
    pub code: String,
    pub message: String,
    pub request_id: String,
    pub host_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename(serialize = "ReceiptHandles"))]
pub(crate) struct AckMessageRequest {
    pub(crate) receipt_handle: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename(serialize = "Message"))]
pub(crate) struct PublishMessageRequest {
    pub(crate) message_body: String,
    pub(crate) message_tag: Option<String>,
    #[serde(serialize_with = "serialize_properties", deserialize_with = "parse_properties", skip_serializing_if = "HashMap::is_empty")]
    pub(crate) properties: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(deserialize = "Messaage"))]
pub struct PublishMessageResponse {
    #[serde(alias = "MessageId")]
    message_id: String,
    #[serde(alias = "MessageBodyMD5")]
    message_body_md5: Option<String>,
}


fn serialize_properties<S>(props: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut res = String::new();
    for (key, value) in props.iter() {
        if !res.is_empty() {
            res.push('|');
        }
        res.push_str(&format!("{}:{}", key, value));
    }
    serializer.serialize_str(&res)
}

fn parse_properties<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let re = regex::Regex::new(r"([^:|]+):([^:|]*)").unwrap();
    let mut props = HashMap::new();
    for cap in re.captures_iter(s.as_str()) {
        let key = cap[1].to_string();
        let value = cap[2].to_string();
        props.insert(key, value);
    }
    Ok(props)
}