mod crypt;
use crypt::Crypt;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use dirs::data_local_dir;

const DEFAULT_SAVE_LOCATION: &str = "crypt/save";

fn main() -> Result<()> {
    let save_path: PathBuf = data_local_dir()
        .context("Could not find data directory")?
        .join(DEFAULT_SAVE_LOCATION);
    print!("{:?}", save_path);
    init_save(save_path).context("Failed to load saved Crypt")?;

    Ok(())
}

fn init_save(save_path: impl AsRef<Path>) -> Result<Crypt> {
    todo!()
}

// todo list:
// create vault objects (in memory)
// create a persistent file if it desn't exist
// open persistent file
// loading and saving from that file into memory as vault objects
// cli args for crud
