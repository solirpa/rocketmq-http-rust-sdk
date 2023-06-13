use std::collections::HashMap;

use quick_xml::{de::from_str, se::to_string};

use crate::{
    client::Client, conf::{ProducerOption, ClientOption}, 
    model::message::{PublishMessageRequest, PublishMessageResponse}, 
    error::ClientError
};

#[derive(Debug)]
pub struct Producer {
    option: ProducerOption,
    client: Client,
}

impl Producer {
    
    pub fn new(
        option: ProducerOption,
        client_option: ClientOption,
    ) -> Self {
        let client = Client::new(client_option);
        Self {
            option,
            client,
        }
    }

    pub async fn publish_message(
        &self,
        body: String,
        tag: String,
        msg_props: HashMap<String, String>,
    ) -> Result<PublishMessageResponse, ClientError> {
        let request = PublishMessageRequest {
            message_body: body,
            message_tag: Some(tag),
            properties: msg_props,
        };

        let mut params = HashMap::new();
        if let Some(ns) = self.client.get_namespace() {
            params.insert("ns", ns);
        };

        let query = serde_urlencoded::to_string(&params).unwrap();
        let resource = format!("/topics/{}/messages?{}", self.option.get_topic(), query);
        let request = to_string(&request)?;
        let response = self.client.post(&resource, request).await?;
        let body = response.text().await?;
        let xml_data: PublishMessageResponse = from_str(body.as_str())?;
        Ok(xml_data)
    }
}