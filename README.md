<div id="top"></div>

<p align="center">
    <a href="https://github.com/domirusz24/spiget-rs/stargazers">
        <img src="https://img.shields.io/github/stars/domirusz24/spiget-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/domirusz24/spiget-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/domirusz24/spiget-rs/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/spiget">
    <img src="https://img.shields.io/crates/d/spiget?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/spiget">
    <img src="https://img.shields.io/crates/v/spiget?style=flat-square" alt="Crates.io" />
</a>

</p>

Spiget client, generated from the OpenAPI spec.

# Usage

```rust
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
```

This example loads configuration from environment variables, specifically:

* `SPIGET_BASE_URL`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
spiget = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/spiget)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*