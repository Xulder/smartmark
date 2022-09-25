use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_search: String,
    shortcuts: Vec<Shortcut>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortcut {
    keyword: String,
    url: String,
}

// Todo: Set up config de/serialization
// Todo: Implement url validation, query substitution (like the %s in adding a custom browser)
// Todo: Implement url crate
// Todo: Add unit tests for all of this