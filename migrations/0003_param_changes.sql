-- =============================================================================
-- Migration 0003 — Param change log
-- Ludeme / SQLite
--
-- Records every parameter change during a play session.
-- Each row = one slider adjustment (key, old value, new value, frame).
-- =============================================================================

CREATE TABLE IF NOT EXISTS param_changes (
    id           TEXT PRIMARY KEY,
    session_id   TEXT REFERENCES sessions(id),
    demo_id      TEXT NOT NULL REFERENCES playable_demos(id),
    frame        INTEGER NOT NULL,
    param_key    TEXT NOT NULL,
    old_value    REAL,
    new_value    REAL NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_paramchanges_session ON param_changes (session_id);
CREATE INDEX IF NOT EXISTS idx_paramchanges_demo    ON param_changes (demo_id);
