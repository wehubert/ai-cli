use crate::config::Config;
use clap::{command, error::ErrorKind, Arg, Error};

#[derive(Debug, Clone, Default)]
pub struct Command {
    pub name: String,
    pub message: String,
}

impl Command {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let subcommands: Vec<clap::Command> = config
            .assistants
            .iter()
            .map(|assitant| -> clap::Command {
                clap::Command::new(&assitant.name)
                    .about(&assitant.help)
                    .arg(
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

        let (subcmd_name, mut subcmd_args) = args
            .remove_subcommand()
            .ok_or(Error::new(ErrorKind::InvalidSubcommand))?;

        let sumbcmd_message = subcmd_args
            .remove_many::<String>("message")
            .unwrap_or_default()
            .collect::<Vec<_>>()
            .join(" ");

        Ok(Command {
            name: subcmd_name,
            message: sumbcmd_message,
        })
    }
}
