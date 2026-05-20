
use ::server_ohc::organization::Organization;
use ::server_ohc::organization::TeamMember;
use ::server_ohc::organization::RoleProfile;
use ::server_ohc::common::Role;
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
            id: format!("{}-marketing-1", id),
            name: "Marketing Manager".to_string(),
            role: Role::MarketingManager as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-designer-1", id),
            name: "UI/UX Designer".to_string(),
            role: Role::Designer as i32,
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
            id: format!("{}-swe-2", id),
            name: "Software Engineer 2".to_string(),
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
        TeamMember {
            id: format!("{}-security-1", id),
            name: "Security Engineer".to_string(),
            role: Role::SecurityEngineer as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-news-1", id),
            name: "AI News Collector".to_string(),
            role: Role::AiNewsCollector as i32,
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
        role_profiles: vec![],
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
        TeamMember {
            id: format!("{}-content-1", id),
            name: "Content Strategist".to_string(),
            role: Role::ContentStrategist as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-seo-1", id),
            name: "SEO Specialist".to_string(),
            role: Role::SeoSpecialist as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-media-1", id),
            name: "Paid Media Manager".to_string(),
            role: Role::PaidMediaManager as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-analytics-1", id),
            name: "Analytics Engineer".to_string(),
            role: Role::AnalyticsEngineer as i32,
            is_human: false,
            manager_id: director_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-designer-1", id),
            name: "Creative Designer".to_string(),
            role: Role::Designer as i32,
            is_human: false,
            manager_id: ceo_id.clone(),
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
        TeamMember {
            id: format!("{}-bookkeeper-2", id),
            name: "Bookkeeper 2".to_string(),
            role: Role::Bookkeeper as i32,
            is_human: false,
            manager_id: cfo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-tax-1", id),
            name: "Tax Specialist".to_string(),
            role: Role::TaxSpecialist as i32,
            is_human: false,
            manager_id: cfo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-audit-1", id),
            name: "Audit Manager".to_string(),
            role: Role::AuditManager as i32,
            is_human: false,
            manager_id: cfo_id.clone(),
            organization_id: id.to_string(),
        },
        TeamMember {
            id: format!("{}-payroll-1", id),
            name: "Payroll Manager".to_string(),
            role: Role::PayrollManager as i32,
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



