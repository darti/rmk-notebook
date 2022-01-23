use std::path::PathBuf;

mod errors;
mod notebook;

pub use errors::*;

pub fn read_notebook(root: &PathBuf, id: &str) -> Result<()> {
    Ok(())
}
