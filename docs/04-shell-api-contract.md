# Demo Shell API Contract

This is the protocol between a compiled WASM game binary and the SvelteKit outer shell. Both sides must implement this contract. Neither side should know about the other's internals.

## Communication mechanism

The WASM binary uses `wasm-bindgen` to call into a JS object the shell registers at `window.__ludeme`. The shell registers the object before the WASM module initializes. The game calls named methods; the shell responds synchronously or asynchronously.

```typescript
// Shell registers this object before WASM init
window.__ludeme = {
  onEvent: (eventJson: string) => void,
  getParam: (key: string) => number | null,
}
```

The game emits events by calling `ludeme_emit(json_string)` from Rust, which maps to `window.__ludeme.onEvent` via wasm-bindgen glue.

## Event vocabulary

All events are serialized as JSON matching the `GameEvent` enum in `ludeme-core`.

### Session events

```rust
// ludeme-core/src/events.rs

pub enum GameEvent {
    SessionStart(SessionStart),
    SessionEnd(SessionEnd),
    MomentEmit(MomentEmit),
    StateChange(StateChange),
    ParamChange(ParamChange),
    BranchChange(BranchChange),
    FrameTick(FrameTick),     // optional, for heat map traces
}

pub struct SessionStart {
    pub demo_id: String,
    pub branch_id: String,
    pub seed: u64,
    pub param_manifest: ParamManifest,
}

pub struct SessionEnd {
    pub frame_count: u64,
    pub duration_ms: u64,
    pub input_log_available: bool,
}

pub struct MomentEmit {
    pub scene_id: String,
    pub frame: u64,
    pub state_blob: Option<Vec<u8>>,  // GameState::snapshot() output
    pub player_label: Option<String>,
    pub auto_tags: Vec<String>,       // mechanic tags the game suggests
}

pub struct StateChange {
    pub from_state: String,
    pub to_state: String,
    pub frame: u64,
}

pub struct ParamChange {
    pub key: String,
    pub value: f32,
    pub frame: u64,
}

pub struct BranchChange {
    pub from_branch: String,
    pub to_branch: String,
}

pub struct FrameTick {
    pub frame: u64,
    pub position: Option<[f32; 2]>,  // for spatial heat map traces
    pub active_states: Vec<String>,
}
```

### Parameter manifest

Declared at `SessionStart`. The shell uses this to render the parameter tuner panel.

```rust
pub struct ParamManifest {
    pub params: Vec<ParamDescriptor>,
}

pub struct ParamDescriptor {
    pub key: String,
    pub label: String,
    pub kind: ParamKind,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub group: Option<String>,   // for organizing sliders into sections
}

pub enum ParamKind {
    Float,
    Integer,
    Toggle,   // rendered as a checkbox, stored as 0.0 / 1.0
}
```

## GameState trait

```rust
// ludeme-core/src/state.rs

pub trait GameState {
    /// Serialize current world state to a byte blob.
    /// Required for moment bookmarks to carry restorable state.
    fn snapshot(&self) -> Vec<u8>;

    /// Restore world state from a blob. Optional.
    /// Required only if the demo supports seekable moments or replay.
    fn restore(&mut self, blob: &[u8]) -> Result<(), StateError>;
}
```

Implement `snapshot()` on every demo. Implement `restore()` only on demos that support seekable moments. The shell gracefully degrades: if `restore()` is not available, moment bookmarks are metadata-only.

## Input recording (deterministic replay foundation)

For demos that implement a fixed timestep, input recording is available at low cost. The shell records all input events with frame numbers. At replay, it feeds the same input sequence from the same seed. Requirements the demo must meet:

- Fixed timestep game loop (macroquad supports this)
- Seed passed in at `SessionStart` and used for all randomness
- No external time calls inside game logic (use frame count instead)

Input log is stored alongside the session record. Replay is a Phase 3 feature but the game-side requirements must be baked in from the first demo.
