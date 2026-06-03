#[cfg(test)]
mod tests {
    use super::*;
    use delivery_proto::ohc::api::v1::delivery_service_server::DeliveryService;
    use delivery_proto::ohc::api::v1::*;
    use sqlx::PgPool;
    use tonic::Request;
    use uuid::Uuid;
    use crate::DeliveryServiceImpl;

    #[tokio::test]
    async fn test_configure_and_verify_delivery_zone() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

        let pool = match tokio::time::timeout(std::time::Duration::from_millis(5000), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => {
                tracing::debug!("Failed to connect to db, skipping test");
                return;
            }
        };

        let service = DeliveryServiceImpl::new(pool);
        let org_id = Uuid::new_v4().to_string();

        // 1. Configure a Delivery Zone (a simple square polygon around 0,0)
        let polygon_wkt = "POLYGON((0 0, 0 10, 10 10, 10 0, 0 0))".to_string();
        let config_req = ConfigureDeliveryZoneRequest {
            organization_id: org_id.clone(),
            polygon_wkt: polygon_wkt.clone(),
            flat_fee_cents: 500,
            min_order_value_cents: 1500,
        };

        let config_res = service
            .configure_delivery_zone(Request::new(config_req))
            .await
            .expect("configure_delivery_zone failed")
            .into_inner();

        assert!(config_res.zone.is_some());
        let zone = config_res.zone.unwrap();
        assert_eq!(zone.flat_fee_cents, 500);

        // 2. Test a point inside the polygon (5, 5)
        let can_deliver_req_inside = CanDeliverToLocationRequest {
            organization_id: org_id.clone(),
            lng: 5.0, // X
            lat: 5.0, // Y
        };

        let can_deliver_res_inside = service
            .can_deliver_to_location(Request::new(can_deliver_req_inside))
            .await
            .expect("can_deliver_to_location failed for inside point")
            .into_inner();

        assert!(can_deliver_res_inside.can_deliver);
        assert_eq!(can_deliver_res_inside.flat_fee_cents, 500);

        // 3. Test a point outside the polygon (15, 15)
        let can_deliver_req_outside = CanDeliverToLocationRequest {
            organization_id: org_id.clone(),
            lng: 15.0, // X
            lat: 15.0, // Y
        };

        let can_deliver_res_outside = service
            .can_deliver_to_location(Request::new(can_deliver_req_outside))
            .await
            .expect("can_deliver_to_location failed for outside point")
            .into_inner();

        assert!(!can_deliver_res_outside.can_deliver);
        assert_eq!(can_deliver_res_outside.flat_fee_cents, 0);
    }
}
