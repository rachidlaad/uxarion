use super::ChatWidget;
use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionAction;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;
use crate::render::renderable::ColumnRenderable;
use codex_core::config::types::DEFAULT_SECURITY_ZAP_BASE_URL;
use codex_core::config::types::SecurityZapConfig;
use ratatui::style::Stylize;
use ratatui::text::Line;
use serde_json::Value;
use url::Url;

const ZAP_STATUS_TIMEOUT_SECS: u64 = 4;

impl ChatWidget {
    pub(crate) fn open_zap_popup(&mut self) {
        if !self.is_session_configured() {
            self.add_info_message(
                "ZAP setup is disabled until startup completes.".to_string(),
                None,
            );
            return;
        }

        let current_config = self.current_zap_config();
        let enable_label = if current_config.enabled {
            "Disable ZAP"
        } else {
            "Enable ZAP"
        };
        let enable_description = if current_config.enabled {
            "Turn off ZAP-backed tools for future sessions."
        } else {
            "Turn on ZAP-backed tools for future sessions."
        };
        let mut enable_config = current_config.clone();
        enable_config.enabled = !current_config.enabled;

        let items = vec![
            selection_item(
                "Show ZAP status".to_string(),
                "Check whether the configured ZAP API is reachable right now.".to_string(),
                current_config.clone(),
            ),
            persist_item(
                "Use localhost default".to_string(),
                format!(
                    "Save the default ZAP API URL for most machines: {DEFAULT_SECURITY_ZAP_BASE_URL}."
                ),
                SecurityZapConfig {
                    base_url: DEFAULT_SECURITY_ZAP_BASE_URL.to_string(),
                    ..current_config.clone()
                },
                "Saved ZAP base URL".to_string(),
            ),
            persist_item(
                enable_label.to_string(),
                enable_description.to_string(),
                enable_config,
                "Updated ZAP integration".to_string(),
            ),
            persist_item(
                "Clear API key".to_string(),
                "Remove the saved ZAP API key from config.".to_string(),
                SecurityZapConfig {
                    api_key: None,
                    ..current_config.clone()
                },
                "Cleared ZAP API key".to_string(),
            ),
        ];

        let mut header = ColumnRenderable::new();
        header.push(Line::from("Configure ZAP".bold()));
        header.push(Line::from(
            format!(
                "Enabled: {}  Base URL: {}",
                if current_config.enabled { "yes" } else { "no" },
                current_config.base_url
            )
            .dim(),
        ));
        header.push(Line::from(
            "Use /zap status, /zap url <http://host:port>, /zap key <value>, or /zap clear-key."
                .dim(),
        ));

        self.bottom_pane.show_selection_view(SelectionViewParams {
            header: Box::new(header),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        });
    }

    pub(crate) fn handle_zap_inline_args(&mut self, args: &str) -> bool {
        let trimmed = args.trim();
        if trimmed.is_empty() {
            self.open_zap_popup();
            return true;
        }

        let (command, remainder) = trimmed.split_once(' ').map_or((trimmed, ""), |parts| parts);
        match command.trim().to_ascii_lowercase().as_str() {
            "status" | "test" => {
                self.spawn_zap_status_check(self.current_zap_config());
                true
            }
            "enable" | "on" => {
                let mut config = self.current_zap_config();
                config.enabled = true;
                self.queue_zap_selection(config, "Enabled ZAP integration".to_string());
                true
            }
            "disable" | "off" => {
                let mut config = self.current_zap_config();
                config.enabled = false;
                self.queue_zap_selection(config, "Disabled ZAP integration".to_string());
                true
            }
            "clear-key" => {
                let mut config = self.current_zap_config();
                config.api_key = None;
                self.queue_zap_selection(config, "Cleared ZAP API key".to_string());
                true
            }
            "url" => match normalize_zap_base_url(remainder) {
                Ok(base_url) => {
                    let mut config = self.current_zap_config();
                    config.base_url = base_url;
                    self.queue_zap_selection(config, "Saved ZAP base URL".to_string());
                    true
                }
                Err(err) => {
                    self.add_error_message(err);
                    false
                }
            },
            "key" => {
                let api_key = remainder.trim();
                if api_key.is_empty() {
                    self.add_error_message(
                        "Usage: /zap key <api-key> or /zap clear-key".to_string(),
                    );
                    return false;
                }
                let mut config = self.current_zap_config();
                config.api_key = Some(api_key.to_string());
                self.queue_zap_selection(config, "Saved ZAP API key".to_string());
                true
            }
            _ => {
                self.add_error_message(
                    "Usage: /zap [status|test|enable|disable|url <http://host:port>|key <value>|clear-key]"
                        .to_string(),
                );
                false
            }
        }
    }

