use std::path::PathBuf;

mod errors;
mod notebook;
mod parse;
mod rm;

pub use errors::*;
use notebook::{read_content, read_metadata, read_pagedata, read_rm};

pub fn read_notebook(root: &PathBuf, id: &str) -> Result<()> {
    let metadata = read_metadata(root, id)?;
    let content = read_content(root, id)?;
    let pagedata = read_pagedata(root, id)?;
    let notebook = read_rm(root, id, &content.pages)?;

    Ok(())
}
