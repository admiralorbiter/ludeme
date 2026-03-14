# Decision Log

A running record of significant product and technical decisions. Add an entry when a tradeoff is made, a direction changes, or a previously open question is settled. Update this file when decisions change, not just when they are first made.

---

## 2026-03-12 — Full Rust backend over Python + Rust split

**Context:** Evaluating whether to use a Python API layer (FastAPI) alongside Rust demo crates, or go full Rust for the backend.

**Decision:** Full Rust — `ludeme-core` as the shared types crate, `ludeme-server` with Axum, SQLx compile-time query checking.

**Rationale:** The primary driver is the shared types crate. `Mechanic`, `Work`, `PlayableDemo`, `GameEvent`, etc. live in `ludeme-core` and are imported by both the backend and every demo crate. TypeScript types are generated from the same source via `ts-rs`. In a Python + Rust split the types are defined twice and schema drift becomes inevitable. SQLx compile-time query verification and single binary deployment are secondary benefits.

**Trade-off acknowledged:** Higher boilerplate upfront. Accepted as front-loaded cost.

**Reference:** docs/02-tech-stack.md — "Why full Rust over Python + Rust"

---

## 2026-03-12 — SvelteKit for the frontend shell, not Leptos/Dioxus

**Context:** Rust's frontend WASM frameworks (Leptos, Dioxus) are maturing but not production-ready for content-heavy sites with SEO requirements and complex layouts.

**Decision:** SvelteKit with TypeScript. Types generated from `ludeme-core` via `ts-rs` create a clean boundary without requiring Rust on the frontend.

**Trade-off acknowledged:** Shell is not fully Rust. Accepted — the shell API contract (`window.__ludeme`) is the explicit boundary between game code and platform code.

**Reference:** docs/02-tech-stack.md — "Frontend exception"
