use crate::client_common::Prompt;
use crate::client_common::ResponseEvent;
use crate::client_common::ResponseStream;
use crate::client_common::tools::ResponsesApiTool;
use crate::client_common::tools::ToolSpec;
use crate::error::CodexErr;
use crate::error::Result;
use crate::error::UnexpectedResponseError;
use crate::models_manager::model_info::BASE_INSTRUCTIONS;
use codex_api::Provider as ApiProvider;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::LocalShellAction;
use codex_protocol::models::MessagePhase;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ApplyPatchToolType;
use codex_protocol::openai_models::ConfigShellToolType;
use codex_protocol::openai_models::InputModality;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::TruncationPolicyConfig;
use codex_protocol::openai_models::WebSearchToolType;
use codex_protocol::openai_models::default_input_modalities;
use codex_protocol::protocol::TokenUsage;
use http::HeaderValue;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc;

const ANTHROPIC_API_VERSION: &str = "2023-06-01";
const DEFAULT_CONTEXT_WINDOW: i64 = 200_000;
const DEFAULT_MAX_OUTPUT_TOKENS: i64 = 8_192;

pub(crate) fn built_in_models() -> Vec<ModelInfo> {
    vec![
        built_in_model(
            "claude-sonnet-4-20250514",
            "Claude Sonnet 4",
            Some(
                "High-performance Claude model with strong reasoning and coding performance."
                    .to_string(),
            ),
            0,
            default_input_modalities(),
        ),
        built_in_model(
            "claude-opus-4-1-20250805",
            "Claude Opus 4.1",
            Some(
                "Most capable Claude model, optimized for advanced reasoning and coding."
                    .to_string(),
            ),
            1,
            default_input_modalities(),
        ),
        built_in_model(
            "claude-opus-4-20250514",
            "Claude Opus 4",
            Some("Powerful Claude model for deep reasoning and complex coding tasks.".to_string()),
            2,
            default_input_modalities(),
        ),
        built_in_model(
            "claude-3-7-sonnet-20250219",
            "Claude Sonnet 3.7",
            Some("Balanced Claude model with strong reasoning and coding performance.".to_string()),
            3,
            default_input_modalities(),
        ),
        built_in_model(
            "claude-3-5-haiku-20241022",
            "Claude Haiku 3.5",
            Some("Fast Claude model for lightweight tasks and shorter responses.".to_string()),
            4,
            vec![InputModality::Text],
        ),
    ]
}

