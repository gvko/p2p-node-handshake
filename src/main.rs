#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;

use reqwest::{Client, Response, header::{HeaderValue, CONTENT_TYPE, AUTHORIZATION}};
use serde_json::json;
use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
use std::io::{Error, ErrorKind};

#[derive(ThisError, Debug)]
pub enum ApiError {
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("RPC Error: {0}")]
    RPCError(String),
    #[error("RequestError: {0}")]
    RequestError(#[from] reqwest::Error),
}
impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::new(ErrorKind::Other, err)
    }
}

const RPC_USERNAME: &str = dotenv!("RPC_USERNAME");
const RPC_PASSWORD: &str = dotenv!("RPC_PASSWORD");
const RPC_DOMAIN_AND_PORT: &str = dotenv!("RPC_DOMAIN_AND_PORT");

async fn make_rpc_call(method: &str, params: Vec<&str>) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let rpc_request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "headers": {
            "Content-Type": "application/json",
        },
        "id": 1
    });

    let auth_header_value = format!("{}:{}", RPC_USERNAME, RPC_PASSWORD);
    let encoded_auth_header_value = BASE64.encode(auth_header_value.as_bytes());
    let auth_header = HeaderValue::from_str(&format!("Basic {}", encoded_auth_header_value)).unwrap();

    let response = client.post(RPC_DOMAIN_AND_PORT)
        .json(&rpc_request)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    Ok(response)
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let response = make_rpc_call("getinfo", vec![])
        .await
        .expect("Could not make RPC call");

    let result: serde_json::Value = response.json()
        .await
        .expect("Could not parse JSON response");

    println!("=========== getaddresses ==========");
    println!("{}", result);

    let response = make_rpc_call("getaddresses", vec![])
        .await
        .expect("Could not make RPC call");

    let result: serde_json::Value = response.json()
        .await
        .expect("Could not parse JSON response");

    println!("\n=========== getaddresses ==========");
    println!("{}", result);

    // Send a message containing the client's public key
    let response = make_rpc_call("validateaddress", vec!["1YXWvSKFm4XG5yPiFcXGGSvEwH4D8K2nRcXWeA"])
        .await
        .expect("Could not make RPC call: ");

    let result: serde_json::Value = response.json()
        .await
        .expect("Could not parse JSON response");

    println!("\n=========== validateaddress ==========");
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
