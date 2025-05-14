#[cfg(feature = "ssr")]
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[cfg(feature = "ssr")]
pub async fn init_db() -> impl Fn() + Send + Sync + Clone {
    let pool = SqlitePoolOptions::new()
        // TODO: configure database URL
        .connect("sqlite://test.db")
        .await
        .expect("Failed to connect to database");

    move || provide_context(pool.clone())
}

#[cfg(feature = "ssr")]
pub fn use_db() -> SqlitePool {
    use sqlx::sqlite::SqlitePool;
    use_context::<SqlitePool>().expect("No database connection found")
}
