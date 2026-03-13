# Systems Catalog

Each system below is a named, designed subsystem. Each has a description, the entities or events it depends on, the implementation approach, and the phase it targets.

---

## 5.1 Unified play shell

**What it does:** A consistent outer wrapper for every demo. Handles launch, controls overlay, fidelity badge display, related-item strip, and the bookmark/note capture panel. Every demo runs inside this shell regardless of mechanic family or era.

**Depends on:** `SessionStart` event (for param manifest), `SessionEnd`, all `GameEvent` types for routing to the right shell panel.

**Implementation:** A SvelteKit layout component wraps a `<canvas>` element. The WASM binary is loaded into the canvas via a dynamic import. The shell listens on `window.__ludeme.onEvent`. Shell panels (param tuner, state display, bookmark form, related-items strip) are driven by events, not hard-coded per demo.

**Phase:** 1

---

## 5.2 Parameter tuner / mechanic tuner

**What it does:** A sidebar panel the shell renders from the demo's `ParamManifest`. Sliders and toggles let the visitor modify named game parameters (gravity, ball speed, enemy reaction time, paddle size) in real time without recompiling.

**Depends on:** `ParamManifest` from `SessionStart`, `ParamChange` event emitted back into the game.

**Implementation:** The shell renders a panel from the manifest. On slider change, it calls `window.__ludeme_set_param(key, value)` which the WASM exposes via wasm-bindgen. The game reads the current param value from a shared `Params` struct on each frame. Param changes are logged to the session record automatically. When a visitor saves an interesting param configuration, it becomes an experiment record with the changed variables pre-populated.

**Design note:** This is the experiment system made tactile instead of bureaucratic. The hypothesis document becomes an optional next step, not a prerequisite.

**Phase:** 2

---

## 5.3 State machine overlay

**What it does:** Makes hidden discrete game states visible while playing. For mechanics with clear state machines (grounded, jumping, falling, coyote-time, wall-slide, charging), the game emits `StateChange` events. The shell renders a live state chip next to the player and optionally a state transition diagram.

**Depends on:** `StateChange` events from the game, a static state graph defined in the demo's metadata.

**Implementation:** The demo declares its state graph in its metadata (as a list of states and allowed transitions). The shell renders this graph statically on the mechanic page. During live play, `StateChange` events highlight the current node. The static diagram is always available; the live animation requires an active session.

**Why it matters:** A platform built on making hidden rules visible. No wiki or video shows a state machine animating in response to live play. This is a signature teaching capability.

**Phase:** 2 (static graph), Phase 3 (live animation)

---

## 5.4 Dual-instance compare mode

**What it does:** Two instances of the same WASM demo run side by side, both receiving the same input, but with different parameter sets. The visitor experiences the difference between two jump arcs, two gravity values, or two scoring pressures simultaneously.

**Depends on:** The shell's ability to instantiate two WASM modules on the same page. Input mirroring from one input handler to both. Param sets stored as named variants.

**Implementation:** SvelteKit component renders two canvases side by side. A single `InputBroker` captures keyboard/gamepad input and fans it out to both WASM instances. Each instance receives its own `SessionStart` with its own param set. A divider with labels names the two variants. Output of both sessions is recorded and linked to a Comparison entity automatically.

**Constraints:** Requires demos to be stateless between instantiations (no global mutable state outside of the game struct). Most macroquad demos satisfy this naturally.

**Phase:** 3

---

## 5.5 Deterministic replay and session scrubbing

**What it does:** Any session with an input log can be replayed frame-exactly. The shell can scrub to any frame, pause, and resume. A replayed session at a paused frame is equivalent to a seekable moment bookmark.

**Depends on:** Fixed timestep game loop, seeded randomness, input log stored from `SessionStart`, `restore()` implemented on `GameState`.

**Implementation:** The shell stores the input log as a compact binary sequence (frame number + action bitmask). Replay mode feeds the log back at real time or at a configurable rate. Scrubbing to frame N requires either replaying from frame 0 (cheap for short demos) or restoring from the nearest snapshot (requires periodic `snapshot()` calls during record, stored as keyframes every N frames).

