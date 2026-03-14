# Crate Architecture

## Workspace layout

```
ludeme/
├── Cargo.toml                  # workspace root
├── crates/
│   ├── ludeme-core/            # shared entity types, event protocol, relation vocabulary
│   ├── ludeme-server/          # Axum backend, SQLx, auth, API routes
│   ├── ludeme-macros/          # optional: derive macros for param manifests, state emission
│   └── ludeme-demos/
│       ├── pong-76/            # one crate per demo
│       ├── maze-80/
│       └── jump-feel/
├── shell/                      # SvelteKit frontend
│   ├── src/
│   └── package.json
├── migrations/                 # SQLx migration files
├── taxonomy/                   # TOML taxonomy files (mechanic families, relation types, fidelity)
│   ├── mechanic-families.toml
│   ├── relation-types.toml
│   └── fidelity-levels.toml
└── tools/                      # Python scripts for bulk import, one-off operations
```

## ludeme-core responsibilities

This crate is the system grammar. It must not import anything from the server or any demo crate.

- All entity structs (`Mechanic`, `Work`, `PlayableDemo`, `Observation`, `RelationshipEdge`, `MomentBookmark`, `Experiment`, `Collection`, `Source`)
- All event types used in the shell API contract (`GameEvent`, `SessionEvent`, `ParamManifest`)
- The `GameState` trait (`snapshot()` and optional `restore()`)
- The `FidelityLevel` enum with documented criteria
- Taxonomy type definitions (the TOML files bind to these at startup)
- `ts-rs` derive attributes on all types that cross the shell boundary

## ludeme-server responsibilities

- Axum route handlers for all entity CRUD
- SQLx queries against SQLite
- Relationship edge creation and traversal
- Search endpoint (full-text via FTS5; JSON vector similarity later)
- Session and moment storage
- Static file serving for compiled WASM demo binaries
- Publish state management and validation rules
- Auth (start with a simple token for solo authoring; expand later)

## Demo crate responsibilities

Each demo crate compiles to a WASM binary that the shell embeds. A demo crate:

- Imports `ludeme-core` for shared event types and the `GameState` trait
- Implements `GameState::snapshot()` (and optionally `restore()`)
- Declares a `ParamManifest` at session start: the named parameters the shell can tune
- Emits `GameEvent`s via the shell API contract (see [Shell API Contract](04-shell-api-contract.md))
- Does not know about the database, the server, or any other demo
- Uses macroquad for rendering and input

### Demo WASM build target

```toml
# In each demo's Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ludeme-core = { path = "../../crates/ludeme-core" }
macroquad = "0.4"
wasm-bindgen = "0.2"
```
