use serde::{Deserialize, Serialize};

use crate::types::*;

// All this will be generated with protocol.pest and templates.rs

#[derive(Debug, Serialize)]
pub struct HeaderRequest {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: NullableString,
}

#[derive(Debug, Deserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}

#[derive(Debug, Deserialize)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub api_versions: Vec<ApiVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ApiVersion {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}