**Phase:** 3

---

## 5.6 Annotation layer on replay

**What it does:** During replay playback, an author can pause and draw on the canvas — circle a hitbox, trace a trajectory, annotate a timing window. Annotations are stored as timestamped overlay data and played back with the session.

**Depends on:** Replay system (5.5). An SVG overlay layer on top of the WASM canvas.

**Implementation:** A transparent SVG element sits above the canvas. When in annotation mode, mouse events draw to the SVG instead of feeding the game. Each stroke is stored as `{ frame, svg_path, label }`. On playback, the SVG layer renders the matching strokes as they arrive in frame order. Annotations become a first-class part of a teaching artifact.

**Why it matters:** The closest thing to a "game film breakdown" the platform can offer. Turns a raw session replay into an authored observation.

**Phase:** 4

---

## 5.7 Spatial heat map / attention trace

**What it does:** Aggregates player position data from `FrameTick` events across multiple sessions and renders a heat map overlaid on the game canvas. Shows where players spend time, where they die, and where a mechanic creates pressure or confusion.

**Depends on:** `FrameTick` events with position data. Multiple stored sessions against the same demo. A canvas overlay renderer.

**Implementation:** `FrameTick` events are batched and stored server-side per session. A heat map endpoint aggregates positions across all sessions for a given demo and returns a density grid. The shell renders this as a color-ramp overlay on a static canvas screenshot. This is opt-in per demo (some mechanics aren't spatial). Controlled via a flag in the demo's metadata.

**Research value:** Turns user behavior into evidence. "Players consistently overcorrect on the first exposure to screen wrap" becomes a claim supportable with data from real play sessions, which can link directly to an Observation entity.

**Phase:** 4

---

## 5.8 Shareable moment cards

**What it does:** A moment bookmark rendered as a shareable card — small canvas capture, mechanic tags, player note, and a deep link back to the demo at that exact state (or the closest replayable frame).

**Depends on:** MomentEmit events, canvas screenshot capability (browser native), the state blob or input log for the deep link.

**Implementation:** On moment capture, the shell takes a `canvas.toDataURL()` screenshot. This plus the mechanic tags, player label, and a generated URL (encoding demo ID, branch, seed, frame) are assembled into a card. The card is rendered as a static HTML/OG image for sharing and as a stored `MomentBookmark` entity in the database. The deep link opens the demo in replay mode at the bookmarked frame.

**Phase:** 2 (basic card), Phase 3 (deep link with replay)

---

## 5.9 Compare lab

**What it does:** A saved side-by-side view of two or more demo implementations, parameter variants, or eras. Supports comparison prompts, annotated dimensions, and conclusion notes.

**Depends on:** Playable Demo entities, MomentBookmark entities, the Comparison entity type, dual-instance mode (5.4) for live comparisons.

**Implementation:** A dedicated Compare Lab surface in the shell. Users select items to compare (demos, moments, parameter snapshots). The surface shows static screenshots or live dual-instance play depending on the comparison type. Dimensions and prompts are authored as part of the Comparison entity. Conclusions become Observation entities linked to the comparison.

**Phase:** 2 (static comparison with screenshots), Phase 3 (live dual-instance)

---

## 5.10 Mechanic lineage graph

**What it does:** A visual graph of a mechanic's evolution — earliest known examples, variants, descendants, things it replaced, things it was discarded by. Navigable and filterable by era or platform.

**Depends on:** RelationshipEdge entities with typed relations, Work entities, the canonical relation vocabulary.

**Implementation:** Server-side graph traversal from a mechanic node to N hops of typed edges. Shell renders as an interactive force-directed or timeline graph (D3 or a lightweight canvas renderer). Edge types are colored by relation type. Nodes are clickable to navigate to the Work or Demo page. Filtered by era or platform to reduce noise.

**Phase:** 3
