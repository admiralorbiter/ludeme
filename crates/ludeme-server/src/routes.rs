// ludeme-server/src/routes.rs
//
// All HTTP route handlers. Grouped by resource.
// Each handler receives the SqlitePool from Axum state.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

pub async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "ludeme-server" }))
}

// ---------------------------------------------------------------------------
// API router — mounted at /api in main.rs
// ---------------------------------------------------------------------------

pub fn api_router() -> Router<SqlitePool> {
    Router::new()
        // Demos
        .route("/demos",         get(list_demos).post(create_demo))
        .route("/demos/{id}",    get(get_demo))
        // Mechanics
        .route("/mechanics",     get(list_mechanics).post(create_mechanic))
        .route("/mechanics/{id}", get(get_mechanic))
        // Works
        .route("/works",         get(list_works).post(create_work))
        .route("/works/{id}",    get(get_work))
        // Relationship edges
        .route("/edges",         get(list_edges).post(create_edge))
        // Sessions
        .route("/sessions",      post(create_session))
        // Bookmarks
        .route("/bookmarks",     post(create_bookmark))
        // Taxonomy
        .route("/taxonomy/families",  get(list_families))
        .route("/taxonomy/relations", get(list_relations))
        .route("/taxonomy/fidelity",  get(list_fidelity))
}

