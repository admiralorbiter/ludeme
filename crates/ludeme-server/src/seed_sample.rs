// ludeme-server/src/seed_sample.rs
//
// Idempotent sample slice seeding. Inserts one complete entity graph on boot
// to prove the schema works end-to-end. Uses fixed IDs so re-runs are safe.
//
// Slice: Pong (1972)
//   Work:      "Pong" (1972, Arcade)
//   Mechanic:  "Ball Deflection" (collision-response)
//   Mechanic:  "Score Threshold" (scoring-pressure)
//   Demo:      "pong-76" (faithful, linked to Work, tagged with both mechanics)
//   Edges:     Work→Demo, Mechanic→Demo, Mechanic→Demo

use sqlx::SqlitePool;
use tracing::info;

// Fixed UUIDs — deterministic so INSERT OR IGNORE works across restarts.
const WORK_PONG:           &str = "00000000-0000-4000-8000-000000000001";
const MECH_BALL_DEFLECT:   &str = "00000000-0000-4000-8000-000000000002";
const MECH_SCORE_THRESHOLD:&str = "00000000-0000-4000-8000-000000000003";
const DEMO_PONG_76:        &str = "00000000-0000-4000-8000-000000000004";
const EDGE_WORK_DEMO:      &str = "00000000-0000-4000-8000-000000000010";
const EDGE_MECH1_DEMO:     &str = "00000000-0000-4000-8000-000000000011";
const EDGE_MECH2_DEMO:     &str = "00000000-0000-4000-8000-000000000012";
const COLLECTION_ORIGINS:  &str = "00000000-0000-4000-8000-000000000020";

pub async fn seed_sample_slice(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut count = 0usize;

    // -- Work: Pong --------------------------------------------------------
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO works (id, title, year, platform, genre, significance, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        WORK_PONG,
        "Pong",
        1972i64,
        "Arcade",
        "Sports",
        "The first commercially successful video game. Established ball-and-paddle as a genre and proved video games as a viable business.",
        "public"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    // -- Mechanic: Ball Deflection -----------------------------------------
    let verbs1 = r#"["deflect","bounce","angle"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_BALL_DEFLECT,
        "Ball Deflection",
        "collision-response",
        "Ball bounces off paddle at an angle determined by where it hits. Center hits return flat; edge hits create steep angles.",
        verbs1,
        "Ball passes the paddle — point scored against the player",
        "Consistent angle control to place the ball where the opponent cannot reach",
        "public"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    // -- Mechanic: Score Threshold -----------------------------------------
    let verbs2 = r#"["score","win","reset"]"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        MECH_SCORE_THRESHOLD,
        "Score Threshold",
        "scoring-pressure",
        "First player to reach a point threshold wins. Creates escalating pressure as scores approach the limit.",
        verbs2,
        "Falling behind in score with limited comeback opportunity",
        "Maintaining a lead while managing risk in each rally",
        "public"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    // -- Demo: pong-76 -----------------------------------------------------
    let tags = r#"["collision-response","scoring-pressure"]"#;
    let state_graph = r#"{"states":[{"id":"serving","label":"Serving"},{"id":"rally","label":"Rally"},{"id":"point_scored","label":"Point Scored"},{"id":"game_over","label":"Game Over"}],"transitions":[{"from":"serving","to":"rally","trigger":"serve"},{"from":"rally","to":"point_scored","trigger":"miss"},{"from":"point_scored","to":"serving","trigger":"reset"},{"from":"point_scored","to":"game_over","trigger":"threshold_reached"}]}"#;
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO playable_demos (id, title, linked_work, mechanic_tags, fidelity_level, branch_id, wasm_path, description, era, platform, state_graph, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        DEMO_PONG_76,
        "Pong",
        WORK_PONG,
        tags,
        "faithful",
        "main",
        "/demos/pong-76/pong_76.js",
        "Ball and paddle — the mechanic that started everything. No friction, no gravity beyond deflection angle and speed. The simplest complete mechanic system in the canon.",
        "1972",
        "Arcade",
        state_graph,
        "public"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    // Ensure wasm_path is correct even if the row existed before this fix
    let wasm_path = "/demos/pong-76/pong_76.js";
    sqlx::query!(
        "UPDATE playable_demos SET wasm_path = ? WHERE id = ? AND (wasm_path != ? OR wasm_path IS NULL)",
        wasm_path, DEMO_PONG_76, wasm_path
    )
    .execute(pool)
    .await?;

    // -- Edges -------------------------------------------------------------
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_WORK_DEMO, WORK_PONG, "work", DEMO_PONG_76, "demo", "demonstrates", "established"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    let r = sqlx::query!(
        "INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH1_DEMO, MECH_BALL_DEFLECT, "mechanic", DEMO_PONG_76, "demo", "demonstrated-in", "established"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    let r = sqlx::query!(
        "INSERT OR IGNORE INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        EDGE_MECH2_DEMO, MECH_SCORE_THRESHOLD, "mechanic", DEMO_PONG_76, "demo", "demonstrated-in", "established"
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    // -- Collection: Origins of Bounce ----------------------------------------
    let ordered = serde_json::json!([DEMO_PONG_76]).to_string();
    let r = sqlx::query!(
        "INSERT OR IGNORE INTO collections (id, title, learning_goal, ordered_items, publish_state)
         VALUES (?, ?, ?, ?, 'public')",
        COLLECTION_ORIGINS,
        "Origins of Bounce",
        "Trace how ball-and-paddle mechanics evolved from the first electronic game to modern physics engines.",
        ordered
    )
    .execute(pool)
    .await?
    .rows_affected();
    count += r as usize;

    info!("Sample slice: {} entities seeded (0 = already present)", count);
    Ok(())
}
