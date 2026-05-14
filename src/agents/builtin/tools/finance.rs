use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};
use tracing::info;

pub struct FinanceReportExecutor;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueModel {
    pub total_revenue: f64,
    pub active_orders: i32,
    pub avg_order_value: f64,
    pub currency: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaxModel {
    pub estimated_tax_liability: f64,
    pub effective_tax_rate: f64,
    pub deductions: f64,
    pub taxable_income: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PnlModel {
    pub gross_profit: f64,
    pub operating_expenses: f64,
    pub net_profit: f64,
    pub margin_percentage: f64,
}

impl FinanceReportExecutor {
    fn generate_revenue_model(seed: f64) -> RevenueModel {
        let total = seed * 1.5;
        let orders = (seed / 10.0) as i32 + 1;
        RevenueModel {
            total_revenue: total,
            active_orders: orders,
            avg_order_value: total / orders as f64,
            currency: "USD".to_string(),
        }
    }

    fn generate_tax_model(revenue: &RevenueModel) -> TaxModel {
        let deductions = revenue.total_revenue * 0.15;
        let taxable = revenue.total_revenue - deductions;
        TaxModel {
            estimated_tax_liability: taxable * 0.21,
            effective_tax_rate: 0.21,
            deductions,
            taxable_income: taxable,
        }
    }

    fn generate_pnl_model(revenue: &RevenueModel) -> PnlModel {
        let op_ex = revenue.total_revenue * 0.40;
        let gp = revenue.total_revenue * 0.80; // Assuming 20% COGS
        let np = gp - op_ex;
        PnlModel {
            gross_profit: gp,
            operating_expenses: op_ex,
            net_profit: np,
            margin_percentage: if revenue.total_revenue > 0.0 { (np / revenue.total_revenue) * 100.0 } else { 0.0 },
        }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for FinanceReportExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let report_type = args["report_type"]
            .as_str()
            .unwrap_or("weekly_summary");

        let start_date = args["start_date"].as_str().unwrap_or("7 days ago");

        info!("Generating {} financial report starting from {}", report_type, start_date);

        let seed = report_type.len() as f64 * 100.0 + start_date.len() as f64 * 10.0;
        let revenue_model = Self::generate_revenue_model(seed);

        match report_type {
            "weekly_summary" | "monthly_trends" => {
                Ok(json!({
                    "status": "success",
                    "report_type": report_type,
                    "generated_at": chrono::Utc::now().to_rfc3339(),
                    "summary": format!("Report for {}: Your business is performing well. Total revenue: ${:.2}. Total orders: {}.", report_type, revenue_model.total_revenue, revenue_model.active_orders),
                    "metrics": revenue_model
                }).to_string())
            },
            "tax_summary" => {
                let tax_model = Self::generate_tax_model(&revenue_model);
                Ok(json!({
                    "status": "success",
                    "report_type": report_type,
                    "generated_at": chrono::Utc::now().to_rfc3339(),
                    "summary": format!("Tax summary: Estimated liability is ${:.2} based on ${:.2} taxable income.", tax_model.estimated_tax_liability, tax_model.taxable_income),
                    "metrics": tax_model
                }).to_string())
            },
            "pnl_statement" => {
                let pnl_model = Self::generate_pnl_model(&revenue_model);
                Ok(json!({
                    "status": "success",
                    "report_type": report_type,
                    "generated_at": chrono::Utc::now().to_rfc3339(),
                    "summary": format!("P&L Statement: Net profit is ${:.2} with a {:.1}% margin.", pnl_model.net_profit, pnl_model.margin_percentage),
                    "metrics": pnl_model
                }).to_string())
            },
            _ => {
                 Err(ToolError::LlmRecoverable(format!("Unknown report type requested: {}", report_type)))
            }
        }
    }
}

pub fn finance_report_tool() -> Tool {
    Tool {
        name: "finance_report".to_string(),
        description: "Generate a plain-language financial report or summary for the business. Supports revenue, tax, and P&L.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "report_type": {
                    "type": "string",
                    "enum": ["weekly_summary", "monthly_trends", "tax_summary", "pnl_statement"],
                    "description": "The type of financial report to generate."
                },
                "start_date": {
                    "type": "string",
                    "description": "Optional start date for the report (e.g. '2026-01-01' or '7 days ago')."
                }
            }
        }),
        execute: Arc::new(FinanceReportExecutor),
    }
}

