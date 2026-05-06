use std::sync::Arc;
use crate::ohc::orchestration::*;
use chrono::{Utc, Datelike};

pub struct DashboardService {
    db: Arc<sqlx::PgPool>,
}

impl DashboardService {
    pub fn new(db: Arc<sqlx::PgPool>) -> Self {
        DashboardService { db }
    }

    pub async fn get_dashboard_summary(&self, req: GetDashboardSummaryRequest) -> Result<GetDashboardSummaryResponse, tonic::Status> {
        let metrics = vec![
            DashboardMetric {
                id: "revenue_today".to_string(),
                label: "Revenue Today".to_string(),
                value: "$450.00".to_string(),
                trend_percentage: 12.5,
                trend_direction: "up".to_string(),
            },
            DashboardMetric {
                id: "active_orders".to_string(),
                label: "Active Orders".to_string(),
                value: "12".to_string(),
                trend_percentage: 5.0,
                trend_direction: "up".to_string(),
            },
        ];

        let _org_id = req.organization_id.clone();

        // Use db for something to avoid unused variable warning
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&*self.db)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let _test_val = row.0;

        Ok(GetDashboardSummaryResponse {
            metrics,
            recent_activities: vec![],
            recommendations: vec![],
        })
    }

    pub async fn get_financial_report(&self, req: GetFinancialReportRequest) -> Result<GetFinancialReportResponse, tonic::Status> {
        let _org_id = req.organization_id.clone();

        let report = FinancialReport {
            period_start: "2023-10-01".to_string(),
            period_end: "2023-10-31".to_string(),
            total_revenue: 12500.0,
            total_expenses: 4200.0,
            net_profit: 8300.0,
            revenue_by_category: std::collections::HashMap::from([
                ("services".to_string(), 8000.0),
                ("products".to_string(), 4500.0),
            ]),
            expense_by_category: std::collections::HashMap::from([
                ("marketing".to_string(), 1200.0),
                ("software".to_string(), 300.0),
                ("supplies".to_string(), 2700.0),
            ]),
        };

        Ok(GetFinancialReportResponse {
            report: Some(report),
        })
    }
}
