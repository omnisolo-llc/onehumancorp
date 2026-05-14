use std::sync::RwLock;
use opentelemetry::global;
use opentelemetry::metrics::Counter;

pub struct ViralLoopTracker {
    invites_sent: RwLock<i32>,
    invites_accepted: RwLock<i32>,
    invites_sent_metric: Counter<u64>,
    invites_accepted_metric: Counter<u64>,
}

impl ViralLoopTracker {
    pub fn new() -> Self {
        let meter = global::meter("ohc.growth");
        let invites_sent_metric = meter.u64_counter("ohc.growth.viral_loop.invites_sent").build();
        let invites_accepted_metric = meter.u64_counter("ohc.growth.viral_loop.invites_accepted").build();

        ViralLoopTracker {
            invites_sent: RwLock::new(0),
            invites_accepted: RwLock::new(0),
            invites_sent_metric,
            invites_accepted_metric,
        }
    }

    pub fn record_invite_sent(&self, _user_id: &str) {
        let mut sent = self.invites_sent.write().unwrap();
        *sent += 1;
        self.invites_sent_metric.add(1, &[]);
    }

    pub fn record_invite_accepted(&self, _invitee_id: &str) {
        let mut accepted = self.invites_accepted.write().unwrap();
        *accepted += 1;
        self.invites_accepted_metric.add(1, &[]);
    }

    pub fn calculate_k_factor(&self) -> f64 {
        let sent = self.invites_sent.read().unwrap();
        let accepted = self.invites_accepted.read().unwrap();

        if *sent == 0 {
            return 0.0;
        }

        *accepted as f64 / *sent as f64
    }
}

pub struct OpenGraphCard {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub site_name: String,
    pub url: String,
}

impl OpenGraphCard {
    pub fn generate_html(&self) -> String {
        format!(
            r#"<meta property="og:title" content="{}" />
<meta property="og:description" content="{}" />
<meta property="og:image" content="{}" />
<meta property="og:site_name" content="{}" />
<meta property="og:url" content="{}" />
<meta name="twitter:card" content="summary_large_image">"#,
            self.title, self.description, self.image_url, self.site_name, self.url
        )
    }
}

pub fn get_viral_footer() -> String {
    r#"<footer style="margin-top: 50px; padding: 20px; text-align: center; border-top: 1px solid #eee; font-size: 14px; color: #666;">
    Built with <a href="https://ohc.app" style="color: #0055ff; text-decoration: none; font-weight: 600;">OneHumanCorp</a> — Start your free business →
</footer>"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viral_loop_tracker() {
        let tracker = ViralLoopTracker::new();
        
        tracker.record_invite_sent("user1");
        tracker.record_invite_sent("user2");
        tracker.record_invite_accepted("invitee1");
        
        let k_factor = tracker.calculate_k_factor();
        assert_eq!(k_factor, 0.5);
    }
}
