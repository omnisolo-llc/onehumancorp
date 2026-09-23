issue_title: "Repair F07: Prevent Fabricated Proposal Terms and Support OHC-04 Qualification"
issue_description: |
  **Problem Statement**
  Currently, as noted in the audit (F07), `proposals.rs:825-845` creates a fixed $5,000 scope/deposit regardless of the actual inquiry. This violates the core requirement for OHC-04 (Customer inquiry → qualified quote) where a real inbound message must create a grounded, priced quote based on actual business rules rather than hardcoded fabricated terms. For small business owners (like Nora), generating inaccurate or fixed-price proposals automatically undermines trust and could lead to unauthorized commitments or lost business.

  **Research Report**
  - **Source Evidence:** `docs/research/native_migration_and_remediation.md` explicitly lists F07 as an open defect: `proposals.rs:825-845` creates fixed $5,000 scope/deposit regardless of inquiry.
  - **Product Strategy:** According to `RESEARCH.md` (OHC-04), the system must support "Customer inquiry → qualified quote" where a real inbound message creates one lead, grounded response, and priced quote. The audit requires "Input/approved-business-rule driven draft; deterministic validated amounts; no unauthorized commitments or fabricated scope".
  - **Market Context:** Small business operators need proposals that reflect their actual services and pricing. Competitors like HoneyBook allow dynamic, rule-based or template-driven proposal generation. Fabricating a $5,000 proposal is unacceptable for businesses with varying pricing structures.

  **Design Doc**
  - **Architecture:** The proposal generation logic in the backend (likely in `proposals.rs`) needs to be refactored. Instead of hardcoding $5,000, the system should:
    1. Parse the incoming inquiry to identify the requested services.
    2. Match these services against the owner's configured pricing rules or offer catalog (established in OHC-03).
    3. If pricing cannot be determined automatically, the system should draft the proposal but flag it with a `NEEDS_PRICING` state, requiring owner review before sending.
    4. Only include selected items in the committed totals. Calculate the deposit dynamically based on the owner's rules (e.g., a percentage of the total or a fixed amount per service).
  - **AI Agent Integration:** The AI agent responsible for drafting proposals should extract requirements from the inquiry and map them to known services, but the actual pricing calculation and enforcement must be handled by the backend business logic to ensure correctness and prevent unauthorized commitments.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Agent as AI Agent
      participant Backend as Backend System (proposals.rs)
      participant Owner

      Customer->>Agent: Sends Inquiry
      Agent->>Backend: Parses requirements & requests Proposal Draft
      Backend->>Backend: Match requirements to pricing rules
      alt Pricing determined
          Backend->>Backend: Calculate total and dynamic deposit
          Backend->>Agent: Return priced Draft
      else Pricing unknown/complex
          Backend->>Backend: Create Draft with NEEDS_PRICING flag
          Backend->>Owner: Notify Owner for manual review
      end
  ```

  **Implementation Prompt**
  Modify the proposal generation logic in `src/server/services/proposals.rs` (around lines 825-845) to remove the hardcoded $5,000 scope and deposit.
  1. Implement a mechanism to derive the proposal scope and pricing from the actual inquiry and the owner's configured business rules/services.
  2. Use checked integer arithmetic for all currency calculations.
  3. If the system cannot confidently determine the price for the requested services, create the proposal in a draft state flagged with `NEEDS_PRICING`.
  4. Ensure that only explicitly selected/approved items are included in the committed totals.
  5. Calculate the deposit dynamically based on the owner's business rules, rather than a fixed amount.
  6. Add real-stack persistence and isolation test cases to verify this behavior.

  **Priority:** P0 (Blocks OHC-04 and represents a severe correctness/authority defect)
  **Estimated Scope:** Medium

  **Strategy Admission**
  - Target ID: OHC-04 / F07
  - Stage: Launch
  - Observed Gap: Hardcoded proposal values instead of dynamic, rule-based pricing.
  - Evidence Level: Documented in codebase (`proposals.rs:825-845`) and audit ledger.
  - Baseline/Result: Currently 100% of auto-generated proposals are $5,000. Target is 100% of proposals reflect actual configured pricing or enter a `NEEDS_PRICING` state.
  - Dependencies: Owner's offer/pricing configuration (OHC-03).
  - Non-goals: Full visual proposal builder UI in this task.
  - Authority Class: System can draft; owner must approve if `NEEDS_PRICING`; system can send if within standing authority and pricing is fully resolved.
  - Acceptance checks: Verify that different inquiries produce differently priced proposals based on rules; verify that missing rules result in `NEEDS_PRICING`; verify checked arithmetic prevents overflow/errors.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
