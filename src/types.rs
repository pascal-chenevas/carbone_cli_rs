use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GenerateReportResult {
    created: bool,
    output: String,
    bytes: u64,
    error: Option<String>
}

impl GenerateReportResult {

    pub fn new(created: bool, output: String, bytes: u64, error: Option<String>) -> Self {
        Self {
            created,
            output,
            bytes,
            error
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UploadResult {
    output: String,
    uploaded: bool,
    template_id: Option<String>,
    error: Option<String>
}

impl UploadResult {

    pub fn new(output: String, uploaded: bool, template_id: Option<String>, error: Option<String>) -> Self {
        Self {
            output,
            uploaded,
            template_id,
            error
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadResult {
    output: String,
    downloaded: bool,
    bytes: u64,
    error: Option<String>
}

impl DownloadResult {

    pub fn new(output: String, downloaded: bool, bytes: u64, error: Option<String>) -> Self {
        Self {
            output,
            downloaded,
            bytes,
            error
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResult {
    deleted: bool,
    template_id: Option<String>,
    error: Option<String>
}

impl DeleteResult {

    pub fn new(deleted: bool, template_id: Option<String>, error: Option<String>) -> Self {
        Self {
            deleted,
            template_id,
            error
        }
    }
}