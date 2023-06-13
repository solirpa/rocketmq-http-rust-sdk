use anyhow::Result;
use mq_http_rust_sdk::{
    Consumer, 
    conf::{ConsumerOption, ClientOption}, 
    model::message::ConsumerResponse
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client_option = ClientOption::default();
    client_option.set_endpoint("your_endpoint");
    client_option.set_access_key_id("your_access_key_id");
    client_option.set_access_key_secret("your_access_key_secret");
    client_option.set_namespace("your_namespace");

    let mut consumer_option = ConsumerOption::default();
    consumer_option.set_group("your_group");
    consumer_option.set_topic("your_topic");
    consumer_option.set_tag("your_tag");
    
    let consumer = Consumer::new(
        consumer_option,
        client_option,
    );

    let message = consumer.consumer_message(3, 3).await?;
    match message {
        ConsumerResponse::Messages(message) => {
            let receipt_handlers: Vec<String> = message.messages.into_iter().map(|x| x.receipt_handle).collect();
            println!("receipt_handlers: {:?}", receipt_handlers);
            let response = consumer.ack(receipt_handlers).await?;
            println!("response: {:?}", response);
        }
        ConsumerResponse::Error(message) => println!("error: {:?}", message),
    }

    Ok(())
}