use std::sync::RwLock;

#[derive(Clone, Debug)]
pub struct MilestoneNotification {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub is_read: bool,
}

pub struct MilestoneManager {
    notifications: RwLock<Vec<MilestoneNotification>>,
}

impl MilestoneManager {
    pub fn new() -> Self {
        MilestoneManager {
            notifications: RwLock::new(Vec::new()),
        }
    }

    pub fn trigger_milestone(&self, user_id: &str, milestone_type: &str, value: i32) {
        let message = match milestone_type {
            "orders" => format!("🎉 You just got your {}th order!", value),
            "visitors" => format!("🚀 Your store has {} visitors today!", value),
            _ => format!("🎉 You reached a new milestone: {} {}", value, milestone_type),
        };

        let notification = MilestoneNotification {
            id: format!("m-{}-{}", user_id, value),
            user_id: user_id.to_string(),
            message,
            is_read: false,
        };

        let mut list = self.notifications.write().unwrap();
        list.push(notification);
    }

    pub fn get_unread_notifications(&self, user_id: &str) -> Vec<MilestoneNotification> {
        let list = self.notifications.read().unwrap();
        list.iter()
            .filter(|n| n.user_id == user_id && !n.is_read)
            .cloned()
            .collect()
    }

    pub fn mark_as_read(&self, notification_id: &str) {
        let mut list = self.notifications.write().unwrap();
        if let Some(n) = list.iter_mut().find(|n| n.id == notification_id) {
            n.is_read = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milestone_flow() {
        let manager = MilestoneManager::new();
        manager.trigger_milestone("user1", "orders", 10);
        manager.trigger_milestone("user1", "visitors", 100);

        let unread = manager.get_unread_notifications("user1");
        assert_eq!(unread.len(), 2);
        assert_eq!(unread[0].message, "🎉 You just got your 10th order!");
        assert_eq!(unread[1].message, "🚀 Your store has 100 visitors today!");

        manager.mark_as_read(&unread[0].id);
        let unread_after = manager.get_unread_notifications("user1");
        assert_eq!(unread_after.len(), 1);
        assert_eq!(unread_after[0].message, "🚀 Your store has 100 visitors today!");
    }
}
