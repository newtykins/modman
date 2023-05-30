use clap::{Parser, Subcommand};
use modman::Error;

pub mod create;
pub mod select;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Create a new profile
    Create,

    /// Select a profile
    Select,
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Create => create::execute()?,
        Subcommands::Select => select::execute()?,
    }

    Ok(())
}
