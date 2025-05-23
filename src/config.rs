use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Assistant {
    pub name: String,
    pub help: String,
    pub model: String,
    pub max_tokens: u32,
    pub system_message: String,
    pub prefix: String,
}

impl Default for Assistant {
    fn default() -> Self {
        Assistant {
            name: "chat".to_string(),
            help: "Ask a generic question".to_string(),
            model: "gpt-4.1-nano".to_string(),
            max_tokens: 100,
            system_message: "You are a helpful assistant".to_string(),
            prefix: String::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub debug_mode: bool,
    pub assistants: Vec<Assistant>,
}

impl Config {
    pub fn load() -> Result<Self, confy::ConfyError> {
        let name = clap::crate_name!();
        let config_name = "config";
        let config: Self = confy::load(name, config_name)?;
        confy::store(name, config_name, &config)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            openai_api_key: None,
            debug_mode: false,
            assistants: vec![
                Assistant::default(),
                Assistant {
                    name: "howto".to_string(),
                    help: "Get bash command that solves the problem".to_string(),
                    model: "gpt-4.1-nano".to_string(),
                    max_tokens: 100,
                    system_message: "Answer only with linux bash command no markdown".to_string(),
                    prefix: "How to".to_string(),
                },
            ],
        }
    }
}
