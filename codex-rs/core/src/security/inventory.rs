use super::SECURITY_BINARY_ALLOWLIST;
use super::zap;
use crate::config::types::SecurityZapConfig;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SecurityToolInventory {
    pub available: Vec<String>,
    pub missing: Vec<String>,
    pub zap_enabled: bool,
    pub zap_api_base_url: String,
    pub zap_api_key_configured: bool,
    pub zap_api_reachable: bool,
    pub zap_version: Option<String>,
    pub zap_error: Option<String>,
}

impl SecurityToolInventory {
    pub(crate) fn disabled(zap_config: &SecurityZapConfig) -> Self {
        Self {
            available: Vec::new(),
            missing: Vec::new(),
            zap_enabled: zap_config.enabled,
            zap_api_base_url: zap_config.base_url.clone(),
            zap_api_key_configured: zap_config.api_key.is_some(),
            zap_api_reachable: false,
            zap_version: None,
            zap_error: None,
        }
    }

    pub(crate) async fn discover(zap_config: &SecurityZapConfig) -> Self {
        let mut available = Vec::new();
        let mut missing = Vec::new();
        for binary in SECURITY_BINARY_ALLOWLIST {
            if which::which(binary).is_ok() {
                available.push((*binary).to_string());
            } else {
                missing.push((*binary).to_string());
            }
        }
        available.sort();
        missing.sort();

        let zap_status = zap::probe_zap_api(zap_config).await;

        Self {
            available,
            missing,
            zap_enabled: zap_status.enabled,
            zap_api_base_url: zap_status.base_url,
            zap_api_key_configured: zap_status.api_key_configured,
            zap_api_reachable: zap_status.reachable,
            zap_version: zap_status.version,
            zap_error: zap_status.error,
        }
    }
}
