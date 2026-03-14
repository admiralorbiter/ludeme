// ludeme-server/src/seed_sample.rs
//
// Idempotent sample slice seeding. Inserts complete entity graphs on boot
// to prove the schema works end-to-end. Uses fixed IDs so re-runs are safe.
//
// Slice 1: Pong (1972)  — collision-response, scoring-pressure
// Slice 2: Maze Chase (1980) — ai-behavior, state-transitions

use sqlx::SqlitePool;
use tracing::info;

// Fixed UUIDs — deterministic so INSERT OR IGNORE works across restarts.
// Pong slice
const WORK_PONG:           &str = "00000000-0000-4000-8000-000000000001";
const MECH_BALL_DEFLECT:   &str = "00000000-0000-4000-8000-000000000002";
const MECH_SCORE_THRESHOLD:&str = "00000000-0000-4000-8000-000000000003";
const DEMO_PONG_76:        &str = "00000000-0000-4000-8000-000000000004";
const EDGE_WORK_DEMO:      &str = "00000000-0000-4000-8000-000000000010";
const EDGE_MECH1_DEMO:     &str = "00000000-0000-4000-8000-000000000011";
const EDGE_MECH2_DEMO:     &str = "00000000-0000-4000-8000-000000000012";

// Maze slice
const WORK_PACMAN:         &str = "00000000-0000-4000-8000-000000000030";
const MECH_GHOST_AI:       &str = "00000000-0000-4000-8000-000000000031";
const MECH_STATE_MACHINE:  &str = "00000000-0000-4000-8000-000000000032";
const DEMO_MAZE_80:        &str = "00000000-0000-4000-8000-000000000033";
const EDGE_WORK_MAZE:      &str = "00000000-0000-4000-8000-000000000034";
const EDGE_MECH3_MAZE:     &str = "00000000-0000-4000-8000-000000000035";
const EDGE_MECH4_MAZE:     &str = "00000000-0000-4000-8000-000000000036";

// Collection
const COLLECTION_ORIGINS:  &str = "00000000-0000-4000-8000-000000000020";

