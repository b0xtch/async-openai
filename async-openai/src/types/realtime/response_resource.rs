use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailedError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteReason {
    Interruption,
    MaxOutputTokens,
    ContentFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ResponseStatusDetail {
    #[serde(rename = "incomplete")]
    Incomplete { reason: IncompleteReason },
    #[serde(rename = "failed")]
    Failed { error: Option<FailedError> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseResource {
    /// The unique ID of the response.
    pub id: String,
    /// The object type, must be "realtime.response".
    pub object: String,
    /// The status of the response
    pub status: ResponseStatus,
    /// Additional details about the status.
    pub status_details: Option<ResponseStatusDetail>,
    /// The list of output items generated by the response.
    pub output: Vec<Item>,
    /// Usage statistics for the response.
    pub usage: Option<Usage>,
}
