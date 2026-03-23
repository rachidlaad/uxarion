# ADR 0002: Keep npm as a wrapper around a native runtime

## Status

Accepted

## Decision

Use the npm package as a thin distribution wrapper that downloads the native runtime archive from the canonical repo.

## Why

- keeps install simple for users
- avoids making users compile Rust locally
- allows the same runtime to be used by npm and direct-install paths

## Consequences

- GitHub release assets and the raw runtime path must stay healthy
- runtime version and npm version should usually move together
- release verification must include a fresh npm install smoke test
