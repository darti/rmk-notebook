use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    time::SystemTime,
};

use log::info;
use serde::Deserialize;
use serde_with::serde_as;

use crate::{rm::LinesData, Error, Result};

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub file_type: Option<String>,
    pub page_count: usize,
    pub pages: Vec<String>,
    pub orientation: String,
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
    let metadata = serde_json::from_str(&file)?;
    Ok(metadata)
}

pub fn read_content(root: &PathBuf, id: &str) -> Result<Content> {
    let file = std::fs::read_to_string(root.join(format!("{}.content", id)))?;
    let content = serde_json::from_str(&file)?;
    Ok(content)
}

pub fn read_pagedata(root: &PathBuf, id: &str) -> Result<Vec<String>> {
    let file = File::open(root.join(format!("{}.pagedata", id)))?;
    let pagedata = BufReader::new(file)
        .lines()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(pagedata)
}

pub fn read_rm(root: &PathBuf, id: &str, pages: &Vec<String>) -> Result<()> {
    let path = root.join(id).join("*.rm");

    for page in pages.iter() {
        let rm_file = root.join(id).join(format!("{}.rm", page));

        info!("{}", rm_file.display());
        let mut file = File::open(rm_file.clone())?;
        let lines_data = LinesData::parse(&mut file).unwrap();

        // lines_are_rusty::render_pdf(output, &lines_data.pages).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Result;
    use std::path::PathBuf;

    #[test]
    fn metadata() -> Result<()> {
        let root = PathBuf::from("samples");
        let id = "0d9af7de-39f8-4251-8500-330eec0d00f0";

        let metadata = super::read_metadata(&root, id)?;

        Ok(())
    }
}
