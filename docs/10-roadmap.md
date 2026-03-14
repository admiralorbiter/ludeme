# Developer Roadmap

> **Roadmap framing:** Think in capability phases, not calendar promises. Each phase has an entry state, a set of deliverables, and an exit criterion. Do not start a phase until the previous phase's exit criterion is met.

---

## Phase 0 — Grammar and scaffold

**Goal:** Lock the content model, crate structure, and taxonomy before any UI or content work begins.

**Entry state:** This document exists and is agreed upon.

**Deliverables:**

- `ludeme-core` crate with all entity types, `GameEvent` enum, `GameState` trait, `ParamManifest`
- `ts-rs` generation pipeline from core types to TypeScript
- `taxonomy/` TOML files for mechanic families, relation types, and confidence levels
- Database migration for all entity tables and the relationship edges table
- `ludeme-server` skeleton: Axum app boots, connects to SQLite, seeds taxonomy on startup
- SvelteKit shell skeleton: routes exist, types imported from generated TS
- One sample slice described end-to-end in the templates without inventing new fields
- `fidelity-levels.toml` with decision criteria

**Exit criterion:** A mechanic, a work, a demo brief, and a relationship edge can all be created via the API and retrieved without inventing new fields. The full entity graph for one demo slice exists in the database.

**Progress:**

- [x] `ludeme-core` crate with entity types and `GameEvent` enum
- [x] `taxonomy/` TOML files for mechanic families, relation types, and fidelity levels
- [x] Database migration for all entity tables and relationship edges
- [x] `ludeme-server` skeleton: boots, connects to SQLite, seeds taxonomy
- [x] SvelteKit shell skeleton: routes exist, static types in `types.ts`
- [x] `fidelity-levels.toml` with decision criteria
- [x] CRUD API endpoints for mechanics, works, demos, and edges
- [x] One sample slice (Pong) existing end-to-end in the database
- [ ] `ts-rs` generation pipeline from core types to TypeScript (deferred — manual `types.ts` in sync)

---

## Phase 1 — Playable shell and demo core

**Goal:** Make the platform interactive before it becomes elaborate.

**Entry state:** Phase 0 exit criterion met.

**Deliverables:**

- First WASM demo compiled and served (`pong-76` or equivalent)
- Demo Shell API contract implemented: `SessionStart`, `SessionEnd`, `MomentEmit` events flowing from WASM to shell
- Unified play shell in SvelteKit: canvas embed, fidelity badge, control hints overlay
- Bookmark capture flow: shortcut → screenshot → moment form → saved `MomentBookmark`
- Basic collections: a curated trail of at least one ordered set of demos
- Home / Discover surface: browsable by mechanic family and era
- Search: full-text across demo titles, mechanic names, and work titles
- Publish state management: draft, review, public
- Release readiness checklist enforced on publish

**Exit criterion:** At least three demos can be launched through the same shell and metadata model. A moment bookmark can be created from inside play and retrieved via the API. The play-to-note flow (see [UX Flows](09-ux-flows.md)) works end-to-end.

**Progress:**

- [x] First WASM demo compiled and served (pong-76)
- [x] Home / Discover surface: browsable by mechanic family
- [x] Play shell route exists with canvas embed and state machine overlay
- [x] Session creation API endpoint
- [x] Bookmark creation API endpoint
- [ ] Demo Shell API contract: `SessionStart`, `SessionEnd`, `MomentEmit` events flowing from WASM to shell
- [ ] Bookmark capture flow: shortcut → screenshot → moment form → saved bookmark
- [ ] Collections: curated trails
- [ ] Search: full-text across demo titles, mechanic names, work titles
- [ ] Publish state management and release readiness checklist
- [ ] At least two more playable demos

---

## Phase 2 — Research-integrated product

**Goal:** Attach context directly to play. Make the platform more than a demo gallery.

**Entry state:** Phase 1 exit criterion met.

**Deliverables:**

