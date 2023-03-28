#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;

use data_encoding::BASE64;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use thiserror::Error as ThisError;

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

struct RpcMethods;

impl RpcMethods {
    const GETINFO: &'static str = "getinfo";
    const LISTADDRESSES: &'static str = "listaddresses";
    const VALIDATEADDRESS: &'static str = "validateaddress";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    error: Option<serde_json::Value>,
    id: u32,
    result: ApiResult,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResult {
    Info(GetInfoResult),
    Addresses(Vec<ListAddressesResult>),
    ValidateAddress(ValidateAddress),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetInfoResult {
    balance: f64,
    blocks: u64,
    burnaddress: String,
    chainname: String,
    connections: u64,
    description: String,
    difficulty: f64,
    errors: String,
    nodeaddress: String,
    nodeversion: u32,
    paytxfee: f64,
    protocol: String,
    protocolversion: u32,
    proxy: String,
    testnet: bool,
    version: String,
    // some properties are omitted due to limited scope of the work
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListAddressesResult {
    pub address: String,
    pub ismine: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateAddress {
    pub account: String,
    pub address: String,
    pub iscompressed: bool,
    pub ismine: bool,
    pub isvalid: bool,
    pub pubkey: String,
    pub synchronized: bool,
    // some properties are omitted due to limited scope of the work
}

const RPC_USERNAME: &str = dotenv!("RPC_USERNAME");
const RPC_PASSWORD: &str = dotenv!("RPC_PASSWORD");
const RPC_DOMAIN_AND_PORT: &str = dotenv!("RPC_DOMAIN_AND_PORT");

/// Make an RPC request to a given method, parse and return the JSON response
///
/// # Parameters
/// * `method`: A string slice representing the RPC method of the API to which the request should be made.
/// * `params`: A string representing the request payload to be included in the request.
///
/// # Returns
/// The deserialized response from the API as the specified type `T`.
///
/// # Example
/// ```
/// let response: MyResponseType = make_rpc_request("my_method", "some_value").await;
/// ```
async fn make_rpc_call<T>(method: &str, params: Vec<&str>) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a> + Debug,
{
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
    let auth_header =
        HeaderValue::from_str(&format!("Basic {}", encoded_auth_header_value)).unwrap();

    let response = client
        .post(RPC_DOMAIN_AND_PORT)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .json(&rpc_request)
        .send()
        .await
        .map_err(ApiError::RequestError)?;

    let api_result: serde_json::Value = response.json().await.map_err(ApiError::RequestError)?;

    if let Some(error) = api_result.get("error").as_ref().filter(|&x| !x.is_null()) {
        let err_msg = error["message"].as_str().unwrap_or("Unknown API error");
        return Err(ApiError::RPCError(err_msg.to_owned()));
    }

    let result_deserialized =
        serde_json::from_value::<T>(api_result["result"].clone()).map_err(ApiError::SerdeError);
    match result_deserialized {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Could not deserialize API response. Endpoint: {method}, Params: {params:?}");
            Err(err)
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result: ApiResult = make_rpc_call(RpcMethods::GETINFO, vec![]).await?;

    println!("=========== getinfo ==========");
    println!("{:?}", result);

    let result: ApiResult = make_rpc_call(RpcMethods::LISTADDRESSES, vec![]).await?;

    println!("\n=========== listaddresses ==========");
    println!("{:?}", result);

    let result: ApiResult = make_rpc_call(
        RpcMethods::VALIDATEADDRESS,
        vec!["1YXWvSKFm4XG5yPiFcXGGSvEwH4D8K2nRcXWeA"],
    )
    .await?;

    println!("\n=========== validateaddress ==========");
    println!("{:?}", result);

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
