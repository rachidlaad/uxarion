# ADR 0003: Keep API as the default provider

## Status

Accepted

## Decision

Keep the API-backed provider as the default product experience and treat local models as an opt-in advanced path.

## Why

- strongest out-of-the-box model quality
- lower first-run friction for most users
- fewer setup variables than local inference stacks

## Consequences

- `/provider` should keep API as the default
- local providers need clear docs and explicit selection
- provider changes must be saved clearly and must not silently route API users to local backends
