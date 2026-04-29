# Architecture

## Top-level shape

Uxarion is a single-repo project with:

- product source
- public docs
- release artifacts
- npm wrapper
- native runtime packaging

The repo is the canonical home for:

- issues
- roadmap
- releases
- install/update paths

## Main components

### Rust workspace

The main product code lives in `codex-rs/`.

Important areas:

- `codex-rs/core`
  - provider configuration
  - auth storage
  - security tooling
  - ZAP integration
  - request/response behavior
- `codex-rs/tui`
  - terminal UI
  - onboarding
  - slash commands
  - update banners
  - provider and ZAP controls
- `codex-rs/utils/*`
  - shared utilities used across the workspace

### npm wrapper

The npm distribution layer lives in `codex-cli/`.

Important files:

- `codex-cli/package.json`
- `codex-cli/bin/uxarion.js`

The wrapper:

- detects the platform
- downloads the matching runtime archive if needed
- extracts it into the local runtime cache
- launches the native binary
- forces `CODEX_HOME` to `UXARION_HOME` or `~/.uxarion` so the wrapper does not inherit the desktop Codex home by accident
- sets install-channel environment markers for the child process
- defaults runtime sessions to the `security` profile unless the user overrides it explicitly

### Release artifacts

Versioned runtime archives are published as GitHub Release assets under:

- `https://github.com/rachidlaad/uxarion/releases/download/vX.Y.Z/`

Local packaging may still stage archives under:

- `releases/vX.Y.Z/`

Those local artifacts are release scratch output, not the canonical runtime download path for npm installs.

## Provider model

Public product behavior:

- default provider: OpenAI
- optional remote provider: Claude (Anthropic) via saved API key
- optional local providers: Ollama and LM Studio
- security profile honors the configured provider instead of forcing the local Responses-compatible backend

Auth model:

- OpenAI supports either ChatGPT sign-in or a saved OpenAI API key
- Anthropic uses a saved Anthropic API key
- provider changes are persisted for the next session; they do not hot-swap the active session
- the Anthropic bridge translates Uxarion function tools into Anthropic tool definitions and filters provider-incompatible OpenAI-native tools such as `web_search` and image generation

Important note:

- internal provider and crate plumbing still contains inherited `codex` naming
- public behavior and docs should stay Uxarion-branded

## ZAP integration model

Uxarion uses the ZAP API directly.

It does not script the desktop UI.

Current configuration paths:

- slash commands such as `/zap`, `/zap status`, `/zap url`, `/zap key`
- config file values
- env overrides:
  - `UXARION_ZAP_BASE_URL`
  - `UXARION_ZAP_API_KEY`

Default expectation:

- same-machine setups use `http://127.0.0.1:8080`
- WSL + Windows ZAP setups may need the Windows host IP instead

## Reporting model

Uxarion reporting is now a hybrid:

- persisted security session state remains the source of truth
- `/findings` stays local and deterministic
- `/report` runs as a normal model turn with the bundled `security-reporting` system skill
- the low-level `report_write` tool remains the canonical local save path for Markdown artifacts

Current intent:

- the model can inspect saved findings, evidence files, and screenshots to draft a better report
- report output is still written locally to the canonical session report path
- the user-facing report flow is no longer the old app-side deterministic `/report` action
- report generation is limited to the security session artifacts: `findings.json`, `state.json`, `evidence/`, skill references, and the existing session `report.md`
- reports must only be written through `report_write` into the thread's security session directory
- findings and evidence artifacts must likewise be persisted through the built-in security tools instead of shell-written session files
- when exact artifact paths are already available, reporting must not fall back to broad local filesystem searches

## Security scope model

Security scope is now intentionally stricter for exact URLs:

- if the user provides an exact URL, the active assessment is bound to that exact scheme, host, port, and path
- the model should not infer a different route, repo, or local server when the exact target is unavailable
- persisted evidence, findings, and reports all live under the thread's security session directory for one canonical audit trail
- report generation is limited to the security session artifacts: `findings.json`, `state.json`, `evidence/`, skill references, and the existing session `report.md`
- reports must only be written through `report_write` into the thread's security session directory
- shell commands must not fabricate `state.json`, `findings.json`, `report.md`, or `evidence/` files directly during assessment or reporting
- broad local searches across `/root`, `$HOME`, workspaces, or historical sessions are disallowed when the current session artifact paths are already known

## Update model

Current update source:

- GitHub releases from `rachidlaad/uxarion`

Channel-specific update actions:

- npm: `npm install -g uxarion@latest`
- bun: `bun install -g uxarion@latest`
- source checkout: `uxarion update`

Current caveats:

- the update banner depends on cached release metadata and may not surface same-day releases immediately
- Claude provider support is function-tool-first; Anthropic-native web/search/image tool parity is not yet implemented

## Config and local state

Public runtime state is stored under `CODEX_HOME`, which the npm wrapper defaults to the Uxarion home directory.

Common local files include:

- auth storage
- config
- sessions
- cached update metadata
- optional anonymous telemetry install id under `CODEX_HOME/telemetry/install_id`

Tracked repo files must not contain secrets.

Private operator or local-agent notes should live in ignored local files such as `.codex/`.

## Telemetry model

Uxarion telemetry is a separate anonymous product-metrics path and does not reuse the inherited Codex analytics endpoint.

Current design:

- top-level config under `[uxarion_telemetry]`
- global opt-out still honors `[analytics].enabled = false`
- anonymous local install id persisted under `CODEX_HOME/telemetry/install_id`
- client emits small HTTPS JSON event batches to the configured endpoint
- intended backend shape is `Uxarion client -> Supabase Edge Function -> Supabase Postgres`

Current event scope:

- `app_opened`
- `session_started`
- `report_generated`

Current payload intent:

- version, OS, architecture, install channel
- provider id and provider kind
- active profile and security-mode flag
- report kind and whether it was finding-scoped

Out of scope for telemetry:

- prompts
- findings text
- targets and URLs
- evidence contents
- screenshots
- API keys
- local file paths
