use crate::ohc::organization::Organization;
use crate::ohc::organization::TeamMember;
use crate::ohc::organization::RoleProfile;
use crate::ohc::common::Role;
use chrono::{DateTime, Utc};

pub fn new_software_company(id: &str, name: &str, ceo_name: &str, now: DateTime<Utc>) -> Organization {
    let ceo_id = format!("{}-ceo", id);
    let director_id = format!("{}-director-eng", id);

    let members = vec![
        TeamMember {
            id: ceo_id.clone(),
            name: ceo_name.to_string(),
            role: Role::Ceo as i32,
            is_human: true,
            manager_id: "".to_string(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: director_id.clone(),
            name: "Engineering Director".to_string(),
            role: Role::EngineeringDirector as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-pm-1", id),
            name: "Product Manager".to_string(),
            role: Role::ProductManager as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-swe-1", id),
            name: "Software Engineer 1".to_string(),
            role: Role::SoftwareEngineer as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-qa-1", id),
            name: "QA Tester".to_string(),
            role: Role::QaTester as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
    ];

    Organization {
        id: id.to_string(),
        name: name.to_string(),
        domain: "software_company".to_string(),
        ceo_id,
        tier: "Free".to_string(),
        created_at_unix: now.timestamp(),
        members,
        role_profiles: vec![], // TODO: Add default profiles
    }
}

pub trait OrganizationExt {
    fn action_limit(&self) -> i64;
    fn member_by_id(&self, id: &str) -> Option<&TeamMember>;
    fn members_by_manager(&self, manager_id: &str) -> Vec<&TeamMember>;
}

impl OrganizationExt for Organization {
    fn action_limit(&self) -> i64 {
        match self.tier.as_str() {
            "Starter" => 1000,
            "Pro" => -1,
            "Free" | _ => 100,
        }
    }

    fn member_by_id(&self, id: &str) -> Option<&TeamMember> {
        self.members.iter().find(|m| m.id == id)
    }

    fn members_by_manager(&self, manager_id: &str) -> Vec<&TeamMember> {
        self.members.iter().filter(|m| m.manager_id == manager_id).collect()
    }
}

pub fn new_digital_marketing_agency(id: &str, name: &str, ceo_name: &str, now: DateTime<Utc>) -> Organization {
    let ceo_id = format!("{}-ceo", id);
    let director_id = format!("{}-director-mkt", id);

    let members = vec![
        TeamMember {
            id: ceo_id.clone(),
            name: ceo_name.to_string(),
            role: Role::Ceo as i32,
            is_human: true,
            manager_id: "".to_string(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: director_id.clone(),
            name: "Marketing Director".to_string(),
            role: Role::MarketingManager as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-growth-1", id),
            name: "Growth Agent".to_string(),
            role: Role::GrowthAgent as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
    ];

    Organization {
        id: id.to_string(),
        name: name.to_string(),
        domain: "digital_marketing_agency".to_string(),
        ceo_id,
        tier: "Free".to_string(),
        created_at_unix: now.timestamp(),
        members,
        role_profiles: vec![],
    }
}

pub fn new_accounting_firm(id: &str, name: &str, ceo_name: &str, now: DateTime<Utc>) -> Organization {
    let ceo_id = format!("{}-ceo", id);
    let cfo_id = format!("{}-cfo", id);

    let members = vec![
        TeamMember {
            id: ceo_id.clone(),
            name: ceo_name.to_string(),
            role: Role::Ceo as i32,
            is_human: true,
            manager_id: "".to_string(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: cfo_id.clone(),
            name: "Chief Financial Officer".to_string(),
            role: Role::Cfo as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-bookkeeper-1", id),
            name: "Bookkeeper".to_string(),
            role: Role::Bookkeeper as i32,
            is_human: false,
            manager_id: cfo_id.clone(),
            organization_id: id.to_string(),
        },
    ];

    Organization {
        id: id.to_string(),
        name: name.to_string(),
        domain: "accounting_firm".to_string(),
        ceo_id,
        tier: "Free".to_string(),
        created_at_unix: now.timestamp(),
        members,
        role_profiles: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_software_company() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let org = new_software_company("org1", "My Soft Corp", "Alice", now);

        assert_eq!(org.id, "org1");
        assert_eq!(org.name, "My Soft Corp");
        assert_eq!(org.domain, "software_company");
        assert_eq!(org.ceo_id, "org1-ceo");
        assert_eq!(org.members.len(), 5);

        let ceo = org.member_by_id("org1-ceo").unwrap();
        assert_eq!(ceo.name, "Alice");
        assert_eq!(ceo.role, Role::Ceo as i32);
        assert!(ceo.is_human);
    }

    #[test]
    fn test_organization_ext() {
        let now = Utc::now();
        let mut org = new_software_company("org1", "My Soft Corp", "Alice", now);

        assert_eq!(org.action_limit(), 100); // Default Free tier

        org.tier = "Pro".to_string();
        assert_eq!(org.action_limit(), -1);

        let members = org.members_by_manager("org1-ceo");
        assert_eq!(members.len(), 2); // Director and PM
    }
}