- Mechanic pages: definition, verbs, state graph (static), example demos, lineage edges
- Work pages: historical context, notable constraints, related mechanics, linked demos
- Structured Observation creation from moment bookmarks and free-form notes
- Parameter tuner panel: shell renders sliders from `ParamManifest`, game reads params per frame
- `ParamChange` events logged to session record
- "Save as Experiment" flow from the tuner panel
- Basic Comparison entity: static screenshot comparison with dimension notes
- Shareable moment cards: screenshot + tags + deep link URL
- State machine overlay: `StateChange` events drive live state chip display
- Breadcrumb and entity chip navigation across demo → mechanic → work
- Mechanic-to-compare flow (see [UX Flows](09-ux-flows.md)) works end-to-end

**Exit criterion:** Every playable generates linked observations and every observation points back to evidence. A visitor can move from a demo to a mechanic page, to a comparison, and back to a playable in three jumps or fewer.

---

## Phase 3 — Comparison, sensemaking, and replay

**Goal:** Reveal evolution, branching, and resurfacing. Make the platform intellectually powerful.

**Entry state:** Phase 2 exit criterion met.

**Deliverables:**

- Dual-instance compare mode: two WASM instances side by side, input mirrored
- Live Compare Lab backed by dual-instance mode
- Deterministic replay: input log stored, replay playback in shell
- Seekable moments via `restore()` on supported demos
- Mechanic lineage graph: interactive graph view of typed relationship edges
- Timeline view: works and demos placed on a filterable era timeline
- Graph explorer: entity graph navigation with era and mechanic family filters
- Spatial heat map: aggregate `FrameTick` position data rendered as canvas overlay
- Shareable moment deep links (demo + branch + seed + frame)
- Saved comparison views with follow-up links

**Exit criterion:** At least three strong cross-era comparisons demonstrably work in the compare lab. The lineage graph for at least one mechanic family is navigable and accurate. A replay session can be scrubbed to any frame.

---

## Phase 4 — Authoring at scale and annotation

**Goal:** Reduce the cost of adding new demos and research. Introduce authored teaching artifacts.

**Entry state:** Phase 3 exit criterion met.

**Deliverables:**

- Studio forms: schema-driven forms for all entity types
- Validation rules: required fields enforced before publish, fidelity gate active
- Versioning: entity revisions tracked, previous states recoverable
- Review workflow: draft → review → public with reviewer notes
- Relationship editor: create and manage typed edges between any two entities
- Annotation layer on replay: draw on paused replay canvas, store timestamped annotations
- Annotation playback in session replay mode
- Curated trail builder: ordered item selection, compare prompts, exit understanding field
- Bulk import tooling (Python scripts in `tools/`) for batch work or mechanic entry

**Exit criterion:** Adding a new work, demo, and mechanic package is form-driven and repeatable without writing any custom page logic. An annotated replay is publishable as a teaching artifact.

---

## Phase 5 — Signature differentiators

**Goal:** Make Ludeme distinctively compelling. Features that a wiki plus a game launcher cannot do.

**Entry state:** Phase 4 exit criterion met.

**Deliverables:**

- Mechanic lens overlays: visual highlights of hitboxes, trajectories, and collision zones rendered on the live canvas
- Richer session analytics: completion rates, common moment-bookmark frames, param tuner usage patterns
- Semantic search over mechanic definitions and observations (embedding vectors stored as JSON, cosine similarity in app layer; Turso or Postgres if needed at scale)
- Recommendation logic: "you played this mechanic demo, here are related demos and open observations"
- Ghost comparison: overlay two input logs on the same replay to compare player behavior
- Expert trail authoring: a designated trail type where expert replay + annotations + notes form a structured lesson
- Parameter space mapping: given a mechanic with two exposed params, render a 2D grid of variant snapshots

**Exit criterion:** The platform demonstrably does something a wiki plus a game launcher cannot replicate. At least one feature in this phase is used by a non-author visitor to discover something they would not have found through browsing alone.

---

## Proving slices (recommended first content)

| Slice | Mechanic family | What it proves |
|---|---|---|
| Ball dynamics | Collision response, scoring pressure | Shared shell, mechanic tagging, param tuner, basic compare |
| Maze and chase | AI behavior, state transitions | Observation capture, state overlay, lineage notes |
| Jump feel | Movement, timing windows | Dual-instance compare, parameter space, mechanic pages |

These are capability choices, not hard content requirements. Substitute any example that proves the same system behavior.
