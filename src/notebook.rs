use std::{path::PathBuf, time::SystemTime};

use serde::Deserialize;
use serde_with::serde_as;

use crate::Result;

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub file_type: Option<String>,
    // pub page_count: usize,
    // pub pages: Vec<String>,
    // pub orientation: String,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub _deleted: bool,
    // #[serde_as(as = "serde_with::TimestampSeconds<String>")]
    // last_opened: SystemTime,
    // last_opened_page: usize,
    #[serde_as(as = "serde_with::TimestampSeconds<String>")]
    pub last_modified: SystemTime,
    pub _metadatamodified: bool,
    pub _modified: bool,

    pub parent: String,
    pub _pinned: bool,
    pub _synced: bool,
    #[serde(rename = "type")]
    pub typ: String,
    pub _version: usize,
    pub visible_name: String,
}

pub fn read_metadata(root: &PathBuf, id: &str) -> Result<Metadata> {
    let file = std::fs::read_to_string(root.join(format!("{}.metadata", id)))?;
    let metadata: Metadata = serde_json::from_str(&file)?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn metadata() {
        let root = PathBuf::from("samples");
        let id = "0d9af7de-39f8-4251-8500-330eec0d00f0";
    }
}
