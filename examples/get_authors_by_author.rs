#![allow(unused_imports)]
use spiget::SpigetClient;
use spiget::model::*;
#[tokio::main]
async fn main() {
    let client = SpigetClient::from_env();
    let author = 1.0;
    let response = client.get_authors_by_author(author).await.unwrap();
    println!("{:#?}", response);
}