pub async fn seed_sample_slice(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut count = 0usize;

    // =====================================================================
    // Slice 1: Pong (1972)
    // =====================================================================

    let r = sqlx::query!(
        "INSERT OR IGNORE INTO works (id, title, year, platform, genre, significance, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        WORK_PONG, "Pong", 1972i64, "Arcade", "Sports",
        "The first commercially successful video game. Established ball-and-paddle as a genre and proved video games as a viable business.",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let verbs1 = r#"["deflect","bounce","angle"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_BALL_DEFLECT, "Ball Deflection", "collision-response",
        "Ball bounces off paddle at an angle determined by where it hits. Center hits return flat; edge hits create steep angles.",
        verbs1,
        "Ball passes the paddle — point scored against the player",
        "Consistent angle control to place the ball where the opponent cannot reach",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let verbs2 = r#"["score","win","reset"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_SCORE_THRESHOLD, "Score Threshold", "scoring-pressure",
        "First player to reach a point threshold wins. Creates escalating pressure as scores approach the limit.",
        verbs2,
        "Falling behind in score with limited comeback opportunity",
        "Maintaining a lead while managing risk in each rally",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let tags = r#"["collision-response","scoring-pressure"]"#;
    let pong_graph = r#"{"states":[{"id":"serving","label":"Serving"},{"id":"rally","label":"Rally"},{"id":"point_scored","label":"Point Scored"},{"id":"game_over","label":"Game Over"}],"transitions":[{"from":"serving","to":"rally","trigger":"serve"},{"from":"rally","to":"point_scored","trigger":"miss"},{"from":"point_scored","to":"serving","trigger":"reset"},{"from":"point_scored","to":"game_over","trigger":"threshold_reached"}]}"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO playable_demos (id, title, linked_work, mechanic_tags, fidelity_level, branch_id, wasm_path, description, era, platform, state_graph, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        DEMO_PONG_76, "Pong", WORK_PONG, tags, "faithful", "main",
        "/demos/pong-76/pong_76.js",
        "Ball and paddle — the mechanic that started everything. No friction, no gravity beyond deflection angle and speed. The simplest complete mechanic system in the canon.",
        "1972", "Arcade", pong_graph, "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    // Ensure wasm_path is correct even if the row existed before
    sqlx::query!("UPDATE playable_demos SET wasm_path = '/demos/pong-76/pong_76.js' WHERE id = ? AND (wasm_path != '/demos/pong-76/pong_76.js' OR wasm_path IS NULL)", DEMO_PONG_76)
        .execute(pool).await?;

    // Pong edges
    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_WORK_DEMO, WORK_PONG, "work", DEMO_PONG_76, "demo", "demonstrates", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH1_DEMO, MECH_BALL_DEFLECT, "mechanic", DEMO_PONG_76, "demo", "demonstrated-in", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH2_DEMO, MECH_SCORE_THRESHOLD, "mechanic", DEMO_PONG_76, "demo", "demonstrated-in", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    // =====================================================================
    // Slice 2: Maze Chase / Pac-Man (1980)
    // =====================================================================

    let r = sqlx::query!(
        "INSERT OR IGNORE INTO works (id, title, year, platform, genre, significance, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        WORK_PACMAN, "Pac-Man", 1980i64, "Arcade", "Maze",
        "Introduced character-driven gameplay and distinct AI ghost personalities. Pioneered the maze-chase genre and became a cultural icon.",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let verbs3 = r#"["chase","scatter","pursue"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_GHOST_AI, "Ghost Chase AI", "ai-behavior",
        "Enemies move toward the player using pathfinding. Different ghosts can have different chase strategies, creating emergent behavior.",
        verbs3,
        "Getting cornered by converging ghost paths with no escape route",
        "Reading ghost movement patterns to predict safe corridors and create openings",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let verbs4 = r#"["transition","switch","trigger"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_STATE_MACHINE, "State Machine Transitions", "state-transitions",
        "Game cycles through discrete states (chase, scatter, frightened) that change how entities behave. Players must adapt to the current state.",
        verbs4,
        "Misjudging the current state and treating a chase phase as scatter",
        "Tracking state durations and exploiting transition windows for maximum pellet gain",
        "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let maze_tags = r#"["ai-behavior","state-transitions"]"#;
    let maze_graph = r#"{"states":[{"id":"playing","label":"Playing"},{"id":"caught","label":"Caught"},{"id":"level_clear","label":"Level Clear"},{"id":"game_over","label":"Game Over"}],"transitions":[{"from":"playing","to":"caught","trigger":"ghost_collision"},{"from":"caught","to":"playing","trigger":"respawn"},{"from":"caught","to":"game_over","trigger":"no_lives"},{"from":"playing","to":"level_clear","trigger":"all_pellets"}]}"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO playable_demos (id, title, linked_work, mechanic_tags, fidelity_level, branch_id, wasm_path, description, era, platform, state_graph, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        DEMO_MAZE_80, "Maze Chase", WORK_PACMAN, maze_tags, "interpreted", "main",
        "/demos/maze-80/maze_80.js",
        "Navigate a maze, collect all pellets, and evade ghost pursuers. The AI ghosts chase using pathfinding — read their patterns to survive.",
        "1980", "Arcade", maze_graph, "public"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    // Maze edges
    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_WORK_MAZE, WORK_PACMAN, "work", DEMO_MAZE_80, "demo", "demonstrates", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH3_MAZE, MECH_GHOST_AI, "mechanic", DEMO_MAZE_80, "demo", "demonstrated-in", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    let r = sqlx::query!("INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence) VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH4_MAZE, MECH_STATE_MACHINE, "mechanic", DEMO_MAZE_80, "demo", "demonstrated-in", "established"
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    // =====================================================================
    // Collection: Origins of Arcade (Pong + Maze Chase)
    // =====================================================================

    let ordered = serde_json::json!([DEMO_PONG_76, DEMO_MAZE_80]).to_string();
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO collections (id, title, learning_goal, ordered_items, publish_state)
         VALUES (?, ?, ?, ?, 'public')",
        COLLECTION_ORIGINS,
        "Origins of Bounce",
        "Trace how ball-and-paddle mechanics evolved from the first electronic game to modern physics engines.",
        ordered
    ).execute(pool).await?.rows_affected();
    count += r as usize;

    // Update collection to include maze-80 if it already existed
    sqlx::query!("UPDATE collections SET ordered_items = ? WHERE id = ?", ordered, COLLECTION_ORIGINS)
        .execute(pool).await?;

    info!("Sample slice: {} entities seeded (0 = already present)", count);
    Ok(())
}
