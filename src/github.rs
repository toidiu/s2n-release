use crate::error::*;
use octocrab::{models::repos::Release, repos::RepoHandler};
use std::str::FromStr;

pub async fn release_list(repo: &RepoHandler<'_>) -> Result<Vec<Release>> {
    let release = repo
        .releases()
        .list()
        .send()
        .await
        .map_err(|_| ReleaseError::GitHubAPI)?
        .items;

    for r in &release {
        println!(
            "the release name: {} id {}, prerelease {}",
            r.tag_name, r.id, r.prerelease
        );
    }
    Ok(release)
}

pub async fn release_latest(repo: &RepoHandler<'_>) -> Result<Option<Semver>> {
    let r = repo.releases().get_latest().await;
    if r.is_err() {
        return Ok(None);
    }
    let r = r.expect("handled error above");

    println!(
        "latest release name: {} id {}, prerelease {}",
        r.tag_name, r.id, r.prerelease
    );
    let semver = Semver::from_str(&r.tag_name);
    semver.map(Some)
}

pub async fn release_create(repo: &RepoHandler<'_>, semver: Semver) -> Result<()> {
    repo.releases()
        .create(&semver.to_string())
        .target_commitish("main")
        .name(&semver.to_string())
        .body("body")
        .prerelease(false)
        .draft(false)
        .send()
        .await
        .map_err(|_| ReleaseError::GitHubAPI)?;

    Ok(())
}

pub struct Semver {
    patch: u32,
    minor: u32,
    major: u32,
}

impl Semver {
    pub fn new(patch: u32, minor: u32, major: u32) -> Self {
        Semver {
            patch,
            minor,
            major,
        }
    }
}

impl ToString for Semver {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Semver {
    type Err = ReleaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let versions: Vec<&str> = s
            .strip_prefix('v')
            .ok_or(ReleaseError::SemverParse)?
            .split('.')
            // .map(|x| x.parse::<u32>().unwrap_or(ReleaseError::SemverParse))
            .collect();

        assert!(versions.len() == 3);

        #[allow(clippy::get_first)]
        let major: u32 = versions
            .get(0)
            .ok_or(ReleaseError::SemverParse)?
            .parse::<u32>()
            .map_err(|_| ReleaseError::SemverParse)?;
        let minor: u32 = versions
            .get(1)
            .ok_or(ReleaseError::SemverParse)?
            .parse::<u32>()
            .map_err(|_| ReleaseError::SemverParse)?;
        let patch: u32 = versions
            .get(2)
            .ok_or(ReleaseError::SemverParse)?
            .parse::<u32>()
            .map_err(|_| ReleaseError::SemverParse)?;

        Ok(Semver::new(major, minor, patch))
    }
}
