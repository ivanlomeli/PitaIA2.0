use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn create_pool() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida");
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;
    
    // Ejecutar migraciones
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
