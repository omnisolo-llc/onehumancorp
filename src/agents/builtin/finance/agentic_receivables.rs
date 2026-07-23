use sqlx::{Pool, Postgres};
use uuid::Uuid;
use anyhow::Result;

pub struct ReceivablesAgent {
    db: Pool<Postgres>,
}

impl ReceivablesAgent {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn check_overdue_invoices(&self, tenant_id: &str) -> Result<()> {
        // Find overdue invoices
        let overdue_invoices = sqlx::query!(
            "SELECT id FROM invoices WHERE tenant_id = $1 AND status = 'sent' AND due_date < extract(epoch from now())",
            tenant_id
        )
        .fetch_all(&self.db)
        .await?;

        for invoice in overdue_invoices {
            let action_id = Uuid::new_v4().to_string();
            // Draft reminder logic goes here...

            // Log action
            sqlx::query!(
                "INSERT INTO receivables_actions (id, tenant_id, invoice_id, action) VALUES ($1, $2, $3, $4)",
                action_id,
                tenant_id,
                invoice.id,
                "Drafted overdue reminder"
            )
            .execute(&self.db)
            .await?;
        }

        Ok(())
    }
}
