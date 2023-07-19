#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let id = "your id";
    let response = client.get_webhook_status_by_id(id).await.unwrap();
    println!("{:#?}", response);
}