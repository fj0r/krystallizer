use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Provider {
    pub name: String,
    pub baseurl: String,
    pub api_key: String,
    pub default_model: String,
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

fn default_temp() -> f32 {
    0.5
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub provider: String,
    pub adapter: Adapter,
    pub name: String,
    #[serde(default = "default_bool_true")]
    pub has_fn: bool,
    #[serde(default = "default_bool_false")]
    pub has_search: bool,
    #[serde(default = "default_bool_false")]
    pub has_thinking: bool,
    #[serde(default = "default_temp")]
    pub temp: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: IndexMap<String, Model>,
    pub provider: IndexMap<String, Provider>,
}

impl Config {
    pub fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("agent.toml"))
            .merge(Env::prefixed("AGENT_").split("_"))
            .extract()
    }
}
