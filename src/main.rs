mod error;
mod github;

use crate::error::ReleaseError;
use crate::error::Result;
use crate::github::Semver;

#[tokio::main]
async fn main() -> Result<()> {
    let octo = octocrab::OctocrabBuilder::new()
         // token sys-octo-test 
        .personal_token("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".to_string())
    .build()
        .map_err(|_| ReleaseError::GitHubAPI)?;

    let repo = octo.repos("toidiu", "s2n-release");

    let _latest = github::release_latest(&repo).await?;

    let semver = Semver::new(0, 2, 0);
    // github::release_create(&repo, semver).await?;

    Ok(())
}
