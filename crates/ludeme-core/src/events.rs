// ludeme-core/src/events.rs
//
// The shell API contract event vocabulary. These types are serialized as JSON
// and passed between the WASM game binary and the SvelteKit shell.
// See docs/04-shell-api-contract.md for the full protocol specification.

use serde::{Deserialize, Serialize};

/// Top-level event enum — all events emitted by a demo crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum GameEvent {
    SessionStart(SessionStart),
    SessionEnd(SessionEnd),
    MomentEmit(MomentEmit),
    StateChange(StateChange),
    ParamChange(ParamChange),
    BranchChange(BranchChange),
    /// Optional — only emitted by demos that opt into heat map tracing.
    FrameTick(FrameTick),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStart {
    pub demo_id: String,
    pub branch_id: String,
    pub seed: u64,
    pub param_manifest: ParamManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEnd {
    pub frame_count: u64,
    pub duration_ms: u64,
    pub input_log_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentEmit {
    pub scene_id: String,
    pub frame: u64,
    /// Serialized output of `GameState::snapshot()`.
    pub state_blob: Option<Vec<u8>>,
    pub player_label: Option<String>,
    /// Mechanic tags suggested by the game.
    pub auto_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub from_state: String,
    pub to_state: String,
    pub frame: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamChange {
    pub key: String,
    pub value: f32,
    pub frame: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchChange {
    pub from_branch: String,
    pub to_branch: String,
}

/// Optional — emitted only by demos that opt into spatial heat map tracing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameTick {
    pub frame: u64,
    pub position: Option<[f32; 2]>,
    pub active_states: Vec<String>,
}

// ---------------------------------------------------------------------------
// Parameter manifest — declared at SessionStart, drives the tuner panel
// ---------------------------------------------------------------------------

/// Declared at `SessionStart`. The shell renders sliders/toggles from this.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamManifest {
    pub params: Vec<ParamDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDescriptor {
    pub key: String,
    pub label: String,
    pub kind: ParamKind,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    /// Optional grouping label for organizing sliders into sections.
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamKind {
    Float,
    Integer,
    /// Rendered as a checkbox; stored as 0.0 / 1.0.
    Toggle,
}
