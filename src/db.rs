use sqlx::PgPool;

pub type DbPool = PgPool;

pub async fn get_pool(database_url: &str, max_conns: u32) -> Result<DbPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(database_url)
        .await
}