// ludeme-core/src/taxonomy.rs
//
// Taxonomy type definitions. The actual values live in taxonomy/*.toml files
// and are seeded into the database at server startup.
// See docs/08-taxonomy.md for the full taxonomy specification.

use serde::{Deserialize, Serialize};

/// Fidelity level carried by every published demo.
///
/// Not a quality judgment — a transparency signal about what the demo claims.
/// See docs/07-fidelity-spec.md for full criteria and examples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FidelityLevel {
    /// Good-faith match of original rules, timing, and feel.
    Faithful,
    /// Based on the original but with at least one documented departure.
    #[default]
    Interpreted,
    /// Mechanic as inspiration, not source. Tests a hypothesis or variant.
    Experimental,
}

/// Confidence level used on Observations, RelationshipEdges, and Sources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// Educated guess. Do not cite as settled.
    Speculative,
    /// Reasonable inference from limited evidence.
    #[default]
    Tentative,
    /// Backed by at least one credible source or reproducible observation.
    Supported,
    /// Multiple independent sources agree. Safe as a foundation claim.
    Established,
}
