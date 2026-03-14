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

## Database schema

The full schema is in [`migrations/0001_initial.sql`](../migrations/0001_initial.sql). Key SQLite translation decisions vs a Postgres sketch:

| Postgres type | SQLite equivalent | Notes |
|---|---|---|
| `UUID` | `TEXT` | Stored as hyphenated string, generated in Rust with `uuid::Uuid::new_v4()` |
| `TIMESTAMPTZ` | `TEXT` | ISO-8601 format via `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')` |
| `JSONB` | `TEXT` | Serialized JSON, parsed in app layer |
| `TEXT[]` | `TEXT` | JSON array, e.g. `'["mechanic-a","mechanic-b"]'` |
| `BYTEA` | `BLOB` | Input logs, state blobs |
| `tsvector` / full-text | `FTS5` virtual table | `mechanics_fts`, `works_fts`, `demos_fts` — content-backed, synced via rebuild |

Tables: `mechanics`, `works`, `playable_demos`, `observations`, `experiments`, `comparisons`, `collections`, `sources`, `relationship_edges`, `sessions`, `moment_bookmarks` plus taxonomy tables `mechanic_families`, `relation_types`, `fidelity_levels`.
