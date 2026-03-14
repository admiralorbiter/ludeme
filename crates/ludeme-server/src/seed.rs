// ludeme-server/src/seed.rs
//
// Idempotent taxonomy seeding from taxonomy/*.toml files.
// Called once on startup. Safe to re-run — uses INSERT OR IGNORE.

use sqlx::SqlitePool;
use tracing::info;

// ---------------------------------------------------------------------------
// TOML types matching taxonomy/*.toml file structures
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct FamilyFile {
    families: Vec<FamilyEntry>,
}

#[derive(serde::Deserialize)]
struct FamilyEntry {
    slug:        String,
    label:       String,
    description: String,
}

#[derive(serde::Deserialize)]
struct RelationFile {
    relations: Vec<RelationEntry>,
}

#[derive(serde::Deserialize)]
struct RelationEntry {
    slug:        String,
    label:       String,
    description: String,
    directed:    bool,
}

#[derive(serde::Deserialize)]
struct FidelityFile {
    levels: Vec<FidelityEntry>,
}

#[derive(serde::Deserialize)]
struct FidelityEntry {
    slug:                             String,
    label:                            String,
    description:                      String,
    requires_notable_interpretations: bool,
    requires_hypothesis:              bool,
}

// ---------------------------------------------------------------------------
// Main seeding function
// ---------------------------------------------------------------------------

pub async fn seed_taxonomy(pool: &SqlitePool) -> anyhow::Result<()> {
    seed_families(pool).await?;
    seed_relations(pool).await?;
    seed_fidelity(pool).await?;
    Ok(())
}

async fn seed_families(pool: &SqlitePool) -> anyhow::Result<()> {
    let toml_str = std::fs::read_to_string("taxonomy/mechanic-families.toml")
        .map_err(|e| anyhow::anyhow!("Cannot read mechanic-families.toml: {e}"))?;
    let parsed: FamilyFile = toml::from_str(&toml_str)?;

    let mut count = 0usize;
    for f in parsed.families {
        let rows = sqlx::query!(
            "INSERT OR IGNORE INTO mechanic_families (slug, label, description) VALUES (?, ?, ?)",
            f.slug, f.label, f.description
        )
        .execute(pool)
        .await?
        .rows_affected();
        count += rows as usize;
    }
    info!("Taxonomy: {} mechanic families seeded", count);
    Ok(())
}

async fn seed_relations(pool: &SqlitePool) -> anyhow::Result<()> {
    let toml_str = std::fs::read_to_string("taxonomy/relation-types.toml")
        .map_err(|e| anyhow::anyhow!("Cannot read relation-types.toml: {e}"))?;
    let parsed: RelationFile = toml::from_str(&toml_str)?;

    let mut count = 0usize;
    for r in parsed.relations {
        let directed = if r.directed { 1i64 } else { 0i64 };
        let rows = sqlx::query!(
            "INSERT OR IGNORE INTO relation_types (slug, label, description, directed) VALUES (?, ?, ?, ?)",
            r.slug, r.label, r.description, directed
        )
        .execute(pool)
        .await?
        .rows_affected();
        count += rows as usize;
    }
    info!("Taxonomy: {} relation types seeded", count);
    Ok(())
}

async fn seed_fidelity(pool: &SqlitePool) -> anyhow::Result<()> {
    let toml_str = std::fs::read_to_string("taxonomy/fidelity-levels.toml")
        .map_err(|e| anyhow::anyhow!("Cannot read fidelity-levels.toml: {e}"))?;
    let parsed: FidelityFile = toml::from_str(&toml_str)?;

    let mut count = 0usize;
    for level in parsed.levels {
        let req_interp = if level.requires_notable_interpretations { 1i64 } else { 0i64 };
        let req_hypo   = if level.requires_hypothesis { 1i64 } else { 0i64 };
        let rows = sqlx::query!(
            "INSERT OR IGNORE INTO fidelity_levels (slug, label, description, requires_notable_interpretations, requires_hypothesis)
             VALUES (?, ?, ?, ?, ?)",
            level.slug, level.label, level.description, req_interp, req_hypo
        )
        .execute(pool)
        .await?
        .rows_affected();
        count += rows as usize;
    }
    info!("Taxonomy: {} fidelity levels seeded", count);
    Ok(())
}
