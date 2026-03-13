# Tech Stack

## Stack decisions

| Layer | Technology | Rationale |
|---|---|---|
| Game demos | Rust + macroquad → WASM | Self-contained WASM binaries. macroquad is small, has clean WASM output, and is designed for micro-demos, not full engines. |
| Backend API | Rust + Axum | Shared types with game crates via `ludeme-core`. SQLx compile-time query checking. Single binary deployment. |
| Database | PostgreSQL | Relational enough for the entity graph via a typed edges table. pgvector available later for semantic search. Full-text search via `tsvector`. |
| Frontend shell | SvelteKit + TypeScript | Handles content-heavy sites well, embeds WASM cleanly, lighter than React for this use case. Types generated from Rust core via `ts-rs`. |
| Taxonomy / config | TOML files in version control | Human-readable, append-only, seeded into the database on startup. No migration required to add a new mechanic family or relation type. |
| Tooling scripts | Python (tools/ only) | Acceptable for bulk import, one-off data migration, or research scripting against the API. Not part of the core stack. |

## Why full Rust over Python + Rust

The primary argument is the **shared types crate**. Entity types — Mechanic, Work, Demo, Observation, RelationshipEdge, GameEvent — live in `ludeme-core` and are imported by both the backend and every game crate. TypeScript types for the shell are generated from the same source via `ts-rs`. Define a type once; it is correct everywhere at compile time. In a Python/Rust split you define it twice and schema drift becomes a matter of when, not if.

Secondary arguments: SQLx compile-time query verification is stronger than anything available in Python. Single binary deployment eliminates runtime environment management. The boilerplate cost is real but front-loaded.

The only future pressure point is semantic search over mechanic definitions and observations. pgvector plus periodic embedding generation via an external API endpoint handles this at the scale Ludeme will operate for a long time. It does not require Python in the server.

## Frontend exception

SvelteKit stays TypeScript, not Rust (Leptos, Dioxus). The Rust frontend ecosystem is not mature enough for a content-heavy site with SEO requirements, complex layouts, and a solo development pace. SvelteKit with generated types from `ludeme-core` is a clean boundary, not a compromise.
