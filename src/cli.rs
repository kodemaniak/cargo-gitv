use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitv", bin_name = "gitv")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// WHen we are called from cargo, the first argument is the subcommand. We just parse this for compatibility.
    #[arg(hide = true, value_parser = clap::builder::PossibleValuesParser::new(["gitv"]))]
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
    /// Verify that the version computed from git is consistent with the Cargo version.
    Verify,
    /// Compute the current version based on git and Cargo metadata.
    Version,
}
