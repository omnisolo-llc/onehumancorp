use std::sync::RwLock;
use opentelemetry::global;
use opentelemetry::metrics::Counter;

#[derive(Clone, Debug)]
pub struct EmailCampaign {
    pub id: String,
    pub user_id: String,
    pub template: String,
    pub status: String,
    pub contacts_count: i32,
}

pub struct EmailCampaignManager {
    campaigns: RwLock<Vec<EmailCampaign>>,
    campaigns_sent_metric: Counter<u64>,
}

impl EmailCampaignManager {
    pub fn new() -> Self {
        let meter = global::meter("ohc.growth.email_campaigns");
        let campaigns_sent_metric = meter.u64_counter("ohc.growth.email_campaigns.sent").build();

        EmailCampaignManager {
            campaigns: RwLock::new(Vec::new()),
            campaigns_sent_metric,
        }
    }

    pub fn create_campaign(&self, id: String, user_id: String, template: String, contacts_count: i32) -> EmailCampaign {
        let campaign = EmailCampaign {
            id,
            user_id,
            template,
            status: "DRAFT".to_string(),
            contacts_count,
        };
        let mut list = self.campaigns.write().unwrap();
        list.push(campaign.clone());
        campaign
    }

    pub fn send_campaign(&self, id: &str) -> Result<EmailCampaign, String> {
        let mut list = self.campaigns.write().unwrap();
        if let Some(c) = list.iter_mut().find(|c| c.id == id) {
            c.status = "SENT".to_string();
            self.campaigns_sent_metric.add(1, &[]);
            return Ok(c.clone());
        }
        Err("Campaign not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_campaign_flow() {
        let manager = EmailCampaignManager::new();
        let c = manager.create_campaign("c1".to_string(), "user1".to_string(), "New arrivals".to_string(), 100);
        assert_eq!(c.status, "DRAFT");

        let c_sent = manager.send_campaign("c1").unwrap();
        assert_eq!(c_sent.status, "SENT");

        let err = manager.send_campaign("nonexistent").unwrap_err();
        assert_eq!(err, "Campaign not found");
    }
}
