# Roadmap Context

This file explains why the current roadmap items exist.

## Product direction

Uxarion should become a strong local-first security workflow tool, not a generic autonomous agent.

That means the roadmap should favor:

- operator trust
- scoped testing
- evidence quality
- useful integrations
- repeatable workflows

## Current open roadmap items

### #13 Build an evidence-to-report pipeline

This is one of the highest-value product upgrades.

Why it matters:

- turns raw session work into reusable output
- improves handoff quality
- helps the product feel complete, not just exploratory

### #17 Add Claude provider support

Why it matters:

- avoids over-dependence on one API provider
- gives users more model choice without changing the product shape

### #18 Add Burp Suite integration support

Why it matters:

- many real users already work in Burp
- ZAP should not be the only proxy/workflow path

### #19 Build an engagement graph for target mapping

Why it matters:

- lets Uxarion reason across hosts, endpoints, evidence, and findings
- creates a stronger system-level testing workflow

### #20 Add record-and-replay auth support

Why it matters:

- authenticated testing is a major real-world blocker
- login reuse is a big multiplier for actual usefulness

### #21 Add a scratch automation workspace

Why it matters:

- allows useful automation without touching the user's real workspace by default
- supports controlled script and PoC generation

### #22 Add HackerOne integration support

Why it matters:

- lets Uxarion become program-aware
- improves scope safety and reporting fit for real bounty workflows

### #23 Improve startup update checks so releases are surfaced promptly

Why it matters:

- same-day releases should be visible promptly
- public trust drops when the updater misses obvious available updates

## Prioritization guidance

Current order of product value:

1. reporting and evidence quality
2. auth and testing workflow strength
3. integration depth
4. broader provider support

That means `#13`, `#20`, and `#19` are likely more product-defining than provider expansion alone.
