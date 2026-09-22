issue_title: "Documenting Customer Proposals and Project Scope Workflow"
issue_description: |
  # Customer Proposals and Project Scope Workflow

  **Source Date:** 2026-09-18
  **Study Population:** Non-technical owners/operators configuring business operations, particularly in service loops (inquiry to fulfillment).
  **Metric Definitions:** None explicitly measured in this report; this establishes the documented state of the current capability.
  **Scope:** Documents how an owner accomplishes verified business work regarding proposals, specifically drafting, persistence, and progression to payment.

  ## Overview
  This documentation explains how owners use the OmniSolo system to define project scope and issue proposals to clients. The documentation is based on the actual, current capabilities of the platform as verified in the `business_capability_and_usage_economics_audit.md` (Revision: 2026-09-18-usage-audit).

  ## Current Capabilities
  OmniSolo provides a route to handle customer proposals and project scopes (`api/proposals.rs`). The current active features are:
  - **Tenant/Customer Records:** The system supports persistent records for tenants and customers.
  - **Proposal Structuring:** Proposals can include exact cent amounts, requested deposits, defined scope, and milestones.
  - **Lifecycle States:** Proposals move through `draft`, `intake`, and `approval` states.

  ## Setting up a Proposal
  1. **Intake and Definition:** Currently, the system establishes a defined scope and price when moving from intake to a proposal (`proposals.rs:815-845`). Note that while intake exists, the generation process might apply fixed scope definitions.
  2. **Approval and Payment Linking:** Once a proposal is approved, it calls the `StripeClient::create_checkout_session` (`proposals.rs:397-426`) to move toward payment collection.

  ## Evidence & Exceptions
  - **Evidence:** The current implementation persists proposal and invoice line items. Approvals bridge to payment APIs.
  - **Exceptions & Unverified Paths:** As noted in the audit, while a milestone schema exists, it is not currently verified against actual service fulfillment. The intake process might ignore custom inquiries in favor of constructing fixed scope and price, requiring owner review before finalization.

  ## Superpowers loaded
  - Using Superpowers
  - Brainstorming
  - Writing Plans
  - (Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71)
issue_priority: "P1"
issue_category: "documentation"
issue_type: "documentation"
issue_label: ["ohc:lane:documentation", "ohc:journey:J1"]
assignees: ["scribe-agent"]
