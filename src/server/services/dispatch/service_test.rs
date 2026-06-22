use dispatch_proto_lib::ohc::dispatch::{
    dispatch_service_server::DispatchService, CreateRouteRequest, GetTravelPaddingRequest,
    LocalDispatchRoute,
};
use tonic::Request;
use crate::services::dispatch::service::DispatchServiceImpl;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_travel_padding() {
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/ohc").await;
        if let Ok(pool) = pool {
            let service = DispatchServiceImpl::new(pool);
            let req = Request::new(GetTravelPaddingRequest {
                from_location_id: "loc1".to_string(),
                to_location_id: "loc2".to_string(),
                time_of_day: "morning".to_string(),
            });

            let res = service.get_travel_padding(req).await;
            assert!(res.is_ok());
            let padding = res.unwrap().into_inner();
            assert_eq!(padding.estimated_minutes, 15);
        }
    }
}
