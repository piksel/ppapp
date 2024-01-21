
use semver::{BuildMetadata, Prerelease, Version};
// static version: Version =

use git_version::git_version;
use lazy_static::lazy_static;

pub(crate) const GIT_VERSION: &str = git_version!(args = ["--always"]);

lazy_static!{
    pub static ref VERSION: Version = Version::parse(GIT_VERSION)
    .unwrap_or_else(|_| Version{
        major: 0,
        minor: 0,
        patch: 0,
        pre: Prerelease::new(GIT_VERSION).unwrap_or_default(),
        build: BuildMetadata::EMPTY,
    });
}

pub fn app_version() -> String {
    return format!("v{}.{}.{}-{}", VERSION.major, VERSION.minor, VERSION.patch, VERSION.pre);
}