# Known Issues

## Startup update banner can miss same-day releases

- Issue: `#23`
- Summary: the updater is cache-first, uses a long freshness window, and refreshes in the background
- Impact: users may stay on an older version without seeing a banner the same day

## Public runtime support is still effectively Linux x64 first

- Summary: current public release flow is reliable for Linux `x86_64`
- Impact: broader cross-platform support is not yet a strong public promise

## Full TUI validation can be slow on mounted Windows workspaces

- Summary: `cargo test -p codex-tui` and `just fix -p codex-tui` can be very slow when the workspace lives under `/mnt/c`
- Impact: maintainers may need to rely on targeted tests and explicit release caveats when working from that environment

## Internal naming still contains inherited `codex-*`

- Summary: public branding is Uxarion, but many internal crates and folders still use `codex-*`
- Impact: large internal renames are risky and should only be done as dedicated refactors

## Claude provider does not yet support provider-native web or image tools

- Summary: Claude support currently uses function-style tool definitions through the Anthropic Messages API bridge.
- Impact: OpenAI-native tools such as `web_search` and image generation are filtered out during Claude turns until provider-native parity is implemented.