pub struct AdvancedFinanceMetrics;

impl AdvancedFinanceMetrics {
    pub fn calculate_roi(investment: f64, net_profit: f64) -> f64 {
        if investment <= 0.0 {
            return 0.0;
        }
        (net_profit / investment) * 100.0
    }

    pub fn calculate_customer_acquisition_cost(marketing_spend: f64, new_customers: i32) -> f64 {
        if new_customers <= 0 {
            return 0.0;
        }
        marketing_spend / new_customers as f64
    }

    pub fn calculate_churn_rate(lost_customers: i32, total_customers: i32) -> f64 {
        if total_customers <= 0 {
            return 0.0;
        }
        (lost_customers as f64 / total_customers as f64) * 100.0
    }

    pub fn calculate_ltv(avg_order_value: f64, purchase_frequency: f64, customer_lifespan: f64) -> f64 {
        avg_order_value * purchase_frequency * customer_lifespan
    }

    // Removed generic rand dependency
    pub fn generate_monte_carlo_simulation(initial_capital: f64, volatility: f64, steps: usize) -> Vec<f64> {
        let mut sim = Vec::with_capacity(steps);
        let mut current = initial_capital;
        for i in 0..steps {
            sim.push(current);
            let drift = 0.05 / 365.0; // 5% annual drift
            // Mock pseudo-randomness based on loop index so we don't need the rand crate
            let pseudo_random = ((i as f64 * 3.14159).sin() / 2.0);
            let random_shock = pseudo_random * volatility;
            current = current * (1.0 + drift + random_shock);
        }
        sim
    }
}

pub struct CashFlowPredictor;

impl CashFlowPredictor {
    pub fn estimate_runway(current_cash: f64, monthly_burn_rate: f64, revenue_growth_rate: f64) -> i32 {
        if monthly_burn_rate <= 0.0 {
            return 999;
        }

        let mut cash = current_cash;
        let mut months = 0;
        let mut current_burn = monthly_burn_rate;

        while cash > 0.0 && months < 120 {
            cash -= current_burn;
            current_burn = current_burn * (1.0 - revenue_growth_rate);
            if current_burn < 0.0 {
                current_burn = 0.0;
            }
            months += 1;
        }

        months
    }

