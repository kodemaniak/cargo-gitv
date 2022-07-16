use anyhow::Result;

use crate::build_context::BuildContext;

pub fn version(build_context: &BuildContext) -> Result<()> {
    println!("{}", build_context.dev_version()?);

    Ok(())
}
