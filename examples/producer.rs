use anyhow::Result;
use std::collections::HashMap;
use mq_http_rust_sdk::{
    Producer, 
    conf::{ProducerOption, ClientOption},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client_option = ClientOption::default();
    client_option.set_endpoint("your_endpoint");
    client_option.set_access_key_id("your_access_key_id");
    client_option.set_access_key_secret("your_access_key_secret");
    client_option.set_namespace("your_namespace");

    let mut producer_option = ProducerOption::default();
    producer_option.set_topic("your_topic");
    
    let producer = Producer::new(
        producer_option,
        client_option,
    );

    let message = r#"your_message"#.to_string();

    let message = producer.publish_message(message, "your_tag".to_string(), HashMap::new()).await?;
    println!("message: {:?}", message);

    Ok(())
}