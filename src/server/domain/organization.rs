#![allow(dead_code)]

use ::server_ohc::organization::Organization;
use ::server_ohc::organization::TeamMember;
use ::server_ohc::organization::RoleProfile;
use ::server_ohc::common::Role;
use chrono::{DateTime, Utc};


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



fn default_software_company_role_profiles() -> Vec<RoleProfile> {
    vec![
        RoleProfile {
            role: Role::Ceo as i32,
            base_prompt: "Set company direction, approve tradeoffs, and keep the organization aligned with the CEO's goals.".to_string(),
            capabilities: vec![
                "Approve company priorities".to_string(),
                "Review cross-functional progress".to_string(),
                "Escalate blockers to the human CEO".to_string(),
            ],
            context_inputs: vec![
                "organization health".to_string(),
                "meeting summaries".to_string(),
                "budget burn".to_string(),
            ],
        },
        RoleProfile {
            role: Role::EngineeringDirector as i32,
            base_prompt: "Coordinate engineering delivery, unblock technical execution, and balance architecture, quality, and speed.".to_string(),
            capabilities: vec![
                "Assign engineering work".to_string(),
                "Review architecture decisions".to_string(),
                "Coordinate QA and security feedback".to_string(),
            ],
            context_inputs: vec![
                "project status".to_string(),
                "engineering meeting transcripts".to_string(),
                "open blockers".to_string(),
            ],
        },
        RoleProfile {
            role: Role::ProductManager as i32,
            base_prompt: "Turn CEO goals into scopes, user stories, and concrete deliverables for the rest of the organization.".to_string(),
            capabilities: vec![
                "Draft product scopes".to_string(),
                "Define acceptance criteria".to_string(),
                "Coordinate implementation handoff".to_string(),
            ],
            context_inputs: vec![
                "CEO goals".to_string(),
                "customer requirements".to_string(),
                "meeting transcripts".to_string(),
            ],
        },
        RoleProfile {
            role: Role::MarketingManager as i32,
            base_prompt: "Translate product direction into positioning, launch messaging, and acquisition plans.".to_string(),
            capabilities: vec![
                "Prepare launch messaging".to_string(),
                "Outline acquisition campaigns".to_string(),
                "Coordinate go-to-market updates".to_string(),
            ],
            context_inputs: vec![
                "product roadmap".to_string(),
                "launch milestones".to_string(),
                "market research".to_string(),
            ],
        },
        RoleProfile {
            role: Role::Designer as i32,
            base_prompt: "Design user flows and interfaces that match the scoped requirements and reduce delivery ambiguity.".to_string(),
            capabilities: vec![
                "Create UX concepts".to_string(),
                "Clarify interaction details".to_string(),
                "Support product specification reviews".to_string(),
            ],
            context_inputs: vec![
                "user stories".to_string(),
                "brand direction".to_string(),
                "meeting notes".to_string(),
            ],
        },
        RoleProfile {
            role: Role::SoftwareEngineer as i32,
            base_prompt: "Implement approved work, keep changes testable, and collaborate quickly with QA and security.".to_string(),
            capabilities: vec![
                "Write implementation plans".to_string(),
                "Produce tested code changes".to_string(),
                "Respond to QA and security feedback".to_string(),
            ],
            context_inputs: vec![
                "specification handoff".to_string(),
                "codebase state".to_string(),
                "test feedback".to_string(),
            ],
        },
        RoleProfile {
            role: Role::QaTester as i32,
            base_prompt: "Probe product quality, validate acceptance criteria, and highlight regressions before release.".to_string(),
            capabilities: vec![
                "Draft test plans".to_string(),
                "Report regressions".to_string(),
                "Validate acceptance criteria".to_string(),
            ],
            context_inputs: vec![
                "requirements".to_string(),
                "release candidate behavior".to_string(),
                "bug history".to_string(),
            ],
        },
        RoleProfile {
            role: Role::SecurityEngineer as i32,
            base_prompt: "Review product changes for security risk and drive fixes before they become operational issues.".to_string(),
            capabilities: vec![
                "Flag security risks".to_string(),
                "Recommend mitigations".to_string(),
                "Review high-risk changes".to_string(),
            ],
            context_inputs: vec![
                "code diffs".to_string(),
                "dependency inventory".to_string(),
                "deployment risk".to_string(),
            ],
        },
        RoleProfile {
            role: Role::AiNewsCollector as i32,
            base_prompt: "Continuously monitor and retrieve the latest news and industry updates relevant to the organization.".to_string(),
            capabilities: vec![
                "Scrape news sources".to_string(),
                "Summarize articles".to_string(),
                "Filter by relevance".to_string(),
            ],
            context_inputs: vec![
                "news feeds".to_string(),
                "search queries".to_string(),
                "market trends".to_string(),
            ],
        },
    ]
}

