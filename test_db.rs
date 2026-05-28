use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost/postgres")
        .await;

    if pool.is_ok() {
        println!("DB connection successful");
    } else {
        println!("DB connection failed");
    }

    Ok(())
}
