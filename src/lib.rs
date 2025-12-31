use async_openai::{config::OpenAIConfig, Client};
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PpqBlinkError {
    #[error("Blink API error: {0}")]
    Api(String),
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Serialize)]
struct GraphqlRequest {
    query: String,
    variables: Value,
}

#[derive(Deserialize)]
struct GraphqlResponse<T> {
    data: Option<T>,
    errors: Option<Vec<Value>>,
}

#[derive(Deserialize)]
struct LnInvoicePaymentSend {
    #[serde(rename = "lnInvoicePaymentSend")]
    payment: PaymentSendPayload,
}

#[derive(Deserialize)]
struct PaymentSendPayload {
    status: String,
    errors: Option<Vec<Value>>,
}

pub struct PpqBlinkClient {
    pub ppq_client: Client<OpenAIConfig>,
    api_key: String,
    wallet_id: String,
    http_client: reqwest::Client,
}

impl PpqBlinkClient {
    pub fn new(ppq_api_key: String, blink_api_key: String, wallet_id: String) -> Self {
        let ppq_config = OpenAIConfig::default()
            .with_api_base("https://api.ppq.ai/v1")
            .with_api_key(ppq_api_key);
        let ppq_client = Client::with_config(ppq_config);

        Self {
            ppq_client,
            api_key: blink_api_key,
            wallet_id,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn pay_lightning_invoice(&self, bolt11: &str) -> Result<String, PpqBlinkError> {
        let query = r#"
            mutation LnInvoicePaymentSend($input: LnInvoicePaymentInput!) {
                lnInvoicePaymentSend(input: $input) {
                    status
                    errors {
                        message
                    }
                }
            }
        "#.to_string();

        let variables = serde_json::json!({
            "input": {
                "walletId": self.wallet_id,
                "paymentRequest": bolt11.trim()
            }
        });

        let request_body = GraphqlRequest { query, variables };

        let response = self
            .http_client
            .post("https://api.blink.sv/graphql")
            .header(header::CONTENT_TYPE, "application/json")
            .header("X-API-KEY", &self.api_key)
            .json(&request_body)
            .send()
            .await?;

        let resp: GraphqlResponse<LnInvoicePaymentSend> = response.json().await?;

        // Handle top-level GraphQL errors
        if let Some(errors) = resp.errors {
            let error_msg = serde_json::to_string_pretty(&errors)
                .unwrap_or_else(|_| "Unknown GraphQL error".to_string());
            return Err(PpqBlinkError::Api(error_msg));
        }

        let payload = resp
            .data
            .ok_or_else(|| PpqBlinkError::Api("No data in response".to_string()))?
            .payment;

        // If there are business-level errors, report them
        if let Some(errors) = payload.errors {
            if !errors.is_empty() {
                let error_msg = serde_json::to_string_pretty(&errors)
                    .unwrap_or_else(|_| "Payment failed with errors".to_string());
                return Err(PpqBlinkError::Api(error_msg));
            }
        }

        // Success: return the status (usually "SUCCESS")
        Ok(payload.status)
    }

    pub fn ppq_client(&self) -> &Client<OpenAIConfig> {
        &self.ppq_client
    }
}
