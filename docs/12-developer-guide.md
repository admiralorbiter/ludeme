# Developer Guide

Practical commands for working on Ludeme day-to-day. Keep this updated as the stack evolves.

---

## Prerequisites

| Tool | Install | Required for |
|---|---|---|
| Rust (stable) | `rustup` | Everything Rust |
| `wasm32-unknown-unknown` target | Auto-installed by `build-demo.ps1` | Building demo crates |
| `wasm-bindgen-cli` | Auto-installed by `build-demo.ps1` | Generating JS glue for WASM |
| `sqlx-cli` | `cargo install sqlx-cli --no-default-features --features sqlite,rustls` | Running migrations |
| Node.js ≥ 20 | nodejs.org | SvelteKit shell |


---

## Setup (first time)

```powershell
# 1. Clone and enter
git clone https://github.com/admiralorbiter/ludeme
cd ludeme

# 2. Install Node root deps (concurrently etc.)
npm install

# 3. Install shell deps
npm install --prefix shell

# 4. Create your local .env
Copy-Item .env.example .env
# Edit DATABASE_URL if needed — default is sqlite:ludeme.db?mode=rwc

# 5. Run migrations (creates ludeme.db automatically)
$env:DATABASE_URL="sqlite://ludeme.db?mode=rwc"; sqlx migrate run

# 6. Start dev servers
npm run dev
```

---

## Daily development

```powershell
# Start both backend + frontend (concurrently)
npm run dev

# Backend only
cargo run -p ludeme-server

# Frontend only
npm run dev --prefix shell
```

When both are running:
- Shell: **http://localhost:5173**
- API: **http://localhost:3000**
- Health: **http://localhost:3000/health**

API calls from the shell use the `/api` prefix and are proxied automatically by Vite — no CORS issues.

---

## Database

```powershell
# Run all pending migrations
$env:DATABASE_URL="sqlite://ludeme.db?mode=rwc"
sqlx migrate run

# Roll back the last migration
sqlx migrate revert

# Create a new migration file
sqlx migrate add <description>
# → creates migrations/NNNN_description.sql

# Inspect the live DB (SQLite CLI, if installed)
sqlite3 ludeme.db
.tables
.schema mechanics
```

> **Note:** `ludeme.db` is created automatically by SQLite when the server connects. Never commit it — it's in `.gitignore`.

The server runs `sqlx::migrate!()` on startup, so after creating new migrations you only need to restart the server — `sqlx migrate run` is not required for normal development.

**Migrations:**
- `0001_initial.sql` — Core schema (all entity tables, taxonomy, sessions, bookmarks)
- `0002_fts5_search.sql` — FTS5 full-text search index (demos, mechanics, works)

---

## Building a demo crate (WASM)

```powershell
# Build and package a demo crate for the shell
# Run from the project root
.\build-demo.ps1 pong-76

# This:
# 1. Runs: cargo build -p pong-76 --target wasm32-unknown-unknown --release
# 2. Runs: wasm-bindgen ... --out-dir shell/static/demos/pong-76 --target web
# 3. Reports output file sizes
```

After building, set `wasm_path` in `+page.server.ts` for that demo:
```ts
wasm_path: '/demos/pong-76/pong_76.wasm',
```

Compiled WASM files live in `shell/static/demos/<id>/` and are gitignored.

---

## Adding a demo crate

```powershell
# 1. Create the crate directory
New-Item -ItemType Directory -Path crates/ludeme-demos/pong-76/src

# 2. Add Cargo.toml (see docs/03-crate-architecture.md for the template)
# 3. Add pong-76 to workspace Cargo.toml members list
# 4. Write the game in src/lib.rs
# 5. Build: .\build-demo.ps1 pong-76
```

---

## Adding taxonomy entries

Taxonomy is defined in `taxonomy/*.toml` files — just add an entry and restart the server. The seed function runs `INSERT OR IGNORE` on every boot, so new entries are picked up automatically.

```powershell
# After editing taxonomy/*.toml, restart the server to seed:
# (If npm run dev is running, stop it, then:)
npm run dev
# Watch the [api] log for "Taxonomy: N families seeded"
```

---

## Cargo workspace

```powershell
# Check all crates compile
cargo check --workspace

# Check a specific crate
cargo check -p ludeme-server
cargo check -p ludeme-core

# Run the server directly (without the shell)
cargo run -p ludeme-server
```

---

## Type generation (ts-rs) — Phase 1

> Not yet wired. Currently `shell/src/lib/types.ts` is maintained manually.

When wired:
```powershell
# Generate TypeScript types from Rust structs
cargo test -p ludeme-core --features ts-rs -- export_types
# → writes to bindings/ directory
# → copy to shell/src/lib/types.ts or configure ts-rs output path
```

---

## Project structure

```
ludeme/
├── crates/
│   ├── ludeme-core/       # Shared types: entities, events, state, taxonomy
│   ├── ludeme-server/     # Axum API server (db, routes, seed modules)
│   ├── ludeme-macros/     # Proc-macro stub (Phase 2)
│   └── ludeme-demos/      # One subdirectory per playable demo crate
├── shell/                 # SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── types.ts          # TS types (mirrors ludeme-core)
│   │   │   ├── session.svelte.ts # Svelte 5 session store
│   │   │   └── ludeme-shell.ts   # window.__ludeme bridge + WASM loader
│   │   └── routes/
│   │       ├── +layout.svelte    # Nav + app shell
│   │       ├── +page.svelte      # Discover/home page
│   │       ├── +error.svelte     # Error boundary
│   │       ├── demo/[id]/        # Demo play shell
│   │       ├── mechanics/        # Stub (Phase 2)
│   │       ├── works/            # Stub (Phase 2)
│   │       └── collections/      # Collection list + detail (curated trails)
│   └── static/
│       └── demos/                # Compiled WASM outputs (gitignored)
├── migrations/            # SQLx SQLite migrations
├── taxonomy/              # mechanic-families, relation-types, fidelity-levels
├── docs/                  # This documentation
├── build-demo.ps1         # WASM build script
├── .env.example           # Environment variable template
├── DECISION-LOG.md        # Architecture decision record
└── package.json           # Root: runs both servers with `npm run dev`
```
