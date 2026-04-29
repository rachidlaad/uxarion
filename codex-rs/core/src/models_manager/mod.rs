pub mod cache;
pub mod collaboration_mode_presets;
pub mod manager;
pub mod model_info;
pub mod model_presets;

use codex_protocol::openai_models::ClientVersion;

use crate::model_provider_info::OPENAI_PROVIDER_ID;

pub const OPENAI_COMPATIBILITY_CLIENT_VERSION: ClientVersion = ClientVersion(0, 98, 0);

/// Convert the client version string to a whole version string (e.g. "1.2.3-alpha.4" -> "1.2.3").
pub fn client_version_to_whole() -> String {
    client_version_to_whole_for_version(product_client_version())
}

pub fn product_client_version() -> ClientVersion {
    ClientVersion(
        parse_version_component(env!("CARGO_PKG_VERSION_MAJOR")),
        parse_version_component(env!("CARGO_PKG_VERSION_MINOR")),
        parse_version_component(env!("CARGO_PKG_VERSION_PATCH")),
    )
}

pub fn compatibility_client_version_for_provider(provider_id: Option<&str>) -> ClientVersion {
    if provider_id == Some(OPENAI_PROVIDER_ID) {
        OPENAI_COMPATIBILITY_CLIENT_VERSION
    } else {
        product_client_version()
    }
}

pub fn client_version_to_whole_for_provider(provider_id: Option<&str>) -> String {
    client_version_to_whole_for_version(compatibility_client_version_for_provider(provider_id))
}

fn client_version_to_whole_for_version(version: ClientVersion) -> String {
    let ClientVersion(major, minor, patch) = version;
    format!("{major}.{minor}.{patch}")
}

fn parse_version_component(value: &str) -> i32 {
    value.parse().unwrap_or_default()
}
