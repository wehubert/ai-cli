mod command;
mod config;
mod openai;

use command::Command;
use config::Config;
use openai::OpenAI;

fn get_log_level(debug_mode: bool) -> log::Level {
    if debug_mode {
        log::Level::Debug
    } else {
        log::Level::Info
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    simple_logger::init_with_level(get_log_level(config.debug_mode))?;

    log::debug!("Config read: {:?}", config);

    let command = Command::new(&config)?;
    let response = OpenAI::new(&config)
        .set_assistant(&command.name)
        .chat(&command.message)
        .await?;
    println!("{}", response);

    Ok(())
}
