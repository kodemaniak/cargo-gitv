use std::time::SystemTime;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use git2::Repository;
use semver::Version;

fn main() -> Result<()> {
    let repo = Repository::open("./")
      .context("Could not find a git repository. Please run from the top-level folder of a git repository.")?;

    let mut tags: Vec<Version> = repo
        .tag_names(Some("v*"))?
        .iter()
        .flatten()
        .flat_map(|v| v.strip_prefix('v'))
        .flat_map(semver::Version::parse)
        .collect();
    tags.sort();
    tags.reverse();

    let current_release_version = tags.first().ok_or_else(|| {
        anyhow!("No current release version found! Needs at least one version tag.",)
    })?;
    println!("current release verion: {}", current_release_version);

    let timestamp: DateTime<Utc> = SystemTime::now().into();
    let timestamp_formatted = timestamp.format("%Y%m%d%H%M%S");
    println!("current timesatmp: {}", timestamp_formatted);

    let dev_version = format!("{}-{}", current_release_version, timestamp_formatted);
    println!("{}", dev_version);

    Ok(())
}