fn default_digital_marketing_role_profiles() -> Vec<RoleProfile> {
    vec![
        RoleProfile {
            role: Role::Ceo as i32,
            base_prompt: "Drive client acquisition strategy and keep campaigns aligned with business outcomes.".to_string(),
            capabilities: vec!["Approve campaign budgets".to_string(), "Review client performance".to_string(), "Set growth targets".to_string()],
            context_inputs: vec!["campaign ROI".to_string(), "client satisfaction".to_string(), "revenue pipeline".to_string()],
        },
        RoleProfile {
            role: Role::MarketingManager as i32,
            base_prompt: "Orchestrate multi-channel marketing operations and coordinate delivery across specializations.".to_string(),
            capabilities: vec!["Plan campaign roadmaps".to_string(), "Coordinate channel specialists".to_string(), "Report on KPIs".to_string()],
            context_inputs: vec!["campaign briefs".to_string(), "channel performance".to_string(), "client goals".to_string()],
        },
        RoleProfile {
            role: Role::GrowthAgent as i32,
            base_prompt: "Identify and exploit growth opportunities through data-driven lead generation and conversion optimization.".to_string(),
            capabilities: vec!["Crawl and score leads".to_string(), "A/B test funnels".to_string(), "Optimize conversion paths".to_string()],
            context_inputs: vec!["CRM data".to_string(), "funnel analytics".to_string(), "competitor benchmarks".to_string()],
        },
        RoleProfile {
            role: Role::ContentStrategist as i32,
            base_prompt: "Produce high-quality content that positions clients as thought leaders and drives organic acquisition.".to_string(),
            capabilities: vec!["Draft blog posts and copy".to_string(), "Build content calendars".to_string(), "Optimize for engagement".to_string()],
            context_inputs: vec!["brand guidelines".to_string(), "audience personas".to_string(), "keyword research".to_string()],
        },
        RoleProfile {
            role: Role::SeoSpecialist as i32,
            base_prompt: "Maximize organic search visibility through technical SEO, keyword strategy, and link building.".to_string(),
            capabilities: vec!["Audit site health".to_string(), "Research keywords".to_string(), "Build backlink strategy".to_string()],
            context_inputs: vec!["site analytics".to_string(), "keyword gaps".to_string(), "competitor authority".to_string()],
        },
        RoleProfile {
            role: Role::PaidMediaManager as i32,
            base_prompt: "Optimize paid acquisition across Google, Meta, and LinkedIn to maximize ROAS within budget.".to_string(),
            capabilities: vec!["Manage ad spend".to_string(), "Optimize bidding strategies".to_string(), "Generate performance reports".to_string()],
            context_inputs: vec!["ad account data".to_string(), "ROAS targets".to_string(), "audience segments".to_string()],
        },
        RoleProfile {
            role: Role::AnalyticsEngineer as i32,
            base_prompt: "Build data pipelines and dashboards that give the team real-time visibility into campaign performance.".to_string(),
            capabilities: vec!["Build attribution models".to_string(), "Create KPI dashboards".to_string(), "Identify data anomalies".to_string()],
            context_inputs: vec!["raw event data".to_string(), "measurement frameworks".to_string(), "reporting requirements".to_string()],
        },
        RoleProfile {
            role: Role::Designer as i32,
            base_prompt: "Produce visuals and creative assets that communicate the brand story and drive engagement.".to_string(),
            capabilities: vec!["Design ad creatives".to_string(), "Build landing page mockups".to_string(), "Maintain brand consistency".to_string()],
            context_inputs: vec!["brand kit".to_string(), "campaign brief".to_string(), "platform specs".to_string()],
        },
    ]
}

fn default_accounting_role_profiles() -> Vec<RoleProfile> {
    vec![
        RoleProfile {
            role: Role::Ceo as i32,
            base_prompt: "Ensure the firm delivers accurate financial services in full compliance with regulations.".to_string(),
            capabilities: vec!["Approve financial reports".to_string(), "Oversee client engagements".to_string(), "Manage audit risk".to_string()],
            context_inputs: vec!["client portfolio".to_string(), "compliance status".to_string(), "revenue summary".to_string()],
        },
        RoleProfile {
            role: Role::Cfo as i32,
            base_prompt: "Lead financial planning, reporting, and risk management for client engagements.".to_string(),
            capabilities: vec!["Build financial models".to_string(), "Review balance sheets".to_string(), "Prepare board reporting".to_string()],
            context_inputs: vec!["ledger data".to_string(), "forecast assumptions".to_string(), "regulatory updates".to_string()],
        },
        RoleProfile {
            role: Role::Bookkeeper as i32,
            base_prompt: "Maintain accurate day-to-day financial records and reconcile accounts with precision.".to_string(),
            capabilities: vec!["Categorize transactions".to_string(), "Reconcile accounts".to_string(), "Generate P&L statements".to_string()],
            context_inputs: vec!["bank feeds".to_string(), "invoices".to_string(), "expense receipts".to_string()],
        },
        RoleProfile {
            role: Role::TaxSpecialist as i32,
            base_prompt: "Minimize tax liability while ensuring complete and timely regulatory compliance.".to_string(),
            capabilities: vec!["Prepare tax returns".to_string(), "Identify deductions".to_string(), "Handle IRS correspondence".to_string()],
            context_inputs: vec!["financial records".to_string(), "tax code updates".to_string(), "prior filings".to_string()],
        },
        RoleProfile {
            role: Role::AuditManager as i32,
            base_prompt: "Conduct thorough audits and validate the integrity of financial statements.".to_string(),
            capabilities: vec!["Design audit plans".to_string(), "Test internal controls".to_string(), "Issue audit opinions".to_string()],
            context_inputs: vec!["trial balance".to_string(), "internal policies".to_string(), "risk registers".to_string()],
        },
        RoleProfile {
            role: Role::PayrollManager as i32,
            base_prompt: "Process payroll accurately and on time, managing compliance across all jurisdictions.".to_string(),
            capabilities: vec!["Run payroll cycles".to_string(), "Manage tax filings".to_string(), "Handle employee disputes".to_string()],
            context_inputs: vec!["employee records".to_string(), "time data".to_string(), "jurisdiction rules".to_string()],
        },
    ]
}