#[derive(Debug, Serialize)]
struct AnthropicMessagesRequest {
    model: String,
    max_tokens: i64,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "String::is_empty")]
    system: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicTool>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicContentBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_error: bool,
    },
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicMessagesResponse {
    id: String,
    content: Vec<AnthropicResponseBlock>,
    stop_reason: Option<String>,
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicResponseBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: i64,
    output_tokens: i64,
    #[serde(default)]
    cache_creation_input_tokens: Option<i64>,
    #[serde(default)]
    cache_read_input_tokens: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModelsPage {
    data: Vec<AnthropicModel>,
    has_more: bool,
    last_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModel {
    id: String,
    display_name: String,
    #[serde(default)]
    max_input_tokens: Option<i64>,
    #[serde(default)]
    capabilities: Option<AnthropicModelCapabilities>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModelCapabilities {
    #[serde(default)]
    vision: bool,
}

pub(crate) async fn list_models(
    http_client: &reqwest::Client,
    provider: &ApiProvider,
    api_key: &str,
) -> Result<Vec<ModelInfo>> {
    let mut after_id: Option<String> = None;
    let mut models = Vec::new();

    loop {
        let mut request = http_client.get(provider.url_for_path("/v1/models"));
        request = request.headers(build_headers(provider, api_key)?);
        request = request.query(&[("limit", "1000")]);
        if let Some(cursor) = after_id.as_deref() {
            request = request.query(&[("after_id", cursor)]);
        }

        let response = request.send().await.map_err(|err| {
            CodexErr::Stream(format!("failed to fetch Anthropic models: {err}"), None)
        })?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text().await.map_err(|err| {
            CodexErr::Stream(
                format!("failed to read Anthropic models response: {err}"),
                None,
            )
        })?;
        if !status.is_success() {
            return Err(map_anthropic_http_error(
                status,
                provider.url_for_path("/v1/models"),
                headers
                    .get("request-id")
                    .and_then(|value| value.to_str().ok()),
                body,
            ));
        }

        let page: AnthropicModelsPage = serde_json::from_str(&body)?;
        models.extend(page.data.into_iter().map(model_info_from_anthropic_model));
        if !page.has_more {
            break;
        }
        after_id = page.last_id;
        if after_id.is_none() {
            break;
        }
    }

    Ok(models)
}

pub(crate) async fn send_message(
    http_client: &reqwest::Client,
    provider: &ApiProvider,
    api_key: &str,
    prompt: &Prompt,
    model_info: &ModelInfo,
) -> Result<ResponseStream> {
    let request_body = build_messages_request(prompt, model_info)?;
    let response = http_client
        .post(provider.url_for_path("/v1/messages"))
        .headers(build_headers(provider, api_key)?)
        .json(&request_body)
        .send()
        .await
        .map_err(|err| CodexErr::Stream(format!("failed to connect to Anthropic: {err}"), None))?;

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.map_err(|err| {
        CodexErr::Stream(
            format!("failed to read Anthropic response body: {err}"),
            None,
        )
    })?;
    if !status.is_success() {
        return Err(map_anthropic_http_error(
            status,
            provider.url_for_path("/v1/messages"),
            headers
                .get("request-id")
                .and_then(|value| value.to_str().ok()),
            body,
        ));
    }

    let parsed: AnthropicMessagesResponse = serde_json::from_str(&body)?;
    let events = response_to_events(parsed)?;
    let (tx, rx_event) = mpsc::channel(events.len().max(1));
    tokio::spawn(async move {
        for event in events {
            if tx.send(Ok(event)).await.is_err() {
                break;
            }
        }
    });
    Ok(ResponseStream { rx_event })
}

fn build_headers(provider: &ApiProvider, api_key: &str) -> Result<http::HeaderMap> {
    let mut headers = provider.headers.clone();
    headers.insert(
        "x-api-key",
        HeaderValue::from_str(api_key)
            .map_err(|err| CodexErr::Fatal(format!("invalid Anthropic API key header: {err}")))?,
    );
    headers.insert(
        "anthropic-version",
        HeaderValue::from_static(ANTHROPIC_API_VERSION),
    );
    Ok(headers)
}

fn build_messages_request(
    prompt: &Prompt,
    model_info: &ModelInfo,
) -> Result<AnthropicMessagesRequest> {
    let messages = prompt_input_to_anthropic_messages(&prompt.input)?;
    if messages.is_empty() {
        return Err(CodexErr::InvalidRequest(
            "Anthropic requests require at least one user or assistant message.".to_string(),
        ));
    }

    Ok(AnthropicMessagesRequest {
        model: model_info.slug.clone(),
        max_tokens: DEFAULT_MAX_OUTPUT_TOKENS,
        messages,
        system: prompt.base_instructions.text.clone(),
        tools: prompt
            .tools
            .iter()
            .try_fold(Vec::new(), |mut tools, tool| {
                if let Some(tool) = tool_spec_to_anthropic_tool(tool)? {
                    tools.push(tool);
                }
                Ok::<Vec<AnthropicTool>, CodexErr>(tools)
            })?,
    })
}

fn prompt_input_to_anthropic_messages(input: &[ResponseItem]) -> Result<Vec<AnthropicMessage>> {
    let mut messages = Vec::new();

    for item in input {
        match item {
            ResponseItem::Message { role, content, .. }
                if role == "user" || role == "assistant" =>
            {
                let blocks = content
                    .iter()
                    .filter_map(content_item_to_text_block)
                    .collect::<Vec<_>>();
                if !blocks.is_empty() {
                    messages.push(AnthropicMessage {
                        role: role.clone(),
                        content: blocks,
                    });
                }
            }
            ResponseItem::Message { .. }
            | ResponseItem::Reasoning { .. }
            | ResponseItem::WebSearchCall { .. }
            | ResponseItem::ImageGenerationCall { .. }
            | ResponseItem::GhostSnapshot { .. }
            | ResponseItem::Compaction { .. }
            | ResponseItem::Other => {}
            ResponseItem::FunctionCall {
                name,
                arguments,
                call_id,
                ..
            } => {
                messages.push(AnthropicMessage {
                    role: "assistant".to_string(),
                    content: vec![AnthropicContentBlock::ToolUse {
                        id: call_id.clone(),
                        name: name.clone(),
                        input: serde_json::from_str(arguments).unwrap_or(Value::Null),
                    }],
                });
            }
            ResponseItem::FunctionCallOutput { call_id, output } => {
                messages.push(AnthropicMessage {
                    role: "user".to_string(),
                    content: vec![AnthropicContentBlock::ToolResult {
                        tool_use_id: call_id.clone(),
                        content: function_output_to_text(output),
                        is_error: output.success == Some(false),
                    }],
                });
            }
            ResponseItem::LocalShellCall {
                action: LocalShellAction::Exec(exec),
                call_id,
                ..
            } => {
                messages.push(AnthropicMessage {
                    role: "assistant".to_string(),
                    content: vec![AnthropicContentBlock::ToolUse {
                        id: call_id.clone().unwrap_or_else(|| "local_shell".to_string()),
                        name: "local_shell".to_string(),
                        input: serde_json::json!({
                            "command": exec.command,
                            "timeout_ms": exec.timeout_ms,
                            "working_directory": exec.working_directory,
                            "env": exec.env,
                            "user": exec.user,
                        }),
                    }],
                });
            }
            ResponseItem::CustomToolCall { .. } | ResponseItem::CustomToolCallOutput { .. } => {}
        }
    }

    Ok(messages)
}

fn content_item_to_text_block(item: &ContentItem) -> Option<AnthropicContentBlock> {
    match item {
        ContentItem::InputText { text } | ContentItem::OutputText { text } => {
            Some(AnthropicContentBlock::Text { text: text.clone() })
        }
        ContentItem::InputImage { .. } => Some(AnthropicContentBlock::Text {
            text: "[Image input omitted by the Anthropic bridge]".to_string(),
        }),
    }
}

fn function_output_to_text(output: &FunctionCallOutputPayload) -> String {
    if let Some(text) = output.text_content() {
        return text.to_string();
    }

    output
        .content_items()
        .unwrap_or_default()
        .iter()
        .filter_map(|item| match item {
            FunctionCallOutputContentItem::InputText { text } => Some(text.clone()),
            FunctionCallOutputContentItem::InputImage { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn tool_spec_to_anthropic_tool(tool: &ToolSpec) -> Result<Option<AnthropicTool>> {
    match tool {
        ToolSpec::Function(ResponsesApiTool {
            name,
            description,
            parameters,
            ..
        }) => Ok(Some(AnthropicTool {
            name: name.clone(),
            description: description.clone(),
            input_schema: serde_json::to_value(parameters)?,
        })),
        ToolSpec::WebSearch { .. } | ToolSpec::ImageGeneration { .. } => Ok(None),
        ToolSpec::LocalShell {} | ToolSpec::Freeform(_) => {
            Err(CodexErr::UnsupportedOperation(format!(
                "Anthropic support currently requires function-style tools only. Unsupported tool: {}",
                tool.name()
            )))
        }
    }
}

fn response_to_events(response: AnthropicMessagesResponse) -> Result<Vec<ResponseEvent>> {
    let mut events = vec![ResponseEvent::Created];
    let mut accumulated_text = String::new();
    let tool_use_response = response.stop_reason.as_deref() == Some("tool_use");

    for block in response.content {
        match block {
            AnthropicResponseBlock::Text { text } => {
                accumulated_text.push_str(&text);
            }
            AnthropicResponseBlock::ToolUse { id, name, input } => {
                if !accumulated_text.is_empty() {
                    events.push(ResponseEvent::OutputItemDone(ResponseItem::Message {
                        id: None,
                        role: "assistant".to_string(),
                        content: vec![ContentItem::OutputText {
                            text: std::mem::take(&mut accumulated_text),
                        }],
                        end_turn: Some(false),
                        phase: Some(MessagePhase::Commentary),
                    }));
                }
                events.push(ResponseEvent::OutputItemDone(ResponseItem::FunctionCall {
                    id: None,
                    name,
                    arguments: serde_json::to_string(&input)?,
                    call_id: id,
                }));
            }
            AnthropicResponseBlock::Other => {}
        }
    }

    if !accumulated_text.is_empty() {
        events.push(ResponseEvent::OutputItemDone(ResponseItem::Message {
            id: None,
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: accumulated_text,
            }],
            end_turn: Some(!tool_use_response),
            phase: Some(if tool_use_response {
                MessagePhase::Commentary
            } else {
                MessagePhase::FinalAnswer
            }),
        }));
    }

    events.push(ResponseEvent::Completed {
        response_id: response.id,
        token_usage: response.usage.map(|usage| TokenUsage {
            input_tokens: usage.input_tokens,
            cached_input_tokens: usage.cache_read_input_tokens.unwrap_or(0)
                + usage.cache_creation_input_tokens.unwrap_or(0),
            output_tokens: usage.output_tokens,
            reasoning_output_tokens: 0,
            total_tokens: usage.input_tokens + usage.output_tokens,
        }),
    });

    Ok(events)
}

fn map_anthropic_http_error(
    status: StatusCode,
    url: String,
    request_id: Option<&str>,
    body: String,
) -> CodexErr {
    match status {
        StatusCode::BAD_REQUEST => CodexErr::InvalidRequest(body),
        StatusCode::INTERNAL_SERVER_ERROR => CodexErr::InternalServerError,
        StatusCode::SERVICE_UNAVAILABLE => CodexErr::ServerOverloaded,
        _ => CodexErr::UnexpectedStatus(UnexpectedResponseError {
            status,
            body,
            url: Some(url),
            cf_ray: None,
            request_id: request_id.map(str::to_string),
        }),
    }
}

fn model_info_from_anthropic_model(model: AnthropicModel) -> ModelInfo {
    let fallback = built_in_models()
        .into_iter()
        .find(|built_in| built_in.slug == model.id);
    let context_window = model
        .max_input_tokens
        .or_else(|| fallback.as_ref().and_then(|model| model.context_window))
        .or(Some(DEFAULT_CONTEXT_WINDOW));
    let input_modalities = if model.capabilities.as_ref().is_some_and(|caps| caps.vision) {
        default_input_modalities()
    } else {
        fallback
            .as_ref()
            .map(|model| model.input_modalities.clone())
            .unwrap_or_else(|| vec![InputModality::Text])
    };
    let slug = model.id;
    let display_name = model.display_name;
    let priority = if let Some(fallback) = fallback.as_ref() {
        fallback.priority
    } else if slug.contains("sonnet") {
        0
    } else if slug.contains("opus") {
        1
    } else if slug.contains("haiku") {
        2
    } else {
        10
    };

    ModelInfo {
        slug,
        display_name,
        description: fallback
            .as_ref()
            .and_then(|model| model.description.clone()),
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::ShellCommand,
        visibility: ModelVisibility::List,
        minimal_client_version: None,
        supported_in_api: true,
        priority,
        available_in_plans: Vec::new(),
        availability_nux: None,
        upgrade: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: Some(ApplyPatchToolType::Function),
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::bytes(10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window,
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities,
        prefer_websockets: false,
        used_fallback_model_metadata: false,
    }
}

fn built_in_model(
    slug: &str,
    display_name: &str,
    description: Option<String>,
    priority: i32,
    input_modalities: Vec<InputModality>,
) -> ModelInfo {
    ModelInfo {
        slug: slug.to_string(),
        display_name: display_name.to_string(),
        description,
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::ShellCommand,
        visibility: ModelVisibility::List,
        minimal_client_version: None,
        supported_in_api: true,
        priority,
        available_in_plans: Vec::new(),
        availability_nux: None,
        upgrade: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: Some(ApplyPatchToolType::Function),
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::bytes(10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window: Some(DEFAULT_CONTEXT_WINDOW),
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities,
        prefer_websockets: false,
        used_fallback_model_metadata: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::spec::JsonSchema;
    use codex_protocol::models::ContentItem;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    fn prompt_with_tools(tools: Vec<ToolSpec>) -> Prompt {
        Prompt {
            input: vec![ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "hello".to_string(),
                }],
                end_turn: None,
                phase: None,
            }],
            tools,
            ..Prompt::default()
        }
    }

    fn test_function_tool(name: &str) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: name.to_string(),
            description: "Test function tool.".to_string(),
            strict: false,
            parameters: JsonSchema::Object {
                properties: BTreeMap::new(),
                required: None,
                additional_properties: None,
            },
            output_schema: None,
        })
    }

    #[test]
    fn build_messages_request_skips_openai_native_tools_for_anthropic() {
        let prompt = prompt_with_tools(vec![
            test_function_tool("shell_command"),
            ToolSpec::WebSearch {
                external_web_access: Some(true),
                filters: None,
                user_location: None,
                search_context_size: None,
                search_content_types: None,
            },
            ToolSpec::ImageGeneration {
                output_format: "png".to_string(),
            },
        ]);
        let model_info = built_in_models()
            .into_iter()
            .next()
            .expect("built-in Anthropic model");

        let request = build_messages_request(&prompt, &model_info).expect("request");

        assert_eq!(request.tools.len(), 1);
        assert_eq!(request.tools[0].name, "shell_command");
    }

    #[test]
    fn build_messages_request_rejects_untranslatable_tools_for_anthropic() {
        let prompt = prompt_with_tools(vec![ToolSpec::LocalShell {}]);
        let model_info = built_in_models()
            .into_iter()
            .next()
            .expect("built-in Anthropic model");

        let error = build_messages_request(&prompt, &model_info).expect_err("unsupported tool");

        assert!(
            matches!(error, CodexErr::UnsupportedOperation(message) if message.contains("local_shell"))
        );
    }
}
