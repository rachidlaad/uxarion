# ADR 0001: Use a single canonical public repo

## Status

Accepted

## Decision

Use `rachidlaad/uxarion` as the canonical public source repo for the project.

## Why

- one clear home for contributors
- one clear home for releases
- one clear home for issues and roadmap
- avoids drift between source, releases, and docs

## Consequences

- the old fork is not the source of truth
- docs and update paths should point to `uxarion`
- release operations should target `uxarion/main`
