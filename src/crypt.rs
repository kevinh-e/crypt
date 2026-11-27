use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

// TODO:
// implement zeroize on vault
// remove debug

#[derive(Serialize, Deserialize, Debug)]
pub struct Crypt {
    entries: BTreeMap<String, Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    value: String,
    notes: String,
}

impl Crypt {
    /// doesn't need ownership, buffer zeroizes on drop
    pub fn from_bytes(s: &[u8]) -> Result<Crypt> {
        let c: Crypt = serde_json::from_slice(&s)?;
        Ok(c)
    }
}
