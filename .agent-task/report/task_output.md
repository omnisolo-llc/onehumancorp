issue_title: "Implement Autonomous AI Proposal & Invoicing System for Service Operators"
issue_description: |
  ## 1. Problem Statement
  Service operators like Carlos (Handyman) and Nora (Agency Principal) cannot use standard e-commerce carts because their work requires custom estimates, proposals, and milestone-based invoicing. Currently, OHC forces them to use external tools (like QuickBooks or manual PDFs) which breaks the unified work assistant experience. They need a way to intake a customer request, have the AI draft a localized, professional proposal with a deposit link, and automatically track invoice approvals directly from a 375px mobile device.

  ## 2. Research Report
  - **Codebase & Docs Audit**: The current `src/server/services/billing/` and `ledger/` modules handle subscriptions and one-off payments via Stripe, but lack an entity for "Proposals" or "Estimates" that can transition to an "Invoice".
  - **Competitor Systems Audit**: Shopify treats services as digital products, which is a poor fit for custom work. Wix provides basic invoicing but requires manual data entry. Specialized tools like Jobber or HoneyBook are too complex and lack proactive AI agents that draft quotes for the user.
  - **Identify Gaps**: OHC lacks a unified data model for `Proposal -> Deposit -> Final Invoice` and an AI capability to draft these autonomously from raw customer chat or intake forms.

  ### Product-Use Evidence (Dogfooding)
  - **Persona:** Carlos (Handyman)
  - **Browser/Playwright Flow Attempted:** Logged into the OHC Web app using seeded test credentials (`test@example.com`). Navigated to the "Work Intake" view to respond to customer inquiries.
  - **CUJ Gap Observed:** When a customer requests a "Quote for a kitchen sink repair", the UI provides no built-in way to generate a structured estimate or deposit link. I had to manually type a text reply with a generic payment link, which breaks the professional workflow and provides no systemic tracking of pending revenue.
  - **Why it matters:** Service owners rely on structured proposals and deposits to secure work. Forcing them to manually calculate and type out quotes on a mobile keyboard causes friction, errors, and lost leads.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ CUSTOMER : has
    CUSTOMER ||--o{ PROPOSAL : requests
    PROPOSAL ||--o{ INVOICE : generates
    INVOICE ||--o{ PAYMENT : receives
    PROPOSAL {
      uuid id
      string status
      jsonb line_items
      decimal total_amount
    }
  ```
  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed**: Carlos receives a new service request card on his phone.
  2. **Agent Draft**: The "Sales Assistant" drafts a proposal based on Carlos's standard rates and the request text.
  3. **Approval Screen**: Carlos views a 375px mobile card with the drafted line items and a "Send Proposal with 20% Deposit Link" button (minimum 44x44px touch target).
  4. **Customer View**: The customer receives a mobile-friendly web link to approve the proposal and pay the deposit via Stripe/Apple Pay.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Listens for `ServiceRequestCreated` events, parses the natural language scope using Gemini Pro, and outputs structured `line_items` to draft the `Proposal` record.

  ### Security & Zero Trust
  - All `Proposal` and `Invoice` records must enforce row-level PostgreSQL security based on `tenant_id` from the authenticated SPIFFE/SPIRE context.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: Carlos can generate, review, and send a professional proposal with a payment link in 3 taps from his phone, powered by the AI Sales Assistant.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Create the `proposals` and `invoices` PostgreSQL schema with tenant isolation.
  2. Implement the gRPC/REST endpoints for managing proposals.
  3. Build the AI Sales Assistant prompt architecture to parse service requests into line items.
  4. Develop the Flutter mobile-first (375px) UI for reviewing and approving proposals using the OHC Premium Token (Translucent Glass) design system.
  5. Verify the entire flow using Playwright E2E tests with seeded service request data. Ensure all interactive elements have their expected effect (no dead buttons).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
