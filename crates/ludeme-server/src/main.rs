// ludeme-server/src/main.rs
//
// Axum backend — boots, connects to SQLite, runs migrations, seeds taxonomy,
// and serves the REST API used by the SvelteKit shell.

mod db;
mod routes;
mod seed;
mod seed_sample;

use axum::{Router, routing::get};
use tracing::info;

use db::Db;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ludeme_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- Database ----------------------------------------------------------
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ludeme.db?mode=rwc".to_string());

    let pool = Db::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");

    // Enable WAL mode for better concurrent read performance
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await
        .expect("Failed to set WAL mode");

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .expect("Failed to enable foreign keys");

    // Run any pending migrations
    // Path is relative to crates/ludeme-server/ (the crate root where sqlx::migrate! runs)
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    info!("Database ready");

    // Seed taxonomy from TOML files (idempotent — safe to run every boot)
    seed::seed_taxonomy(&pool).await.expect("Taxonomy seed failed");

    // Seed the sample Pong slice (idempotent — proves the schema end-to-end)
    seed_sample::seed_sample_slice(&pool).await.expect("Sample slice seed failed");

    // --- Routes ------------------------------------------------------------
    let app = Router::new()
        .route("/health", get(routes::health))
        .nest("/api", routes::api_router())
        .with_state(pool);

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    info!("ludeme-server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
