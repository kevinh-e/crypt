use std::collections::BTreeMap;

// TODO:
// implement zeroize on vault
// remove debug

#[derive(Debug)]
pub struct Crypt {
    entries: BTreeMap<String, Entry>,
}

#[derive(Debug)]
pub struct Entry {
    value: String,
    notes: String,
}
