use anyhow::Result;
use sqlx::{Pool, Postgres};

pub struct DatabaseConnection {
    pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(50)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}