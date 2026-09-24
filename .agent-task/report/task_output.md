issue_title: "[F08] Refactor Checkout Session Handling to Eliminate Fabricated Success Outcomes"
issue_description: |
  We performed a comprehensive investigation following the directives of the Principal Product Architect to identify one actionable gap or uncertainty.

  ## Mission Queue Protocol
  - **Target ID:** OHC-03
  - **Title**: Eliminate Fabricated Checkout Sessions
  - **Problem Statement**: The current booking functionality lacks actual external integration verification and uses simulated/fabricated session outcomes for payments (F08). The user can appear to complete checkout without real money moving, rendering it impossible to trust the system of record.
  - **Research Report**: Based on `docs/research/business_capability_and_usage_economics_audit.md` (F08), existing commerce placeholders fabricate checkout-looking URLs. This is an explicit gap preventing trust in the billing ledger. We must persist real drafts, distinguish draft/sent/delivered, and remove the fabricated checkout flow.
  - **Design Doc**:
    - **Architecture Diagram**:
      ```mermaid
      graph TD;
          A[Client] -->|Create Invoice Request| B(Invoice Service);
          B --> C{Real Provider Configured?};
          C -->|Yes| D[Draft Local Invoice];
          C -->|No| D;
          D --> E(Wait for explicit payment configuration/checkout);
      ```
    - **Mobile UX Flow**: 375px viewport displays "Invoice Drafted" state natively, hiding payment buttons until a provider is formally attached.
    - **AI Integration**: The Finance agent monitors invoice states strictly; it cannot arbitrarily advance a draft invoice to 'Paid' without an explicit provider receipt.
  - **Implementation Prompt**:
    Modify the invoice and booking logic to remove simulated checkout sessions. A newly created invoice must remain in a local drafted/unpaid state and never return a fake checkout URL if an integration like Stripe or Mercado Pago is not present and verified. Webhooks or explicit provider sessions must be the only mechanism that can transition status to success.
  - **Priority**: P0
  - **Estimated Scope**: Medium
  - **Strategy Admission**: Carlos (handyman), Stage: Run (OHC-03 booking/deposit). Evidence Level: Implemented/Test-verified. Metric: Number of successfully recorded drafted invoices vs fabricated completions (Target: 100% drafted). Cost Impact: Zero (local state only). Authority Class: System Internal. Happy path: Invoice drafted locally. Failure path: Invoice remains unpaid.
  - **Superpowers Provenance**: Loaded skills: using-superpowers, brainstorming (SKILL.md read directly from /tmp/superpowers). Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71. Checks: Reviewed `business_capability_and_usage_economics_audit.md` (F08). Outcomes: Gap identified and documented in this report.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
