#[tokio::main]
async fn main() {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let res = SqlitePoolOptions::new().connect_with(conn_opts).await;
    println!("{:?}", res);
}