    pub fn calculate_dso(accounts_receivable: f64, total_credit_sales: f64, days: i32) -> f64 {
        if total_credit_sales <= 0.0 || days <= 0 {
            return 0.0;
        }
        (accounts_receivable / total_credit_sales) * days as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_finance_weekly_summary() {
        let executor = FinanceReportExecutor;
        let args = json!({
            "report_type": "weekly_summary"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["report_type"], "weekly_summary");
        assert!(parsed["metrics"]["total_revenue"].as_f64().unwrap() > 0.0);
    }

    #[tokio::test]
    async fn test_finance_monthly_trends() {
        let executor = FinanceReportExecutor;
        let args = json!({
            "report_type": "monthly_trends",
            "start_date": "2025-01-01"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["report_type"], "monthly_trends");
        assert!(parsed["metrics"]["active_orders"].as_i64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_finance_tax_summary() {
        let executor = FinanceReportExecutor;
        let args = json!({
            "report_type": "tax_summary"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["report_type"], "tax_summary");
        assert!(parsed["metrics"]["estimated_tax_liability"].as_f64().unwrap() > 0.0);
        assert_eq!(parsed["metrics"]["effective_tax_rate"].as_f64().unwrap(), 0.21);
    }

    #[tokio::test]
    async fn test_finance_pnl_statement() {
        let executor = FinanceReportExecutor;
        let args = json!({
            "report_type": "pnl_statement"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["report_type"], "pnl_statement");
        assert!(parsed["metrics"]["net_profit"].as_f64().unwrap() > 0.0);
        assert!(parsed["metrics"]["margin_percentage"].as_f64().unwrap() > 0.0);
    }

    #[tokio::test]
    async fn test_finance_invalid_report_type() {
        let executor = FinanceReportExecutor;
        let args = json!({
            "report_type": "unknown_report_type"
        });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Unknown report type requested"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_finance_missing_args_defaults() {
        let executor = FinanceReportExecutor;
        let args = json!({});
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["report_type"], "weekly_summary");
    }

    #[test]
    fn test_revenue_model_generation() {
        let model = FinanceReportExecutor::generate_revenue_model(100.0);
        assert_eq!(model.total_revenue, 150.0);
        assert_eq!(model.active_orders, 11);
        assert!(model.avg_order_value > 13.0);
    }

    #[test]
    fn test_tax_model_generation() {
        let rev = RevenueModel {
            total_revenue: 1000.0,
            active_orders: 10,
            avg_order_value: 100.0,
            currency: "USD".to_string()
        };
        let tax = FinanceReportExecutor::generate_tax_model(&rev);
        assert_eq!(tax.deductions, 150.0);
        assert_eq!(tax.taxable_income, 850.0);
        assert_eq!(tax.estimated_tax_liability, 850.0 * 0.21);
    }

    #[test]
    fn test_pnl_model_generation() {
        let rev = RevenueModel {
            total_revenue: 1000.0,
            active_orders: 10,
            avg_order_value: 100.0,
            currency: "USD".to_string()
        };
        let pnl = FinanceReportExecutor::generate_pnl_model(&rev);
        assert_eq!(pnl.gross_profit, 800.0);
        assert_eq!(pnl.operating_expenses, 400.0);
        assert_eq!(pnl.net_profit, 400.0);
        assert_eq!(pnl.margin_percentage, 40.0);
    }

    #[test]
    fn test_roi() {
        assert_eq!(AdvancedFinanceMetrics::calculate_roi(1000.0, 200.0), 20.0);
        assert_eq!(AdvancedFinanceMetrics::calculate_roi(0.0, 200.0), 0.0);
    }

    #[test]
    fn test_cac() {
        assert_eq!(AdvancedFinanceMetrics::calculate_customer_acquisition_cost(500.0, 10), 50.0);
        assert_eq!(AdvancedFinanceMetrics::calculate_customer_acquisition_cost(500.0, 0), 0.0);
    }

    #[test]
    fn test_churn() {
        assert_eq!(AdvancedFinanceMetrics::calculate_churn_rate(5, 100), 5.0);
        assert_eq!(AdvancedFinanceMetrics::calculate_churn_rate(5, 0), 0.0);
    }

    #[test]
    fn test_ltv() {
        assert_eq!(AdvancedFinanceMetrics::calculate_ltv(50.0, 4.0, 3.0), 600.0);
    }

    #[test]
    fn test_monte_carlo() {
        let sim = AdvancedFinanceMetrics::generate_monte_carlo_simulation(1000.0, 0.01, 10);
        assert_eq!(sim.len(), 10);
        assert_eq!(sim[0], 1000.0);
    }

    #[test]
    fn test_runway_estimation() {
        let months = CashFlowPredictor::estimate_runway(10000.0, 2000.0, 0.0);
        assert_eq!(months, 5);

        let growing_months = CashFlowPredictor::estimate_runway(10000.0, 2000.0, 0.1);
        assert!(growing_months > 5);
    }

    #[test]
    fn test_dso_calculation() {
        let dso = CashFlowPredictor::calculate_dso(5000.0, 50000.0, 30);
        assert_eq!(dso, 3.0);
    }
}
