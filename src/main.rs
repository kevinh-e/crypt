mod crypt;
use crypt::Crypt;

use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use dirs::data_local_dir;
use zeroize::Zeroizing;

const DEFAULT_DATA_LOCATION: &str = "crypt";
const DEFAULT_SAVE_NAME: &str = "save";

fn main() -> Result<()> {
    let save_path: PathBuf = data_local_dir()
        .context("Could not find data directory")?
        .join(DEFAULT_DATA_LOCATION)
        .join(DEFAULT_SAVE_NAME);
    print!("{:?}", save_path);

    init_save(save_path).context("Failed to load saved Crypt")?;

    Ok(())
}

// make the crypt directory if first time using
// read the save file, create if not there
// return the object
fn init_save(save_path: impl AsRef<Path>) -> Result<Crypt> {
    fs::create_dir_all(DEFAULT_DATA_LOCATION)?;

    // try a temp file from previous incomplete save
    let temp_savepath: PathBuf = save_path.as_ref().with_extension("tmp");
    let mut file = load_file(temp_savepath)?;

    // create buffer with same size as file, since zeroize isn't complete when Vec grows
    let mut buf = Zeroizing::new(Vec::with_capacity(file.metadata()?.len().try_into()?));
    file.read_to_end(&mut buf)?;

    let c = Crypt::from_bytes(&buf);

    // actual save
    let file = load_file(save_path)?;
    todo!()
}

fn load_file(filepath: impl AsRef<Path>) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .open(&filepath)
        .with_context(|| format!("Failed to read file {}", filepath.as_ref().display()))
}

// todo list:
// create vault objects (in memory)
// create a persistent file if it desn't exist (start with plaintext)
// open persistent file
// loading and saving from that file into memory as vault objects
// cli args for crud
// encryption
// ui
// touchid
