#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let resource = 1.0;
    let response = client
        .get_resources_reviews_by_resource(resource)
        .fields("your fields")
        .page(1.0)
        .size(1.0)
        .sort("your sort")
        .await
        .unwrap();
    println!("{:#?}", response);
}