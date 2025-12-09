use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Translation {
    val: String,
    remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    en: Translation,
    zh: Translation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Catalog {
    prompt: Entry,
    entries: IndexMap<String, Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub catalog: IndexMap<String, Catalog>,
}

impl Config {
    pub fn new() -> Result<Self> {
        Figment::new().merge(Toml::file("catalog.toml")).extract();
    }
}
