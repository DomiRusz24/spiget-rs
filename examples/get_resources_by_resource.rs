#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let resource = 1.0;
    let response = client.get_resources_by_resource(resource).await.unwrap();
    println!("{:#?}", response);
}