use serde::{Serialize, Deserialize};

/// A trait for types that can be optimized for mobile clients by stripping heavy or redundant fields.
pub trait MobileShaper {
    fn optimize_for_mobile(&mut self);
}

impl MobileShaper for ::server_ohc::app::DashboardSnapshot {
    fn optimize_for_mobile(&mut self) {
        // Strip large meeting transcripts
        for meeting in &mut self.meetings {
            meeting.transcript.clear();
        }

        // Strip verbose agent details
        for agent in &mut self.agents {
            agent.name = String::new(); // Rely on ID or icon on mobile
        }

        // Strip non-essential product metadata
        for product in &mut self.products {
            product.description = String::new();
            product.metadata_json = String::new();
            product.fulfillment_strategy = String::new();
            product.currency = String::new();
        }

        // Strip verbose order info
        for order in &mut self.orders {
            order.product_id = String::new();
            order.status = String::new();
            order.organization_id = String::new();
        }

        // Strip detailed organization members/profiles
        if let Some(ref mut org) = self.organization {
            org.domain = String::new();
            org.members = vec![];
            org.role_profiles = vec![];
            org.ceo_id = String::new();
            org.created_at_unix = 0;
        }
    }
}

impl MobileShaper for ::server_ohc::app::GetOnboardingStateResponse {
    fn optimize_for_mobile(&mut self) {
        if let Some(ref mut state) = self.state {
             // Example: strip verbose state_json for mobile if it's too large
             if state.state_json.len() > 1024 {
                 // For now, keep it
             }
        }
    }
}

impl MobileShaper for ::server_ohc::orchestration::PollTasksResponse {
    fn optimize_for_mobile(&mut self) {
        for task in &mut self.tasks {
            task.description = String::new();
            task.payload = String::new();
            task.proposed_content = String::new();
        }
    }
}

/// Helper to apply shaping if requested
pub fn shape_if_needed<T: MobileShaper>(payload: &mut T, mobile_optimized: bool) {
    if mobile_optimized {
        payload.optimize_for_mobile();
    }
}
