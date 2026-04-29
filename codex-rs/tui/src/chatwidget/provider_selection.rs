use super::ChatWidget;
use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionAction;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;
use crate::render::renderable::ColumnRenderable;
use codex_core::ANTHROPIC_PROVIDER_ID;
use codex_core::LMSTUDIO_OSS_PROVIDER_ID;
use codex_core::OLLAMA_OSS_PROVIDER_ID;
use codex_core::OPENAI_PROVIDER_ID;
use codex_utils_oss::get_default_model_for_oss_provider;
use ratatui::style::Stylize;
use ratatui::text::Line;

const API_DEFAULT_MODEL: &str = "gpt-5.4";
const ANTHROPIC_DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";

impl ChatWidget {
    pub(crate) fn open_provider_popup(&mut self) {
        if !self.is_session_configured() {
            self.add_info_message(
                "Provider selection is disabled until startup completes.".to_string(),
                None,
            );
            return;
        }

        let current_provider_id = self.config.model_provider_id.clone();
        let items = [
            OPENAI_PROVIDER_ID,
            ANTHROPIC_PROVIDER_ID,
            OLLAMA_OSS_PROVIDER_ID,
            LMSTUDIO_OSS_PROVIDER_ID,
        ]
        .into_iter()
        .map(|provider_id| {
            let label = provider_label(provider_id).to_string();
            let provider_id_string = provider_id.to_string();
            let model = default_model_for_provider(provider_id).to_string();
            let action_label = label.clone();
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::PersistProviderSelection {
                    provider_id: provider_id_string.clone(),
                    model: model.clone(),
                    label: action_label.clone(),
                });
            })];
            SelectionItem {
                name: label,
                description: Some(provider_description(provider_id)),
                is_current: is_popup_current_provider(provider_id, &current_provider_id),
                actions,
                dismiss_on_select: true,
                ..Default::default()
            }
        })
        .collect();

        let mut header = ColumnRenderable::new();
        header.push(Line::from("Select Provider".bold()));
        header.push(Line::from(
            "Choose the default backend for new sessions. The current session stays unchanged."
                .dim(),
        ));

        self.bottom_pane.show_selection_view(SelectionViewParams {
            header: Box::new(header),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        });
    }

    pub(crate) fn handle_provider_inline_args(&mut self, args: &str) -> bool {
        let normalized = args.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "api" | "default" | "openai" => {
                self.queue_provider_selection(OPENAI_PROVIDER_ID);
                true
            }
            "anthropic" | "claude" => {
                self.queue_provider_selection(ANTHROPIC_PROVIDER_ID);
                true
            }
            "ollama" => {
                self.queue_provider_selection(OLLAMA_OSS_PROVIDER_ID);
                true
            }
            "lmstudio" | "lm-studio" => {
                self.queue_provider_selection(LMSTUDIO_OSS_PROVIDER_ID);
                true
            }
            "status" => {
                self.add_info_message(current_provider_status_message(self), Some(provider_hint()));
                true
            }
            _ => {
                self.add_error_message(
                    "Usage: /provider [api|openai|anthropic|claude|ollama|lmstudio|status]"
                        .to_string(),
                );
                false
            }
        }
    }

    fn queue_provider_selection(&self, provider_id: &str) {
        self.app_event_tx.send(AppEvent::PersistProviderSelection {
            provider_id: provider_id.to_string(),
            model: default_model_for_provider(provider_id).to_string(),
            label: provider_label(provider_id).to_string(),
        });
    }
}

fn default_model_for_provider(provider_id: &str) -> &'static str {
    match provider_id {
        OLLAMA_OSS_PROVIDER_ID | LMSTUDIO_OSS_PROVIDER_ID => {
            get_default_model_for_oss_provider(provider_id).unwrap_or(API_DEFAULT_MODEL)
        }
        ANTHROPIC_PROVIDER_ID => ANTHROPIC_DEFAULT_MODEL,
        _ => API_DEFAULT_MODEL,
    }
}

fn provider_label(provider_id: &str) -> &'static str {
    match provider_id {
        OLLAMA_OSS_PROVIDER_ID => "Ollama (local)",
        LMSTUDIO_OSS_PROVIDER_ID => "LM Studio (local)",
        ANTHROPIC_PROVIDER_ID => "Claude (Anthropic)",
        _ => "OpenAI",
    }
}

fn provider_description(provider_id: &str) -> String {
    match provider_id {
        OLLAMA_OSS_PROVIDER_ID => format!(
            "Use a local Ollama server. Default model: {}.",
            default_model_for_provider(provider_id)
        ),
        LMSTUDIO_OSS_PROVIDER_ID => format!(
            "Use a local LM Studio server. Default model: {}.",
            default_model_for_provider(provider_id)
        ),
        ANTHROPIC_PROVIDER_ID => format!(
            "Use Claude models with a saved API key. Default model: {}.",
            default_model_for_provider(provider_id)
        ),
        _ => format!(
            "Use ChatGPT sign-in or a saved OpenAI API key. Default model: {}.",
            default_model_for_provider(provider_id)
        ),
    }
}

fn is_popup_current_provider(candidate_provider_id: &str, current_provider_id: &str) -> bool {
    candidate_provider_id == current_provider_id
}

fn current_provider_status_message(chat: &ChatWidget) -> String {
    let current_provider_id = chat.config.model_provider_id.as_str();
    let current_model = chat.current_model();
    match current_provider_id {
        OLLAMA_OSS_PROVIDER_ID => {
            format!("Current session provider: Ollama (local). Current model: {current_model}.")
        }
        LMSTUDIO_OSS_PROVIDER_ID => {
            format!("Current session provider: LM Studio (local). Current model: {current_model}.")
        }
        OPENAI_PROVIDER_ID => {
            format!("Current session provider: OpenAI. Current model: {current_model}.")
        }
        ANTHROPIC_PROVIDER_ID => {
            format!("Current session provider: Claude (Anthropic). Current model: {current_model}.")
        }
        other => format!(
            "Current session provider: API-backed ({other}). Current model: {current_model}."
        ),
    }
}

fn provider_hint() -> String {
    "Use /provider openai, /provider anthropic, /provider ollama, or /provider lmstudio to save the default for future sessions."
        .to_string()
}
