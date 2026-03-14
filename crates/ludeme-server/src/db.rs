// ludeme-server/src/db.rs
//
// Database helper types and utilities.
// The pool is passed as Axum state into all route handlers.

use sqlx::SqlitePool;

/// Type alias — makes handler signatures cleaner.
pub type Db = SqlitePool;
