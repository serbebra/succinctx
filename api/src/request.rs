use alloy_primitives::{Address, Bytes, FixedBytes};
use anyhow::{Error, Result};
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Data to be sent to the Succinct X API with an offchain request.
struct OffchainInput {
    /// The chain id of the network to be used.
    chainId: u32,
    /// The address of the contract to call.
    to: Address,
    /// The calldata to be used in the contract call.
    data: Bytes,
    /// The Succinct X function id to be called.
    functionId: FixedBytes<32>,
    /// The input to be used in the Succinct X function call.
    input: Bytes,
}

#[derive(Serialize, Deserialize)]
/// Data received from the Succinct X API from an offchain request.
struct OffchainRequestResponse {
    request_id: String,
}

/// Client to interact with the Succinct X API.
pub struct SuccinctClient {
    client: Client,
    /// The base url for the Succinct X API. (ex. https://alpha.succinct.xyz/api)
    base_url: String,
}

impl SuccinctClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Submit an offchain request to the Succinct X API.
    pub async fn submit_platform_request(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: FixedBytes<32>,
        input: Bytes,
    ) -> Result<String> {
        let data = OffchainInput {
            chainId: chain_id,
            to,
            data: calldata,
            functionId: function_id,
            input,
        };

        // Serialize the data to JSON.
        let serialized_data = serde_json::to_string(&data).unwrap();

        // Make off-chain request.
        let request_url = format!("{}{}", self.base_url, "/request/new");
        let res = self
            .client
            .post(request_url)
            .header("Content-Type", "application/json")
            .body(serialized_data)
            .send()
            .await
            .unwrap();

        // Check if the request was successful.
        if res.status().is_success() {
            info!("Request successful!");
            let response: OffchainRequestResponse = res.json().await.unwrap();
            Ok(response.request_id)
        } else {
            error!("Request failed!");
            Err(Error::msg("Failed to submit request to Succinct X API."))
        }
    }
}
