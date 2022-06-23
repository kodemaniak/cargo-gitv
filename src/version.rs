use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::build_context::BuildContext;

pub fn version(build_context: &BuildContext) -> Result<()> {
    let timestamp: DateTime<Utc> = build_context
        .git_commit_timestamp
        .unwrap_or_else(|| SystemTime::now().into());
    let timestamp_formatted = timestamp.format("%Y%m%d%H%M%S");

    let dev_version = format!(
        "{}+{}.{}",
        build_context.cargo_version, timestamp_formatted, build_context.git_sha
    );
    println!("{}", dev_version);

    Ok(())
}
