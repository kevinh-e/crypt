mod crypt;
use crypt::Crypt;

use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, Result};
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

    let c = init_save(save_path).context("Failed to load saved Crypt")?;

    print!("{:?}", c);

    Ok(())
}

// make the crypt directory if first time using
// read the save file, create if not there
// return the object
fn init_save(save_path: impl AsRef<Path>) -> Result<Crypt> {
    fs::create_dir_all(DEFAULT_DATA_LOCATION)?;

    if OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&save_path)
        .is_ok()
    {
        return Ok(Crypt::new());
    };

    // try a temp file from previous incomplete save, otherwise load normally
    let temp_savepath: PathBuf = save_path.as_ref().with_extension("tmp");
    let c = match load_file(temp_savepath) {
        Ok(c) => c,
        // assuming permissions are valid
        Err(_) => load_file(save_path)?,
    };

    Ok(c)
}

fn load_file(filepath: impl AsRef<Path>) -> Result<Crypt> {
    // the buf is already statically sized from read
    let buf = Zeroizing::new(fs::read(filepath)?);
    Crypt::from_bytes(&buf)
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
