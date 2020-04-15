#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let pg_url = std::env::var("DATABASE_URL");
    if let Ok(url) = pg_url {
        let version = if url.starts_with("mysql") {
            use sqlx_core::mysql::MySqlQueryAs;
            let pool = sqlx::MySqlPool::builder().max_size(5).build(&url).await?;
            let row: (String,) = sqlx::query_as(r#"SELECT version() v"#)
                .fetch_one(&pool)
                .await?;
            "MySQL ".to_string() + &row.0
        } else {
            use sqlx_core::postgres::PgQueryAs;
            let pool = sqlx::PgPool::builder().max_size(5).build(&url).await?;
            let row: (String,) = sqlx::query_as(r#"SELECT version() v"#)
                .fetch_one(&pool)
                .await?;
            row.0
        };
        println!("Get connection to {}. 👍", version)
    }
    Ok(())
}
