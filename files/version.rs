//! Static build info for Tana Desktop.

pub const BUILD_VERSION: &str  = "v2.7.18";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "291b6c8e1d6a";
pub const BUILD_TIMESTAMP: u64 = 1721372513;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Tana Desktop", BUILD_VERSION, BUILD_CHANNEL)
}
