#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let pg_url = std::env::var("DATABASE_URL");
    if let Ok(url) = pg_url {
        let version = if url.starts_with("mysql") {
            use sqlx::mysql::MySqlPoolOptions;
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;
            let row: (String,) = sqlx::query_as(r#"SELECT version() v"#)
                .fetch_one(&pool)
                .await?;
            "MySQL ".to_string() + &row.0
        } else {
            use sqlx::postgres::PgPoolOptions;
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;
            let row: (String,) = sqlx::query_as(r#"SELECT version() v"#)
                .fetch_one(&pool)
                .await?;
            row.0
        };
        println!("Get connection to {}. 👍", version)
    }
    Ok(())
}
