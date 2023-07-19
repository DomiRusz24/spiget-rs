#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let id = "your id";
    let secret = "your secret";
    let response = client.delete_webhook_delete_by_secret(id, secret).await.unwrap();
    println!("{:#?}", response);
}