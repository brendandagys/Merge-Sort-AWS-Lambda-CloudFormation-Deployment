use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    INFO,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpInformation {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: String,
    pub source_ip: String,
    pub user_agent: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayRequestContext {
    pub account_id: String,
    pub api_id: String,
    pub domain_name: String,
    pub domain_prefix: String,
    pub http: HttpInformation,
    pub request_id: String,
    pub route_key: String,
    pub stage: String,
    pub time: String,
    pub time_epoch: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ParsedRequestBody {
    pub numbers: Vec<i32>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayEvent {
    pub body: String,
    pub version: String,
    pub route_key: String,
    pub raw_path: String,
    pub raw_query_string: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub request_context: ApiGatewayRequestContext,
    pub is_base_64_encoded: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutput {
    pub sorted_numbers: Vec<i32>,
}
