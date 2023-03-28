#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;

use reqwest::{Client, Error, Response, header::HeaderValue};
use serde_json::json;
use data_encoding::BASE64;

const RPC_USERNAME: &str = dotenv!("RPC_USERNAME");
const RPC_PASSWORD: &str = dotenv!("RPC_PASSWORD");


async fn make_rpc_call(method: &str, params: Vec<&str>) -> Result<Response, Error> {
    let client = Client::new();
    let rpc_request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    let auth_header_value = format!("{}:{}", RPC_USERNAME, RPC_PASSWORD);
    let encoded_auth_header_value = BASE64.encode(auth_header_value.as_bytes());
    let auth_header = HeaderValue::from_str(&format!("Basic {}", encoded_auth_header_value)).unwrap();

    let response = client.post("http://localhost:4001")
        .json(&rpc_request)
        .header("Authorization", auth_header)
        .send()
        .await?;

    Ok(response)
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let response = make_rpc_call("getinfo", vec![])
        .await
        .expect("Could not make RPC call");

    // Parse the JSON response
    let result: serde_json::Value = response.json()
        .await
        .expect("Could not parse JSON response");

    // Print the result
    println!("{}", result);

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}
