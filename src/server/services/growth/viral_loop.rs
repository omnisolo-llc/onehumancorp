use opentelemetry::global;
use opentelemetry::metrics::Counter;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ViralLoopTracker {
    invites_sent_metric: Counter<u64>,
    invites_accepted_metric: Counter<u64>,
    pool: PgPool,
}

impl ViralLoopTracker {
    pub fn new(pool: PgPool) -> Self {
        let meter = global::meter("ohc.growth");
        let invites_sent_metric = meter.u64_counter("ohc.growth.viral_loop.invites_sent").build();
        let invites_accepted_metric = meter.u64_counter("ohc.growth.viral_loop.invites_accepted").build();

        ViralLoopTracker {
            invites_sent_metric,
            invites_accepted_metric,
            pool,
        }
    }

    pub fn record_invite_sent(&self, _user_id: &str) {
        self.invites_sent_metric.add(1, &[]);
    }

    pub fn record_invite_accepted(&self, _invitee_id: &str) {
        self.invites_accepted_metric.add(1, &[]);
    }

    pub async fn get_metrics(&self, tenant_id: Uuid) -> (i32, i32) {
        // Query both team_invites and referrals
        let team_invites_sent: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM team_invites WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let team_invites_accepted: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM team_invites WHERE tenant_id = $1 AND status = 'accepted'")
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let referrals_sent: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM referrals WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let referrals_accepted: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM referrals WHERE tenant_id = $1 AND status = 'accepted'")
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        ((team_invites_sent + referrals_sent) as i32, (team_invites_accepted + referrals_accepted) as i32)
    }

    pub async fn calculate_k_factor(&self, tenant_id: Uuid) -> f64 {
        let (sent, accepted) = self.get_metrics(tenant_id).await;
        if sent == 0 {
            0.0
        } else {
            accepted as f64 / sent as f64
        }
    }
}
