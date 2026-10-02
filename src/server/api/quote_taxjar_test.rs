use super::*;
async fn fixture() -> sqlx::PgPool {
    let url = std::env::var("OHC_QUOTE_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
    assert!(url.starts_with("postgres://postgres@127.0.0.1:"));
    assert!(url.ends_with("/ohc_quote_lookup_test"));
    let schema = format!("quote_lookup_{}", uuid::Uuid::new_v4().simple());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(&format!("SET search_path TO {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    pool
}
#[tokio::test]
#[ignore = "requires an isolated OHC_QUOTE_TEST_DATABASE_URL and explicit test TaxJar environment"]
async fn hosted_missing_optional_table_preserves_quote_transaction() {
    let pool = fixture().await;
    let mut tx = pool.begin().await.unwrap();
    let key = load_quote_taxjar_key(&mut tx, "tenant-a", false)
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&mut *tx)
            .await
            .unwrap(),
        1
    );
    assert!(key.is_none());
    tx.commit().await.unwrap();
}
#[tokio::test]
#[ignore = "requires an isolated OHC_QUOTE_TEST_DATABASE_URL and explicit test TaxJar environment"]
async fn hosted_missing_connection_never_uses_global_key() {
    let pool = fixture().await;
    sqlx::query("CREATE TABLE integrations (tenant_id TEXT,provider_id TEXT,api_token TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    let mut tx = pool.begin().await.unwrap();
    assert!(
        load_quote_taxjar_key(&mut tx, "tenant-a", false)
            .await
            .unwrap()
            .is_none()
    );
}
#[tokio::test]
#[ignore = "requires an isolated OHC_QUOTE_TEST_DATABASE_URL and explicit test TaxJar environment"]
async fn standalone_missing_connection_may_use_explicit_environment_key() {
    let pool = fixture().await;
    let mut tx = pool.begin().await.unwrap();
    assert_eq!(
        load_quote_taxjar_key(&mut tx, "tenant-a", true)
            .await
            .unwrap()
            .as_deref(),
        Some("local-regression-taxjar-key")
    );
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&mut *tx)
            .await
            .unwrap(),
        1
    );
}
#[tokio::test]
#[ignore = "requires an isolated OHC_QUOTE_TEST_DATABASE_URL and explicit test TaxJar environment"]
async fn tenant_key_is_scoped_and_explicit_disable_wins_over_environment() {
    let pool = fixture().await;
    sqlx::query("CREATE TABLE integrations (tenant_id TEXT,provider_id TEXT,api_token TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO integrations VALUES ('tenant-a','taxjar','owner-token'),('tenant-b','taxjar','')").execute(&pool).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    assert_eq!(
        load_quote_taxjar_key(&mut tx, "tenant-a", false)
            .await
            .unwrap()
            .as_deref(),
        Some("owner-token")
    );
    assert!(
        load_quote_taxjar_key(&mut tx, "tenant-b", true)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        load_quote_taxjar_key(&mut tx, "foreign", false)
            .await
            .unwrap()
            .is_none()
    );
}
#[tokio::test]
#[ignore = "requires an isolated OHC_QUOTE_TEST_DATABASE_URL and explicit test TaxJar environment"]
async fn schema_error_is_propagated_without_global_credential_fallback() {
    let pool = fixture().await;
    sqlx::query("CREATE TABLE integrations (tenant_id TEXT,provider_id TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    let mut tx = pool.begin().await.unwrap();
    assert!(
        load_quote_taxjar_key(&mut tx, "tenant-a", true)
            .await
            .is_err()
    );
}
