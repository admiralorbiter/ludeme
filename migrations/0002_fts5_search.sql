-- =============================================================================
-- FTS5 full-text search index
-- =============================================================================
-- Indexes demo titles/descriptions, mechanic names/definitions, and work titles
-- so users can search across all entities from the nav bar.

-- Drop any prior version (from a previous failed migration attempt)
DROP TABLE IF EXISTS search_index;

CREATE VIRTUAL TABLE search_index USING fts5(
    entity_type,       -- 'demo', 'mechanic', or 'work'
    entity_id,         -- UUID of the source row
    title,             -- primary searchable name
    body,              -- description / definition / body text
    tags,              -- space-separated tags for filtering
    tokenize='porter unicode61'
);

-- Populate from existing data
INSERT INTO search_index (entity_type, entity_id, title, body, tags)
SELECT 'demo', id, title, COALESCE(description, ''), COALESCE(REPLACE(REPLACE(mechanic_tags, '"', ''), ',', ' '), '')
FROM playable_demos;

INSERT INTO search_index (entity_type, entity_id, title, body, tags)
SELECT 'mechanic', id, name, COALESCE(short_definition, ''), COALESCE(family, '')
FROM mechanics;

INSERT INTO search_index (entity_type, entity_id, title, body, tags)
SELECT 'work', id, title, COALESCE(significance, ''), COALESCE(platform, '')
FROM works;
