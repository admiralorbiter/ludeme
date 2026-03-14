// ludeme-core/src/state.rs
//
// The GameState trait — the contract every demo crate implements.
// See docs/04-shell-api-contract.md for details.

/// Error returned by `GameState::restore()`.
#[derive(Debug)]
pub struct StateError(pub String);

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StateError: {}", self.0)
    }
}

impl std::error::Error for StateError {}

/// Implemented by every demo crate.
///
/// - `snapshot()` is required — enables moment bookmarks to carry restorable state.
/// - `restore()` is optional — enables seekable moments and deterministic replay.
///   The shell degrades gracefully if `restore()` is not implemented (metadata-only bookmarks).
pub trait GameState {
    /// Serialize current world state to a byte blob.
    fn snapshot(&self) -> Vec<u8>;

    /// Restore world state from a blob.
    ///
    /// Only implement this if the demo supports seekable moments or replay.
    fn restore(&mut self, blob: &[u8]) -> Result<(), StateError>;
}
