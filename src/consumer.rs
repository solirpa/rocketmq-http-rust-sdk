use crate::{
    client::Client, 
    conf::{ConsumerOption, ClientOption}, 
    error::ClientError, model::message::{ConsumerResponse, AckMessageRequest}
};

use std::collections::HashMap;

use log::debug;
use reqwest::Response;
use quick_xml::{de::from_str, se::to_string};

#[derive(Debug)]
pub struct Consumer {
    option: ConsumerOption,
    client: Client,
}

impl Consumer {
    
    pub fn new(
        option: ConsumerOption,
        client_option: ClientOption,
    ) -> Self {
        let client = Client::new(client_option);
        Self {
            option,
            client,
        }
    }

    pub async fn consumer_message(&self, number_of_messages: u8, wait_seconds: u8) -> Result<ConsumerResponse, ClientError> {
        let number_of_messages = number_of_messages.to_string();
        let wait_seconds = wait_seconds.to_string();

        let mut params = HashMap::new();
        params.insert("consumer", self.option.get_group());
        params.insert("numOfMessages", &number_of_messages);
        params.insert("waitSeconds", &wait_seconds);

        if let Some(tag) = self.option.get_tag() {
            params.insert("tag", tag);
        }

        if let Some(ns) = self.client.get_namespace() {
            params.insert("ns", ns);
        };

        let query = serde_urlencoded::to_string(&params).unwrap();
        let resource = format!("/topics/{}/messages?{}", self.option.get_topic(), query);

        let response = self.client.get(&resource).await?;

        let body = response.text().await?;
        let xml_data: ConsumerResponse = from_str(body.as_str()).unwrap();
        Ok(xml_data)
    }

    pub async fn ack(&self, receipt_handles: Vec<String>) -> Result<Response, ClientError> {
        let mut params = HashMap::new();
        if let Some(ns) = self.client.get_namespace() {
            params.insert("ns", ns);
        };
        params.insert("consumer", self.option.get_group());
        
        let query = serde_urlencoded::to_string(&params).unwrap();

        let ack = AckMessageRequest {
            receipt_handle: receipt_handles,
        };

        let ack = to_string(&ack)?;
        debug!("ack: {}", ack);
        let resource = format!("/topics/{}/messages?{}", self.option.get_topic(), query);
        let response = self.client.delete(&resource, ack).await?;
        Ok(response)
    }
}