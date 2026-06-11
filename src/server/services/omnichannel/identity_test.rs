use ::server_lib::db::{DB, DbStore};
use ::server_services::omnichannel::identity::IdentityResolver;
use uuid::Uuid;

#[tokio::test]
async fn test_resolve_identity() {
    unsafe {
        std::env::set_var("OHC_DATABASE_URL", "sqlite::memory:");
    }
    let db = ::server_lib::db::DB::new().await.unwrap();
    let pool = db.pool.clone();

    // Use the actual store from db.new() which is Sqlite in test
    let resolver = IdentityResolver::new(db.store.clone(), pool.clone());

    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());

    // 1. Resolve new identity (should create customer)
    let provider = "instagram";
    let provider_id = format!("ig_user_{}", Uuid::new_v4());

    let customer_id_1 = resolver.resolve_identity(&tenant_id, provider, &provider_id, None, None, Some("Insta User")).await.unwrap();
    assert!(!customer_id_1.is_empty());

    // 2. Resolve same identity (should return same customer_id)
    let customer_id_2 = resolver.resolve_identity(&tenant_id, provider, &provider_id, None, None, None).await.unwrap();
    assert_eq!(customer_id_1, customer_id_2);

    // 3. Resolve by email (link new provider)
    let email = format!("test_{}@example.com", Uuid::new_v4());
    let new_customer_id = Uuid::new_v4().to_string();

    // Have to match here depending on whether db.store is sqlite or postgres.
    match db.store {
        DbStore::Postgres => {
            sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)")
                .bind(&new_customer_id)
                .bind(&tenant_id)
                .bind("Email User")
                .bind(&email)
                .execute(&pool).await.unwrap();
        },
        DbStore::Sqlite(ref sqlite_pool) => {
            sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES (?, ?, ?, ?)")
                .bind(&new_customer_id)
                .bind(&tenant_id)
                .bind("Email User")
                .bind(&email)
                .execute(sqlite_pool).await.unwrap();
        }
    }

    let new_provider_id = format!("wa_user_{}", Uuid::new_v4());
    let resolved_id_3 = resolver.resolve_identity(&tenant_id, "whatsapp", &new_provider_id, Some(&email), None, None).await.unwrap();
    assert_eq!(new_customer_id, resolved_id_3);

    // 4. Resolve same whatsapp (should return same customer_id)
    let resolved_id_4 = resolver.resolve_identity(&tenant_id, "whatsapp", &new_provider_id, None, None, None).await.unwrap();
    assert_eq!(new_customer_id, resolved_id_4);
}
