use sqlx::PgPool;
use uuid::Uuid;
use super::cro::{CroEngine, CroVariant};

#[tokio::test]
async fn test_cro_thompson_sampling_routing() {
    let engine = CroEngine::new(sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap());
    let experiment_id = Uuid::new_v4();

    let v1 = CroVariant {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        experiment_id,
        variant_name: "Control".to_string(),
        content: serde_json::json!({}),
        traffic_weight: 1.0,
        views: 100,
        conversions: 10,
    };

    let v2 = CroVariant {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        experiment_id,
        variant_name: "Treatment".to_string(),
        content: serde_json::json!({}),
        traffic_weight: 1.0,
        views: 100,
        conversions: 50,
    };

    let variants = vec![v1.clone(), v2.clone()];

    let mut v2_count = 0;
    for i in 0..100 {
        let user_id = format!("user_{}", i);
        let selected = engine.select_variant_thompson(&variants, &user_id, experiment_id).unwrap();
        if selected.id == v2.id {
            v2_count += 1;
        }
    }

    assert!(v2_count > 90, "Thompson sampling should heavily favor the clear winner");
}

#[tokio::test]
async fn test_cro_api_conversion_endpoint() {
    let (pool, _) = match crate::builder::builder_test::setup_db().await {
        Some(v) => v,
        None => return,
    };

    let engine = CroEngine::new(pool.clone());

    let experiment_id = Uuid::new_v4();
    let variant_id = Uuid::new_v4();

    let app = super::api::router(pool.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });

    let client = reqwest::Client::new();
    let base_url = format!("http://127.0.0.1:{}", port);

    let res = client.post(&format!("{}/builder/edge/cro/{}/convert", base_url, variant_id))
        .send().await.unwrap();

    assert_eq!(res.status(), 200, "CRO Conversion endpoint should return 200 OK");
}
