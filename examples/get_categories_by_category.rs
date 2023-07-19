#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let category = 1.0;
    let response = client.get_categories_by_category(category).await.unwrap();
    println!("{:#?}", response);
}