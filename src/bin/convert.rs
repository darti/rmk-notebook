use std::path::PathBuf;

use rmk_notebook::read_notebook;

use rmk_notebook::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let root = PathBuf::from("samples");
    let id = "0d9af7de-39f8-4251-8500-330eec0d00f0";

    read_notebook(&root, id)?;

    Ok(())
}
