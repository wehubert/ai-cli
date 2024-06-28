use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Item {
    pub name: String,
    pub help: String,
    pub model: String,
    pub max_tokens: u32,
    pub system_message: String,
    pub prefix: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub debug_mode: bool,
    pub items: Vec<Item>,
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    let name = clap::crate_name!();
    let config_name = "config";

    let config: Config = confy::load(name, config_name)?;
    confy::store(name, config_name, &config)?;
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            openai_api_key: Some(String::default()),
            debug_mode: false,
            items: vec![
                Item {
                    name: "howto".to_string(),
                    help: "Get bash command that solves the problem".to_string(),
                    model: "gpt-3.5-turbo".to_string(),
                    max_tokens: 100,
                    system_message: "Answer with only one command for linux bash shell".to_string(),
                    prefix: "Howto".to_string(),
                },
                Item {
                    name: "chat".to_string(),
                    help: "Ask a generic question".to_string(),
                    model: "gpt-3.5-turbo".to_string(),
                    max_tokens: 100,
                    system_message: "You are a helpful assistant".to_string(),
                    prefix: String::default(),
                },
            ],
        }
    }
}
