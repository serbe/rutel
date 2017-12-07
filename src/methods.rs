#[allow(non_camel_case_types)]

use types::*;
use serde_json::to_string;

// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
#[derive(Serialize, Debug)]
pub struct getUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<String>>,
}

impl getUpdatesParams {
    pub fn new() -> Self {
        getUpdatesParams {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn offset(&mut self, v: u64) {
        self.offset = Some(v);
    }

    pub fn limit(&mut self, v: u64) {
        self.limit = Some(v);
    }

    pub fn timeout(&mut self, v: u64) {
        self.timeout = Some(v);
    }

    pub fn allowed_updates(&mut self, v: Vec<String>) {
        self.allowed_updates = Some(v);
    }

    pub fn json(&self) -> String {
        to_string(self).unwrap()
    }
}