issue_title: "Intake inquiry creates fixed scope ignoring owner constraints"
issue_description: |
  **Title**: Replace fixed $5,000 proposal generation with dynamic inquiry-driven quoting

  **Problem Statement**:
  When a potential client submits an inquiry through the intake form, the system completely ignores their actual message and budget constraints, automatically generating a fixed $5,000 project scope (`proposals.rs:815-845`). For Nora (our solo web/design operator), this is unusable—she needs to review the actual client request and have the system propose a quote that reflects her predefined service packages and pricing, not a hardcoded artifact. This destroys trust and prevents closing deals autonomously.

  **Research Report**:
  - Code audit (`docs/research/business_capability_and_usage_economics_audit.md`) confirms that `api/proposals.rs` constructs fixed scope and price for all intakes, disregarding the inquiry.
  - The current workflow breaks the `OHC-04` loop ("Customer inquiry → qualified quote").
  - Competitor comparison: Tools like HoneyBook parse inquiries and suggest templates, but still require manual adjustment. An AI-native solution should ground the quote in the owner's configured packages and the specific customer request.

  **Design Doc**:
  - High-level architecture: The intake router needs to invoke the Inference service (or an agent coordinator) to classify the inquiry against the owner's configured services (established in OHC-03).
  - Integration points: The `intake` route must parse the incoming text, match it to a service catalog, and generate a dynamic `Proposal` entity rather than a hardcoded struct.
  - UI wireframes / Mobile UX flow (375px): The owner receives a notification (e.g., via SMS or push) summarizing the generated proposal and its alignment with their packages. They can click "Approve", "Edit", or "Reject" directly from their phone before the client sees it.

  **Implementation Prompt**:
  Modify the `intake` endpoint in `src/server/api/proposals.rs`. Remove the hardcoded $5,000 scope generation. Instead, implement a function that takes the raw inquiry and the tenant's configured service packages (from the database), uses the LLM to generate a corresponding proposed scope and price, and creates a `Proposal` record in a `Draft` state. The acceptance criteria is that different incoming messages must produce different, grounded proposal amounts, and it must never send without owner approval.

  **Priority**: P1
  **Estimated Scope**: Medium

  **Strategy Admission**:
  - Target ID: OHC-04 (Customer inquiry → qualified quote)
  - Stage: Launch (Days 15–45)
  - Gap: Observed (Codebase gap: hardcoded $5,000 proposal)
  - Evidence level: Documented source finding
  - Baseline/Result: Currently 0% of inquiries produce dynamic quotes; result is 100% of inquiries produce grounded drafts.
  - Dependencies: OHC-03 (Service catalog)
  - Non-goals: Full automated negotiation
  - Authority class: Explicit owner approval required before sending
  - Cost/measurement: LLM prompt cost per inquiry (expected <$0.02)
  - Acceptance checks: Unit test showing two distinct inquiries yield different scopes; E2E test of the approval flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
