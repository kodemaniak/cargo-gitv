use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// WHen we are called from cargo, the first argument is the subcommand. We just parse this for compatibility.
    #[clap(value_parser)]
    bin: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub fn called_from_cargo(&self) -> bool {
        match &self.bin {
            Some(bin) => bin == "gitv",
            _ => false,
        }
    }

    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// does testing things
    Version,
}
