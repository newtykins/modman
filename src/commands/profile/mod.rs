use clap::{Parser, Subcommand};
use modman::Error;

mod backup;
mod create;
mod select;
mod view;

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

    /// Backup your profiles using git
    Backup(backup::Command),

    /// View a profile
    #[clap(alias = "current")]
    View(view::Args),
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Create => create::execute()?,
        Subcommands::Select => select::execute()?,
        Subcommands::View(args) => view::execute(args)?,
        Subcommands::Backup(subcommand) => backup::parse(subcommand)?,
    }

    Ok(())
}
