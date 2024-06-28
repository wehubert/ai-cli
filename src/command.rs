use crate::config::{Config, Item};
use clap::{command, error::ErrorKind, Arg, Command, Error};

pub fn parse_command(config: &Config) -> Result<(String, Item), Error> {
    let subcommands: Vec<Command> = config
        .items
        .iter()
        .map(|item| -> Command {
            Command::new(&item.name).about(&item.help).arg(
                Arg::new("message")
                    .help("Message content")
                    .num_args(0..)
                    .required(true),
            )
        })
        .collect();

    let mut args = command!()
        .subcommand_required(true)
        .subcommands(subcommands)
        .get_matches();

    log::debug!("called with params: {:?}", &args);

    let (subcmd_name, mut subcmd_args) = args
        .remove_subcommand()
        .ok_or(Error::new(ErrorKind::InvalidSubcommand))?;

    let subcmd_config = config
        .items
        .iter()
        .find(|item| item.name == subcmd_name)
        .cloned()
        .ok_or(Error::new(ErrorKind::InvalidSubcommand))?;

    let message = format!(
        "{} {}",
        subcmd_config.prefix,
        subcmd_args
            .remove_many::<String>("message")
            .unwrap_or_default()
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok((message, subcmd_config))
}
