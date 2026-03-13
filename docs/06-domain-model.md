# Domain Model

## Entity reference

| Entity | What it represents | Key fields |
|---|---|---|
| PlayableDemo | A tiny interactive build focused on one historical slice or mechanic. | title, linked_work, mechanic_tags, fidelity_level, branch_id, wasm_path, param_manifest, state_graph |
| Mechanic | A rule, verb, pattern, or feel layer that can appear across multiple works. | name, family, verbs, rule_summary, failure_state, mastery_pattern, related_demos, lineage_edges |
| Work | A historical game, reference title, or notable design artifact. | title, year, platform, genre, significance, notable_constraints, related_mechanics, linked_demos |
| Platform/Era | Hardware or historical context that shapes design constraints. | name, time_range, input_constraints, technical_pressures, notable_examples |
| MomentBookmark | A specific frame, state, or scene captured from play. | demo_id, frame, seed, state_blob, input_log_ref, player_label, screenshot_url, auto_tags |
| Observation | A structured research note or insight with evidence. | claim, evidence_links, linked_entities, confidence, why_it_matters, follow_up_question, promotion_path |
| Experiment | A fork of a demo made to test a hypothesis. | parent_demo, param_delta, hypothesis, expected_effect, observed_result, decision |
| Comparison | A saved side-by-side view or analytic question. | items, dimensions, prompts, conclusion, follow_up_links |
| Collection/Trail | A curated learning path or thematic grouping. | title, audience, learning_goal, ordered_items, compare_prompts, exit_understanding |
| Source | Any citation, interview, footage, or reference material. | origin, relevance, trust_level, citation_notes, linked_entities |
| RelationshipEdge | A typed link between two entities. | from_id, from_type, to_id, to_type, relation_type, confidence, note |
| Session | A recorded play session. | demo_id, branch_id, seed, input_log, frame_count, duration_ms, frame_ticks |

## Database schema sketch

```sql
-- Core entities use UUID primary keys
-- All entity tables have: id, created_at, updated_at, publish_state

CREATE TABLE mechanics (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    family TEXT NOT NULL REFERENCES mechanic_families(slug),
    short_definition TEXT,
    rule_summary JSONB,
    verbs TEXT[],
    failure_pattern TEXT,
    mastery_pattern TEXT,
    state_graph JSONB,        -- nodes and transitions for overlay
    search_vector TSVECTOR,
    publish_state TEXT NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Typed relationship edges — the core graph table
CREATE TABLE relationship_edges (
    id UUID PRIMARY KEY,
    from_id UUID NOT NULL,
    from_type TEXT NOT NULL,  -- 'mechanic', 'work', 'demo', etc.
    to_id UUID NOT NULL,
    to_type TEXT NOT NULL,
    relation_type TEXT NOT NULL REFERENCES relation_types(slug),
    confidence TEXT NOT NULL DEFAULT 'tentative',
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON relationship_edges (from_id, relation_type);
CREATE INDEX ON relationship_edges (to_id, relation_type);

-- Sessions and moment bookmarks
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    demo_id UUID NOT NULL REFERENCES playable_demos(id),
    branch_id TEXT,
    seed BIGINT NOT NULL,
    input_log BYTEA,
    frame_ticks JSONB,        -- array of FrameTick for heat map
    frame_count BIGINT,
    duration_ms BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE moment_bookmarks (
    id UUID PRIMARY KEY,
    session_id UUID REFERENCES sessions(id),
    demo_id UUID NOT NULL REFERENCES playable_demos(id),
    frame BIGINT NOT NULL,
    state_blob BYTEA,
    player_label TEXT,
    auto_tags TEXT[],
    screenshot_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```
