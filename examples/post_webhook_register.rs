#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let response = client.post_webhook_register().await.unwrap();
    println!("{:#?}", response);
}