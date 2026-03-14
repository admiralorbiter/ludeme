-- =============================================================================
-- Migration 0001 — Initial schema
-- Ludeme / SQLite
--
-- SQLite translation notes vs the Postgres sketch in docs/06-domain-model.md:
--   UUID        → TEXT (stored as hyphenated string, generated in app layer)
--   TIMESTAMPTZ → TEXT (ISO-8601, e.g. "2026-03-14T00:00:00Z")
--   JSONB       → TEXT (serialized JSON, parsed in app layer)
--   TEXT[]      → TEXT (JSON array, e.g. '["a","b"]')
--   BYTEA       → BLOB
--   tsvector    → FTS5 virtual table (see bottom of file)
-- =============================================================================

PRAGMA foreign_keys = ON;
-- NOTE: WAL mode is set at connection time in ludeme-server, not in migrations.
-- sqlx wraps migrations in a transaction; PRAGMA journal_mode cannot change inside one.

-- =============================================================================
-- Taxonomy tables (seeded from taxonomy/*.toml on startup)
-- =============================================================================

CREATE TABLE IF NOT EXISTS mechanic_families (
    slug        TEXT PRIMARY KEY,
    label       TEXT NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS relation_types (
    slug        TEXT PRIMARY KEY,
    label       TEXT NOT NULL,
    description TEXT,
    directed    INTEGER NOT NULL DEFAULT 1  -- boolean: 1=directed, 0=undirected
);

CREATE TABLE IF NOT EXISTS fidelity_levels (
    slug                         TEXT PRIMARY KEY,
    label                        TEXT NOT NULL,
    description                  TEXT,
    requires_notable_interpretations INTEGER NOT NULL DEFAULT 0,
    requires_hypothesis              INTEGER NOT NULL DEFAULT 0
);

-- =============================================================================
-- Core entity tables
-- All have: id TEXT PK, publish_state, created_at, updated_at
-- =============================================================================

CREATE TABLE IF NOT EXISTS mechanics (
    id               TEXT PRIMARY KEY,
    name             TEXT NOT NULL UNIQUE,
    family           TEXT NOT NULL REFERENCES mechanic_families(slug),
    short_definition TEXT,
    verbs            TEXT NOT NULL DEFAULT '[]',  -- JSON array of strings
    failure_pattern  TEXT,
    mastery_pattern  TEXT,
    state_graph      TEXT,                        -- JSON: { states: [...], transitions: [...] }
    publish_state    TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS works (
    id                   TEXT PRIMARY KEY,
    title                TEXT NOT NULL,
    year                 INTEGER,
    platform             TEXT,
    genre                TEXT,
    significance         TEXT,
    notable_constraints  TEXT,
    publish_state        TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at           TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at           TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS playable_demos (
    id                        TEXT PRIMARY KEY,
    title                     TEXT NOT NULL,
    linked_work               TEXT REFERENCES works(id),
    mechanic_tags             TEXT NOT NULL DEFAULT '[]',  -- JSON array of mechanic slugs
    fidelity_level            TEXT NOT NULL DEFAULT 'interpreted'
                                  REFERENCES fidelity_levels(slug),
    branch_id                 TEXT NOT NULL DEFAULT 'main',
    wasm_path                 TEXT,
    param_manifest            TEXT,          -- JSON: ParamManifest
    state_graph               TEXT,          -- JSON: StateGraph
    description               TEXT,
    era                       TEXT,
    platform                  TEXT,
    notable_interpretations   TEXT,          -- JSON array of strings
    hypothesis                TEXT,
    publish_state             TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at                TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at                TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS observations (
    id                TEXT PRIMARY KEY,
    claim             TEXT NOT NULL,
    evidence_links    TEXT NOT NULL DEFAULT '[]',  -- JSON array of source UUIDs
    linked_entities   TEXT NOT NULL DEFAULT '[]',  -- JSON array of { id, type }
    confidence        TEXT NOT NULL DEFAULT 'tentative'
                          CHECK (confidence IN ('speculative','tentative','supported','established')),
    why_it_matters    TEXT,
    follow_up_question TEXT,
    publish_state     TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at        TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at        TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS experiments (
    id               TEXT PRIMARY KEY,
    parent_demo      TEXT NOT NULL REFERENCES playable_demos(id),
    hypothesis       TEXT NOT NULL,
    expected_effect  TEXT,
    observed_result  TEXT,
    decision         TEXT CHECK (decision IN ('keep','discard','revisit', NULL)),
    publish_state    TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS comparisons (
    id           TEXT PRIMARY KEY,
    items        TEXT NOT NULL DEFAULT '[]',  -- JSON array of UUIDs
    prompts      TEXT NOT NULL DEFAULT '[]',  -- JSON array of strings
    conclusion   TEXT,
    publish_state TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS collections (
    title          TEXT NOT NULL,
    id             TEXT PRIMARY KEY,
    learning_goal  TEXT,
    ordered_items  TEXT NOT NULL DEFAULT '[]',  -- JSON array of UUIDs
    publish_state  TEXT NOT NULL DEFAULT 'draft' CHECK (publish_state IN ('draft','review','public')),
    created_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS sources (
    id              TEXT PRIMARY KEY,
    origin          TEXT NOT NULL,
    relevance       TEXT,
    trust_level     TEXT NOT NULL DEFAULT 'tentative'
                        CHECK (trust_level IN ('speculative','tentative','supported','established')),
    citation_notes  TEXT,
    linked_entities TEXT NOT NULL DEFAULT '[]',  -- JSON array of { id, type }
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- =============================================================================
-- Relationship graph — typed edges between any two entities
-- =============================================================================

CREATE TABLE IF NOT EXISTS relationship_edges (
    id            TEXT PRIMARY KEY,
    from_id       TEXT NOT NULL,
    from_type     TEXT NOT NULL,  -- 'mechanic' | 'work' | 'demo' | 'observation' | ...
    to_id         TEXT NOT NULL,
    to_type       TEXT NOT NULL,
    relation_type TEXT NOT NULL REFERENCES relation_types(slug),
    confidence    TEXT NOT NULL DEFAULT 'tentative'
                      CHECK (confidence IN ('speculative','tentative','supported','established')),
    note          TEXT,
    created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_edges_from ON relationship_edges (from_id, relation_type);
CREATE INDEX IF NOT EXISTS idx_edges_to   ON relationship_edges (to_id,   relation_type);

-- =============================================================================
-- Sessions and moment bookmarks
-- =============================================================================

CREATE TABLE IF NOT EXISTS sessions (
    id              TEXT PRIMARY KEY,
    demo_id         TEXT NOT NULL REFERENCES playable_demos(id),
    branch_id       TEXT,
    seed            INTEGER NOT NULL,
    input_log       BLOB,
    frame_ticks     TEXT,       -- JSON array of FrameTick (optional heat map data)
    frame_count     INTEGER,
    duration_ms     INTEGER,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_sessions_demo ON sessions (demo_id);

CREATE TABLE IF NOT EXISTS moment_bookmarks (
    id              TEXT PRIMARY KEY,
    session_id      TEXT REFERENCES sessions(id),
    demo_id         TEXT NOT NULL REFERENCES playable_demos(id),
    frame           INTEGER NOT NULL,
    state_blob      BLOB,
    player_label    TEXT,
    auto_tags       TEXT NOT NULL DEFAULT '[]',  -- JSON array of strings
    screenshot_url  TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_bookmarks_demo    ON moment_bookmarks (demo_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_session ON moment_bookmarks (session_id);

-- =============================================================================
-- FTS5 virtual tables for full-text search
-- Separate from the main tables — keep in sync via triggers (Phase 1)
-- =============================================================================

CREATE VIRTUAL TABLE IF NOT EXISTS mechanics_fts USING fts5(
    name,
    short_definition,
    verbs,
    content='mechanics',
    content_rowid='rowid'
);

CREATE VIRTUAL TABLE IF NOT EXISTS works_fts USING fts5(
    title,
    significance,
    content='works',
    content_rowid='rowid'
);

CREATE VIRTUAL TABLE IF NOT EXISTS demos_fts USING fts5(
    title,
    description,
    content='playable_demos',
    content_rowid='rowid'
);
