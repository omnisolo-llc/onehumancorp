outcome: no_work
issue_title: "[Architectural Design] Customer Identity Resolution & Omnichannel Memory Architecture"
issue_description: |
  Blocked no-work finding. The issue proposes building a new Customer Identity Resolution Engine and Omnichannel Memory architecture, which includes creating new `ohc_customer_aliases` tables and an `IdentityResolutionService`. However, per the 2026-09-18-usage-audit revision and RESEARCH.md scope gate, new epics and capabilities require an explicit evidence-backed decision. "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." Currently, there is no owner need, reuse analysis, or usage economics evidence provided to authorize this expansion. The exclusive digital-service first market is suspended. We must verify existing business logic and tenant-isolation paths before creating another subsystem.
issue_priority: P2
issue_category: research
issue_type: epic
issue_label: draft
assignees: []
