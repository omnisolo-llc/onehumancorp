#[cfg(test)]
mod tests {
    use crate::services::dashboard::service::MyDashboardService;
    use crate::ohc::app::dashboard_service_server::DashboardService;
    use crate::ohc::app::GetDashboardRequest;
    use crate::db::DB;
    use crate::hub::Hub;
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use tonic::Request;

    #[tokio::test]
    async fn test_get_dashboard_basic() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db = Arc::new(DB::new().await.unwrap());
        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));
        let service = MyDashboardService::new(db.clone(), hub.clone());
        let req = Request::new(GetDashboardRequest { organization_id: "test".to_string() });
        let _ = service.get_dashboard(req).await;
    }

    #[tokio::test]
    async fn test_get_lightweight_dashboard_basic() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db = Arc::new(DB::new().await.unwrap());
        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));
        let service = MyDashboardService::new(db.clone(), hub.clone());
        let req = Request::new(GetDashboardRequest { organization_id: "test".to_string() });
        let _ = service.get_lightweight_dashboard(req).await;
    }
}