// ---------------------------------------------------------------------------
// Row types (for sqlx query_as)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DemoRow {
    pub id:                        String,
    pub title:                     String,
    pub linked_work:               Option<String>,
    pub mechanic_tags:             String, // JSON
    pub fidelity_level:            String,
    pub branch_id:                 String,
    pub wasm_path:                 Option<String>,
    pub param_manifest:            Option<String>, // JSON
    pub state_graph:               Option<String>, // JSON
    pub description:               Option<String>,
    pub era:                       Option<String>,
    pub platform:                  Option<String>,
    pub notable_interpretations:   Option<String>, // JSON
    pub hypothesis:                Option<String>,
    pub publish_state:             String,
    pub created_at:                String,
    pub updated_at:                String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MechanicRow {
    pub id:               String,
    pub name:             String,
    pub family:           String,
    pub short_definition: Option<String>,
    pub verbs:            String, // JSON
    pub failure_pattern:  Option<String>,
    pub mastery_pattern:  Option<String>,
    pub state_graph:      Option<String>, // JSON
    pub publish_state:    String,
    pub created_at:       String,
    pub updated_at:       String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WorkRow {
    pub id:                  String,
    pub title:               String,
    pub year:                Option<i64>,
    pub platform:            Option<String>,
    pub genre:               Option<String>,
    pub significance:        Option<String>,
    pub notable_constraints: Option<String>,
    pub publish_state:       String,
    pub created_at:          String,
    pub updated_at:          String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EdgeRow {
    pub id:            String,
    pub from_id:       String,
    pub from_type:     String,
    pub to_id:         String,
    pub to_type:       String,
    pub relation_type: String,
    pub confidence:    String,
    pub note:          Option<String>,
    pub created_at:    String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TaxonomyRow {
    pub slug:        String,
    pub label:       String,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Demo handlers
// ---------------------------------------------------------------------------

async fn list_demos(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let demos = sqlx::query_as::<_, DemoRow>(
        "SELECT * FROM playable_demos ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(demos))
}

async fn get_demo(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let demo = sqlx::query_as::<_, DemoRow>(
        "SELECT * FROM playable_demos WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?;

    match demo {
        Some(d) => Ok(Json(d).into_response()),
        None    => Err(AppError::NotFound(format!("demo '{id}' not found"))),
    }
}

// ---------------------------------------------------------------------------
// Mechanic handlers
// ---------------------------------------------------------------------------

async fn list_mechanics(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let mechanics = sqlx::query_as::<_, MechanicRow>(
        "SELECT * FROM mechanics ORDER BY name ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(mechanics))
}

async fn get_mechanic(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mechanic = sqlx::query_as::<_, MechanicRow>(
        "SELECT * FROM mechanics WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?;

    match mechanic {
        Some(m) => Ok(Json(m).into_response()),
        None    => Err(AppError::NotFound(format!("mechanic '{id}' not found"))),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMechanicBody {
    pub name:             String,
    pub family:           String,
    pub short_definition: Option<String>,
    pub verbs:            Option<Vec<String>>,
    pub failure_pattern:  Option<String>,
    pub mastery_pattern:  Option<String>,
    pub publish_state:    Option<String>,
}

async fn create_mechanic(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateMechanicBody>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let verbs = serde_json::to_string(&body.verbs.unwrap_or_default())
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let publish = body.publish_state.unwrap_or_else(|| "draft".to_string());

    sqlx::query!(
        "INSERT INTO mechanics (id, name, family, short_definition, verbs, failure_pattern, mastery_pattern, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id, body.name, body.family, body.short_definition, verbs, body.failure_pattern, body.mastery_pattern, publish
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Work handlers
// ---------------------------------------------------------------------------

async fn list_works(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let works = sqlx::query_as::<_, WorkRow>(
        "SELECT * FROM works ORDER BY title ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(works))
}

async fn get_work(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let work = sqlx::query_as::<_, WorkRow>(
        "SELECT * FROM works WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?;

    match work {
        Some(w) => Ok(Json(w).into_response()),
        None    => Err(AppError::NotFound(format!("work '{id}' not found"))),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkBody {
    pub title:               String,
    pub year:                Option<i64>,
    pub platform:            Option<String>,
    pub genre:               Option<String>,
    pub significance:        Option<String>,
    pub notable_constraints: Option<String>,
    pub publish_state:       Option<String>,
}

async fn create_work(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateWorkBody>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let publish = body.publish_state.unwrap_or_else(|| "draft".to_string());

    sqlx::query!(
        "INSERT INTO works (id, title, year, platform, genre, significance, notable_constraints, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id, body.title, body.year, body.platform, body.genre, body.significance, body.notable_constraints, publish
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Demo create handler
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateDemoBody {
    pub title:                   String,
    pub linked_work:             Option<String>,
    pub mechanic_tags:           Option<Vec<String>>,
    pub fidelity_level:          Option<String>,
    pub branch_id:               Option<String>,
    pub wasm_path:               Option<String>,
    pub description:             Option<String>,
    pub era:                     Option<String>,
    pub platform:                Option<String>,
    pub notable_interpretations: Option<Vec<String>>,
    pub hypothesis:              Option<String>,
    pub publish_state:           Option<String>,
}

async fn create_demo(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateDemoBody>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let tags = serde_json::to_string(&body.mechanic_tags.unwrap_or_default())
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let fidelity = body.fidelity_level.unwrap_or_else(|| "interpreted".to_string());
    let branch = body.branch_id.unwrap_or_else(|| "main".to_string());
    let publish = body.publish_state.unwrap_or_else(|| "draft".to_string());
    let interp = body.notable_interpretations
        .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| "[]".to_string()));

    sqlx::query!(
        "INSERT INTO playable_demos (id, title, linked_work, mechanic_tags, fidelity_level, branch_id, wasm_path, description, era, platform, notable_interpretations, hypothesis, publish_state)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id, body.title, body.linked_work, tags, fidelity, branch, body.wasm_path, body.description, body.era, body.platform, interp, body.hypothesis, publish
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Edge handlers
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct EdgeQuery {
    pub from_id: Option<String>,
    pub to_id:   Option<String>,
}

async fn list_edges(
    State(pool): State<SqlitePool>,
    Query(q): Query<EdgeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let edges = match (q.from_id, q.to_id) {
        (Some(fid), _) => {
            sqlx::query_as::<_, EdgeRow>(
                "SELECT * FROM relationship_edges WHERE from_id = ? ORDER BY created_at DESC"
            )
            .bind(&fid)
            .fetch_all(&pool)
            .await?
        }
        (_, Some(tid)) => {
            sqlx::query_as::<_, EdgeRow>(
                "SELECT * FROM relationship_edges WHERE to_id = ? ORDER BY created_at DESC"
            )
            .bind(&tid)
            .fetch_all(&pool)
            .await?
        }
        _ => {
            sqlx::query_as::<_, EdgeRow>(
                "SELECT * FROM relationship_edges ORDER BY created_at DESC LIMIT 100"
            )
            .fetch_all(&pool)
            .await?
        }
    };

    Ok(Json(edges))
}

#[derive(Debug, Deserialize)]
pub struct CreateEdgeBody {
    pub from_id:       String,
    pub from_type:     String,
    pub to_id:         String,
    pub to_type:       String,
    pub relation_type: String,
    pub confidence:    Option<String>,
    pub note:          Option<String>,
}

async fn create_edge(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateEdgeBody>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let confidence = body.confidence.unwrap_or_else(|| "tentative".to_string());

    sqlx::query!(
        "INSERT INTO relationship_edges (id, from_id, from_type, to_id, to_type, relation_type, confidence, note)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id, body.from_id, body.from_type, body.to_id, body.to_type, body.relation_type, confidence, body.note
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Session handler
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateSessionBody {
    pub demo_id:   String,
    pub branch_id: Option<String>,
    pub seed:      i64,
}

async fn create_session(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateSessionBody>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let branch = body.branch_id.unwrap_or_else(|| "main".to_string());

    sqlx::query!(
        "INSERT INTO sessions (id, demo_id, branch_id, seed) VALUES (?, ?, ?, ?)",
        id, body.demo_id, branch, body.seed
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Bookmark handler
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateBookmarkBody {
    pub session_id:   Option<String>,
    pub demo_id:      String,
    pub frame:        i64,
    pub player_label: Option<String>,
    pub auto_tags:    Vec<String>,
}

async fn create_bookmark(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateBookmarkBody>,
) -> Result<impl IntoResponse, AppError> {
    let id       = uuid::Uuid::new_v4().to_string();
    let tags_json = serde_json::to_string(&body.auto_tags)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO moment_bookmarks (id, session_id, demo_id, frame, player_label, auto_tags)
         VALUES (?, ?, ?, ?, ?, ?)",
        id, body.session_id, body.demo_id, body.frame, body.player_label, tags_json
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// ---------------------------------------------------------------------------
// Taxonomy handlers
// ---------------------------------------------------------------------------

async fn list_families(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, TaxonomyRow>(
        "SELECT slug, label, description FROM mechanic_families ORDER BY slug"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(rows))
}

async fn list_relations(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, TaxonomyRow>(
        "SELECT slug, label, description FROM relation_types ORDER BY slug"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(rows))
}

async fn list_fidelity(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, TaxonomyRow>(
        "SELECT slug, label, description FROM fidelity_levels ORDER BY slug"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(rows))
}

// ---------------------------------------------------------------------------
// Error type — maps DB and app errors to HTTP responses
// ---------------------------------------------------------------------------

pub enum AppError {
    Sqlx(sqlx::Error),
    NotFound(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { AppError::Sqlx(e) }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Sqlx(e)      => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound(m)  => (StatusCode::NOT_FOUND, m),
            AppError::Internal(m)  => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
