# Ludeme — System Documentation

> **Ludeme** is a platform where micro-demos, mechanic research, lineage maps, comparisons, and experiments live in one connected system.

This directory contains the living system documentation for the Ludeme project. Each document covers a distinct area. Read them in order for a full picture, or jump to whichever area you need.

## Reading Order

| # | Document | Covers |
|---|---|---|
| 01 | [Vision & Guardrails](01-vision.md) | Name, product thesis, design principles, non-goals, and system guardrails |
| 02 | [Tech Stack](02-tech-stack.md) | Stack decisions, Rust rationale, frontend exception |
| 03 | [Crate Architecture](03-crate-architecture.md) | Workspace layout, crate responsibilities, WASM build target |
| 04 | [Shell API Contract](04-shell-api-contract.md) | Demo↔Shell protocol, event vocabulary, GameState trait, replay |
| 05 | [Systems Catalog](05-systems-catalog.md) | All designed subsystems (play shell, tuner, overlays, compare, replay, heat maps, etc.) |
| 06 | [Domain Model](06-domain-model.md) | Entity reference and database schema sketch |
| 07 | [Fidelity Specification](07-fidelity-spec.md) | Faithful / Interpreted / Experimental levels and publish gate |
| 08 | [Taxonomy](08-taxonomy.md) | Mechanic families, relation types, confidence levels |
| 09 | [UX Flows](09-ux-flows.md) | Primary user journeys — play-to-note, compare, research-to-play, experiment |
| 10 | [Roadmap](10-roadmap.md) | Phases 0–5, deliverables, exit criteria, proving slices |
| 11 | [Source-of-Truth Documents](11-source-of-truth.md) | List of living documents and when to update them |

## Conventions

- **Update when direction changes**, not just when features ship.
- These docs replace the original monolithic `ludeme_system-documentation.md`.
- The taxonomy TOML files (`taxonomy/*.toml`) are the canonical data source — these docs describe their structure and purpose.
