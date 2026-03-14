# Tech Stack

## Stack decisions

| Layer | Technology | Rationale |
|---|---|---|
| Game demos | Rust + macroquad → WASM | Self-contained WASM binaries. macroquad is small, has clean WASM output, and is designed for micro-demos, not full engines. |
| Backend API | Rust + Axum | Shared types with game crates via `ludeme-core`. SQLx compile-time query checking. Single binary deployment. |
| Database | SQLite (WAL mode) via SQLx | Read-heavy, write-light, single-server workload. No separate process — the DB is a file. Full-text search via FTS5 virtual tables. SQLx compile-time query checking works identically to Postgres. See `DECISION-LOG.md` for the full rationale. |
| Frontend shell | SvelteKit + TypeScript | Handles content-heavy sites well, embeds WASM cleanly, lighter than React for this use case. Types generated from Rust core via `ts-rs`. |
| Taxonomy / config | TOML files in version control | Human-readable, append-only, seeded into the database on startup. No migration required to add a new mechanic family or relation type. |
| Tooling scripts | Python (tools/ only) | Acceptable for bulk import, one-off data migration, or research scripting against the API. Not part of the core stack. |

## Why full Rust over Python + Rust

The primary argument is the **shared types crate**. Entity types — Mechanic, Work, Demo, Observation, RelationshipEdge, GameEvent — live in `ludeme-core` and are imported by both the backend and every game crate. TypeScript types for the shell are generated from the same source via `ts-rs`. Define a type once; it is correct everywhere at compile time. In a Python/Rust split you define it twice and schema drift becomes a matter of when, not if.

Secondary arguments: SQLx compile-time query verification is stronger than anything available in Python. Single binary deployment eliminates runtime environment management. The boilerplate cost is real but front-loaded.

The only future pressure point is semantic search over mechanic definitions and observations. SQLite FTS5 handles keyword search natively. For embedding-based semantic search, the plan is to store vectors as JSON blobs and compute cosine similarity in the app layer — sufficient at Ludeme's scale. If this proves inadequate, migration to Turso (distributed SQLite) or Postgres is realistic given SQLx's abstractions.

## Frontend exception

SvelteKit stays TypeScript, not Rust (Leptos, Dioxus). The Rust frontend ecosystem is not mature enough for a content-heavy site with SEO requirements, complex layouts, and a solo development pace. SvelteKit with generated types from `ludeme-core` is a clean boundary, not a compromise.
