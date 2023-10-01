use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Created(bool),
    Deleted(bool),
    Uploaded(bool),
    Downloaded(bool),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonResult {
    pub state: State,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub bytes: Option<u64>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}
