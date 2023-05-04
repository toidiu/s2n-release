mod error;
mod github;

use std::time::Duration;

use crate::error::ReleaseError;
use crate::error::Result;
use crate::github::Semver;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("GH_TOKEN").map_err(|_| ReleaseError::MissingAuthToken)?;

    let octo = octocrab::OctocrabBuilder::new()
        // token sys-octo-test
        .personal_token(token.to_string())
        .build()
        .map_err(|_| ReleaseError::GitHubAPI)?;

    let repo = octo.repos("toidiu", "s2n-release");

    let semver = Semver::new(0, 3, 0);
    github::release_create(&repo, semver).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;
    let _latest = github::release_latest(&repo).await?;

    Ok(())
}
// change push
// change push 2
// change push 3
// change push 4
// change push 5
// change push 6
