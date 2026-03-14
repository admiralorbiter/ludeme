// ludeme-core/src/entities.rs
//
// All domain entity structs. These are the first-class objects in the Ludeme
// data model. See docs/06-domain-model.md for field details.
//
// All types that cross the shell boundary derive `TS` for TypeScript generation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A rule, verb, pattern, or feel layer that can appear across multiple works.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mechanic {
    pub id: Uuid,
    pub name: String,
    pub family: String,
    pub short_definition: Option<String>,
    pub verbs: Vec<String>,
    pub failure_pattern: Option<String>,
    pub mastery_pattern: Option<String>,
    pub publish_state: PublishState,
}

/// A historical game, reference title, or notable design artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: Uuid,
    pub title: String,
    pub year: Option<i32>,
    pub platform: Option<String>,
    pub genre: Option<String>,
    pub significance: Option<String>,
    pub notable_constraints: Option<String>,
    pub publish_state: PublishState,
}

/// A tiny interactive build focused on one historical slice or mechanic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayableDemo {
    pub id: Uuid,
    pub title: String,
    pub linked_work: Option<Uuid>,
    pub mechanic_tags: Vec<String>,
    pub fidelity_level: crate::taxonomy::FidelityLevel,
    pub branch_id: String,
    pub wasm_path: Option<String>,
    pub publish_state: PublishState,
}

/// A structured research note or insight with evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub claim: String,
    pub evidence_links: Vec<Uuid>,
    pub linked_entities: Vec<Uuid>,
    pub confidence: crate::taxonomy::Confidence,
    pub why_it_matters: Option<String>,
    pub follow_up_question: Option<String>,
    pub publish_state: PublishState,
}

/// A fork of a demo made to test a hypothesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: Uuid,
    pub parent_demo: Uuid,
    pub hypothesis: String,
    pub expected_effect: Option<String>,
    pub observed_result: Option<String>,
    pub decision: Option<ExperimentDecision>,
    pub publish_state: PublishState,
}

/// A saved side-by-side view or analytic question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub id: Uuid,
    pub items: Vec<Uuid>,
    pub prompts: Vec<String>,
    pub conclusion: Option<String>,
    pub publish_state: PublishState,
}

/// A curated learning path or thematic grouping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub title: String,
    pub learning_goal: Option<String>,
    pub ordered_items: Vec<Uuid>,
    pub publish_state: PublishState,
}

/// A citation, interview, footage, or reference material.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,
    pub origin: String,
    pub relevance: Option<String>,
    pub trust_level: crate::taxonomy::Confidence,
    pub citation_notes: Option<String>,
    pub linked_entities: Vec<Uuid>,
}

/// A typed link between two entities — the core graph primitive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEdge {
    pub id: Uuid,
    pub from_id: Uuid,
    pub from_type: String,
    pub to_id: Uuid,
    pub to_type: String,
    pub relation_type: String,
    pub confidence: crate::taxonomy::Confidence,
    pub note: Option<String>,
}

/// A recorded play session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub demo_id: Uuid,
    pub branch_id: Option<String>,
    pub seed: u64,
    pub frame_count: Option<u64>,
    pub duration_ms: Option<u64>,
}

/// A specific frame, state, or scene captured from play.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentBookmark {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub demo_id: Uuid,
    pub frame: u64,
    pub player_label: Option<String>,
    pub auto_tags: Vec<String>,
    pub screenshot_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Shared enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PublishState {
    #[default]
    Draft,
    Review,
    Public,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentDecision {
    Keep,
    Discard,
    Revisit,
}
