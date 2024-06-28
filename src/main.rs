mod command;
mod config;
mod openai;

use crate::command::parse_command;
use crate::config::load_config;
use crate::openai::chat_openai;

fn get_log_level(debug_mode: bool) -> log::Level {
    if debug_mode {
        log::Level::Debug
    } else {
        log::Level::Info
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    simple_logger::init_with_level(get_log_level(config.debug_mode))?;

    log::debug!("Config read: {:?}", &config);

    let (message, config_item) = parse_command(&config)?;
    let response = chat_openai(config.openai_api_key, message, config_item).await?;
    println!("{}", response);

    Ok(())
}