    pub(crate) fn set_security_zap(&mut self, config: SecurityZapConfig) {
        self.config.security_zap = config;
    }

    fn current_zap_config(&self) -> SecurityZapConfig {
        self.config.security_zap.clone()
    }

    fn queue_zap_selection(&self, config: SecurityZapConfig, label: String) {
        self.app_event_tx
            .send(AppEvent::PersistZapSelection { config, label });
    }

    fn spawn_zap_status_check(&self, config: SecurityZapConfig) {
        let tx = self.app_event_tx.clone();
        tokio::spawn(async move {
            let report = build_zap_status_report(config).await;
            tx.send(report);
        });
    }
}

fn selection_item(name: String, description: String, config: SecurityZapConfig) -> SelectionItem {
    let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
        let status_config = config.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(build_zap_status_report(status_config).await);
        });
    })];

    SelectionItem {
        name,
        description: Some(description),
        actions,
        dismiss_on_select: true,
        ..Default::default()
    }
}

fn persist_item(
    name: String,
    description: String,
    config: SecurityZapConfig,
    label: String,
) -> SelectionItem {
    let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
        tx.send(AppEvent::PersistZapSelection {
            config: config.clone(),
            label: label.clone(),
        });
    })];

    SelectionItem {
        name,
        description: Some(description),
        actions,
        dismiss_on_select: true,
        ..Default::default()
    }
}

fn normalize_zap_base_url(value: &str) -> Result<String, String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return Err("Usage: /zap url <http://host:port>".to_string());
    }

    let parsed =
        Url::parse(trimmed).map_err(|err| format!("Invalid ZAP URL `{trimmed}`: {err}"))?;
    match parsed.scheme() {
        "http" | "https" => Ok(trimmed.to_string()),
        scheme => Err(format!(
            "Invalid ZAP URL scheme `{scheme}`; use http or https."
        )),
    }
}

async fn build_zap_status_report(config: SecurityZapConfig) -> AppEvent {
    if !config.enabled {
        return AppEvent::ZapStatusReport {
            message: format!(
                "ZAP integration is disabled. Saved base URL: {}.",
                config.base_url
            ),
            hint: Some("Run /zap enable to re-enable ZAP-backed scanning.".to_string()),
            is_error: false,
        };
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(ZAP_STATUS_TIMEOUT_SECS))
        .build();
    let client = match client {
        Ok(client) => client,
        Err(err) => {
            return AppEvent::ZapStatusReport {
                message: format!("Failed to build a ZAP probe client: {err}"),
                hint: None,
                is_error: true,
            };
        }
    };

    let endpoint = format!("{}/JSON/core/view/version/", config.base_url);
    let mut request = client.get(&endpoint);
    if let Some(api_key) = &config.api_key {
        request = request.query(&[("apikey", api_key)]);
    }

    match request.send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            if !status.is_success() {
                return AppEvent::ZapStatusReport {
                    message: format!(
                        "ZAP API probe failed with HTTP {status} at {}.",
                        config.base_url
                    ),
                    hint: Some(body),
                    is_error: true,
                };
            }

            match serde_json::from_str::<Value>(&body).ok().and_then(|json| {
                json.get("version")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            }) {
                Some(version) => AppEvent::ZapStatusReport {
                    message: format!("Connected to ZAP {} at {}.", version, config.base_url),
                    hint: Some(if config.api_key.is_some() {
                        "API key is configured for this session.".to_string()
                    } else {
                        "No ZAP API key is configured.".to_string()
                    }),
                    is_error: false,
                },
                None => AppEvent::ZapStatusReport {
                    message: format!(
                        "ZAP API at {} responded successfully, but the version field was missing.",
                        config.base_url
                    ),
                    hint: Some(body),
                    is_error: true,
                },
            }
        }
        Err(err) => AppEvent::ZapStatusReport {
            message: format!("Failed to reach ZAP at {}: {err}", config.base_url),
            hint: Some(
                "Make sure ZAP is running with the API enabled, then run /zap status again."
                    .to_string(),
            ),
            is_error: true,
        },
    }
}
