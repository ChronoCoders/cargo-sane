//! Version comparison utilities

use semver::Version;

pub fn is_major_update(current: &Version, latest: &Version) -> bool {
    latest.major > current.major
}

pub fn is_minor_update(current: &Version, latest: &Version) -> bool {
    latest.major == current.major && latest.minor > current.minor
}

pub fn is_patch_update(current: &Version, latest: &Version) -> bool {
    latest.major == current.major && latest.minor == current.minor && latest.patch > current.patch
}
