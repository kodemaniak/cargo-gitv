use anyhow::Result;
use semver::{Version, VersionReq};

use crate::build_context::BuildContext;
use anyhow::anyhow;

pub fn verify(build_context: &BuildContext) -> Result<()> {
    let workspace_version = build_context.dev_version()?;
    let base_version = Version::new(
        workspace_version.major,
        workspace_version.minor,
        workspace_version.patch,
    );
    let req = VersionReq::parse(&build_context.cargo_version.to_string()).unwrap();

    if req.matches(&base_version) {
        Ok(())
    } else {
        Err(anyhow!(
            "Cargo version ${} does not match computed version {}!",
            build_context.cargo_version,
            build_context.dev_version()?
        ))
    }
}
