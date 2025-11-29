use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Provider {
    pub label: String,
    pub baseurl: String,
    pub api_key: String,
    pub default_model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProviderRef {
    #[allow(non_camel_case_types)]
    name(String),
    #[allow(non_camel_case_types)]
    provider(Provider),
}

impl Deref for ProviderRef {
    type Target = Provider;
    fn deref(&self) -> &Self::Target {
        if let ProviderRef::provider(x) = self {
            return &x;
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Adapter {
    OpenAI,
}

fn default_bool_true() -> bool {
    true
}

fn default_bool_false() -> bool {
    false
}

fn default_temperature() -> f32 {
    0.5
}

fn default_max_token() -> u32 {
    512
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capabilities {
    #[serde(default = "default_bool_true")]
    pub function: bool,
    #[serde(default = "default_bool_true")]
    pub json: bool,
    #[serde(default = "default_bool_false")]
    pub search: bool,
    #[serde(default = "default_bool_false")]
    pub thinking: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub provider: ProviderRef,
    pub adapter: Adapter,
    pub name: String,
    pub capabilities: Capabilities,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_token")]
    pub max_token: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurrealMigration {
    pub path: String,
}

fn const_localhost() -> String {
    "localhost".to_string()
}
fn const_default() -> String {
    "default".to_string()
}
fn const_public() -> String {
    "public".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurrealConfig {
    #[serde(default = "const_localhost")]
    pub host: String,
    pub port: String,
    #[serde(default = "const_default")]
    pub ns: String,
    #[serde(default = "const_public")]
    pub db: String,
    pub user: String,
    pub pass: String,
    pub migration: SurrealMigration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub surreal: SurrealConfig,
}

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
pub struct SessionWindow {
    size: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    window: SessionWindow,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: IndexMap<String, Model>,
    pub provider: IndexMap<String, Provider>,
    pub database: Database,
    pub catalog: IndexMap<String, Catalog>,
    pub session: Session,
}

impl Config {
    pub fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("agent.toml"))
            .merge(Env::prefixed("AGENT__").split("__"))
            .extract()
    }

    pub fn get_model(&self, name: impl AsRef<str>) -> Option<Model> {
        let m = self.model.get(name.as_ref())?;
        match &m.provider {
            ProviderRef::name(n) => Some(Model {
                provider: ProviderRef::provider(self.provider.get(n)?.clone()),
                ..m.clone()
            }),
            ProviderRef::provider(_) => Some(m.clone()),
        }
    }
}
