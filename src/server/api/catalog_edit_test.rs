use super::*;
fn edit(price: &str) -> ProductEdit {
    ProductEdit {
        name: "Saved cake".into(),
        description: "Owner description".into(),
        price: price.into(),
    }
}
#[test]
fn exact_cents_reject_fractional_negative_overflow_and_non_numeric_prices() {
    for (price, cents) in [
        ("0", 0),
        ("45.01", 4501),
        ("1.2", 120),
        ("10000000.00", 1_000_000_000),
    ] {
        assert_eq!(price_cents(&edit(price)), Some(cents));
    }
    for price in [
        "-0.01",
        "NaN",
        "1e100",
        "10000000.01",
        "45.001",
        "1.",
        ".1",
        " 1.00",
        "999999999999999999999999999",
    ] {
        assert_eq!(price_cents(&edit(price)), None, "{price}");
    }
    let mut empty = edit("1");
    empty.name = " ".into();
    assert_eq!(price_cents(&empty), None);
}
#[test]
fn catalog_edits_require_bounded_owner_or_admin_claims() {
    let mut claims = server_common::Claims {
        sub: "owner".into(),
        exp: 0,
        iat: 0,
        organization_id: Some("tenant-a".into()),
        username: String::new(),
        email: String::new(),
        roles: vec!["ADMIN".into()],
        session_id: None,
        jti: String::new(),
    };
    assert_eq!(authorized_tenant(&claims), Ok("tenant-a"));
    claims.roles = vec!["MEMBER".into()];
    assert_eq!(authorized_tenant(&claims), Err(StatusCode::FORBIDDEN));
    claims.roles = vec!["OWNER".into()];
    assert_eq!(authorized_tenant(&claims), Ok("tenant-a"));
    for tenant in ["", "system", " tenant-a "] {
        claims.organization_id = Some(tenant.into());
        assert_eq!(authorized_tenant(&claims), Err(StatusCode::UNAUTHORIZED));
    }
}
async fn fixture() -> sqlx::PgPool {
    let url =
        std::env::var("OHC_CATALOG_TEST_DATABASE_URL").expect("isolated test database required");
    assert!(url.starts_with("postgres://postgres@127.0.0.1:"));
    assert!(url.ends_with("/ohc_catalog_edit_test"));
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    let schema = format!("catalog_edit_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(&format!("SET search_path TO {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TABLE products(id TEXT PRIMARY KEY,tenant_id TEXT,title TEXT,description TEXT,type TEXT,price_cents BIGINT,price NUMERIC,inventory_count INT,seo_title TEXT,seo_description TEXT,seo_schema_json JSONB,updated_at TIMESTAMPTZ); INSERT INTO products VALUES ('a','tenant-a','Original','Original description','Service',5000,50,8,'Old SEO','Old SEO','{}',CURRENT_TIMESTAMP),('b','tenant-b','Private','Private description','Product',9900,99,4,'Private SEO','Private SEO','{}',CURRENT_TIMESTAMP);").execute(&pool).await.unwrap();
    pool
}
#[tokio::test]
#[ignore = "requires OHC_CATALOG_TEST_DATABASE_URL"]
async fn postgres_edit_changes_only_owned_fields_and_invalidates_old_metadata() {
    let pool = fixture().await;
    assert_eq!(
        update_postgres(&pool, "tenant-a", "a", &edit("45.01"), 4501)
            .await
            .unwrap()
            .as_deref(),
        Some("Service")
    );
    let row:(String,String,i64,String,i32,bool)=sqlx::query_as("SELECT title,description,price_cents,price::text,inventory_count,seo_title IS NULL AND seo_description IS NULL AND seo_schema_json IS NULL FROM products WHERE id='a'").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "Saved cake");
    assert_eq!(row.1, "Owner description");
    assert_eq!(row.2, 4501);
    assert_eq!(row.3.parse::<f64>().unwrap(), 45.01);
    assert_eq!(row.4, 8);
    assert!(row.5);
    assert!(
        update_postgres(&pool, "tenant-a", "b", &edit("1"), 100)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        update_postgres(&pool, "tenant-a", "missing", &edit("1"), 100)
            .await
            .unwrap()
            .is_none()
    );
    let other: (String, i64) =
        sqlx::query_as("SELECT title,price_cents FROM products WHERE id='b'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(other, ("Private".into(), 9900));
}
#[tokio::test]
#[ignore = "requires OHC_CATALOG_TEST_DATABASE_URL"]
async fn postgres_rejected_edit_preserves_price_title_and_seo() {
    let pool = fixture().await;
    sqlx::query("ALTER TABLE products ADD CONSTRAINT reject_edit CHECK(title <> 'Saved cake')")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        update_postgres(&pool, "tenant-a", "a", &edit("45.01"), 4501)
            .await
            .is_err()
    );
    let row: (String, i64, String) =
        sqlx::query_as("SELECT title,price_cents,seo_title FROM products WHERE id='a'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row, ("Original".into(), 5000, "Old SEO".into()));
}
#[test]
fn product_events_carry_matching_verified_tenant_fields_and_saved_cents() {
    let event = product_event_payload("tenant-a", "p", "Cake", "Description", "Product", 4501);
    assert_eq!(event["tenant_id"], "tenant-a");
    assert_eq!(event["organization_id"], "tenant-a");
    assert_eq!(event["price_cents"], 4501);
    assert_eq!(event["price"], 45.01);
}
