// ludeme-server/src/main.rs
//
// Axum backend entry point. Boots the HTTP server, connects to Postgres,
// and seeds the taxonomy tables on startup.
//
// Phase 0 stub — routes and DB logic will be filled in during Phase 0 proper.

use axum::{response::IntoResponse, routing::get, Json, Router};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load .env file if present.
    dotenvy::dotenv().ok();

    // Initialize structured logging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ludeme_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO(phase-0): establish SQLx database connection pool.
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to Postgres");

    // TODO(phase-0): run pending SQLx migrations.
    // sqlx::migrate!("./migrations").run(&pool).await.expect("Migration failed");

    // TODO(phase-0): seed taxonomy TOML files into the database.

    let app = Router::new().route("/health", get(health_handler));

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("ludeme-server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "ludeme-server" }))
}
