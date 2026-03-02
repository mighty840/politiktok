use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::infrastructure::Error;

/// Database wrapper around a PostgreSQL connection pool.
#[derive(Clone, Debug)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Connect to PostgreSQL and run migrations.
    pub async fn connect(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .map_err(|e| Error::DatabaseError(format!("Failed to connect: {e}")))?;

        tracing::info!("Connected to PostgreSQL");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| Error::DatabaseError(format!("Migration failed: {e}")))?;

        tracing::info!("Database migrations applied");

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
