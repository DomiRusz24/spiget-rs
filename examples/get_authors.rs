#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let response = client
        .get_authors()
        .fields("your fields")
        .page(1.0)
        .size(1.0)
        .sort("your sort")
        .await
        .unwrap();
    println!("{:#?}", response);
}