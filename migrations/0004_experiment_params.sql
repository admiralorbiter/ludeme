-- =============================================================================
-- Migration 0004 — Add param_snapshot to experiments
-- Ludeme / SQLite
--
-- Stores the current parameter values at the time an experiment is saved.
-- JSON object: { "gravity": 0.55, "jump_force": -8.0, ... }
-- =============================================================================

ALTER TABLE experiments ADD COLUMN param_snapshot TEXT;
