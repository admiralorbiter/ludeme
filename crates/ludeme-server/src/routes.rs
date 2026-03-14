// ludeme-server/src/routes.rs
//
// All HTTP route handlers. Grouped by resource.
// Each handler receives the SqlitePool from Axum state.

use axum::{
    extract::{Path, State},
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
        .route("/demos",         get(list_demos))
        .route("/demos/{id}",    get(get_demo))
        // Mechanics
        .route("/mechanics",     get(list_mechanics))
        .route("/mechanics/{id}", get(get_mechanic))
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
        "SELECT * FROM playable_demos WHERE publish_state = 'public' ORDER BY created_at DESC"
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
        "SELECT * FROM mechanics WHERE publish_state = 'public' ORDER BY name ASC"
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
