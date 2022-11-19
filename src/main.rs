use anyhow::Result;
use cargo_gitv::{
    build_context::load_build_context,
    cli::{Cli, Command},
    verify::verify,
    version::version,
};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let build_context = load_build_context()?;

    match cli.command() {
        Command::Version => version(&build_context),
        Command::Verify => verify(&build_context),
    }
}
