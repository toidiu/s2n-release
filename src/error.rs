// pub type Error = Box<dyn 'static + std::error::Error + Send + Sync>;
pub type Result<T, E = ReleaseError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ReleaseError {
    #[error("missing github auth token")]
    MissingAuthToken,
    #[error("github api failure")]
    GitHubAPI,
    #[error("failed to parse SemVer string. Expected format v1.2.3")]
    SemverParse,
}
