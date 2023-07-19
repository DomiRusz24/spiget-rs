#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let resource = 1.0;
    let version = 1.0;
    let response = client
        .get_resources_versions_by_version(resource, version)
        .await
        .unwrap();
    println!("{:#?}", response);
}