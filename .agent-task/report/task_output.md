issue_title: "Architectural Gap: Autonomous Agent-Driven Quote-to-Cash & Dynamic Deposit Pipeline"
issue_description: |
  ## 1. Problem Statement
  Service-based operators (Carlos the Handyman), bespoke creators (Maya the Baker), and agencies (Nora) cannot rely on static shopping carts. Their work intake often begins as an ambiguous request in a DM, email, or form ("How much for a custom vegan cake for 50 people?", "I need my gutters cleaned"). Traditional platforms like Shopify or Wix treat these as square-peg-in-round-hole problems, requiring owners to either manually create custom "products" for every quote or rely on complex, disconnected third-party invoicing and CRM tools. The manual process of tracking quotes, calculating deposits, managing milestone payments, and chasing invoices causes massive friction and lost revenue.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for invoicing and quotes (e.g., Quote Builder apps), adding monthly costs and fracturing the experience. Square Invoices handles the payment side well but lacks AI-driven context (it doesn't draft the quote based on an Instagram DM). HoneyBook is excellent for service professionals but operates as a siloed CRM rather than a unified owner workspace.
  - **The OHC Opportunity**: By natively integrating a Quote-to-Cash pipeline into the OHC platform, we can empower the Sales and Operations Agents to bridge the gap between unstructured demand (messages) and structured revenue (deposits/invoices).
  - **Competitor Gaps**:
    - *Shopify*: Products must be pre-defined; poor native support for custom services, complex deposits, or milestone payments without apps.
    - *Square/HoneyBook*: Great for payments/invoicing but lacks the AI assistant layer that automatically drafts the proposal directly from social media inquiries and negotiates/follows up autonomously.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
    participant Customer
    participant TriageAgent as Triage Agent
    participant SalesAgent as Sales Agent
    participant Ledger as Ledger (PostgreSQL)
    participant Stripe as Stripe API
    participant Owner as Owner (Mobile UX)

    Customer->>TriageAgent: DM: "Need custom cake, 50 ppl"
    TriageAgent->>SalesAgent: Extract intent & requirements
    SalesAgent->>Ledger: Retrieve pricing rules & availability
    SalesAgent->>SalesAgent: Draft Proposal & Deposit Config
    SalesAgent->>Owner: Push Action Card (Draft Quote)
    Owner->>SalesAgent: Tap "Approve & Send"
    SalesAgent->>Stripe: Generate Payment Link (Deposit)
    SalesAgent->>Customer: Reply with Proposal & Link
    Customer->>Stripe: Pays Deposit
    Stripe-->>Ledger: Webhook (Payment Intent Succeeded)
    Ledger->>OperationsAgent: Trigger Work Tasks
  ```

  ### Mobile UX Flow (375px)
  1. **Work Feed Card**: Owner sees a priority card: "Maya, 3 new custom cake inquiries. Quotes drafted."
  2. **Quote Review**: Tapping the card opens a clean, translucent glass-styled quote summary showing the extracted requirements, the suggested price (based on past similar orders), and a 50% deposit requirement.
  3. **One-Tap Action**: Primary action button "Approve & Send Link" (≥44x44px target). Secondary actions "Edit" or "Reject".
  4. **Active Quotes View**: A Kanban-style or simple list view tracking "Drafts", "Sent (Awaiting Deposit)", and "Booked".

  ### AI Agent Integration Points
  - **Work Triage Agent**: Listens to unified inbox, identifies service/quote intents vs. simple support questions.
  - **Sales & Revenue Agent**: Uses RAG over past successful quotes to price the new request. Generates the Stripe Checkout/Payment Intent for the deposit.
  - **Finance & Decision Agent**: Monitors unpaid deposits and drafts follow-up reminders.

  ### Key Design Decisions
  - **Native Ledger Integration**: Quotes and Invoices must be native entities in the Postgres database (tenant-isolated), not bolted on.
  - **Dynamic State Transitions**: A Quote transitions to an Order/Booking automatically the moment the Stripe webhook confirms the deposit.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Quote-to-Cash Pipeline
  **User Outcome**: When a customer requests a custom service via message, the owner receives a pre-drafted quote with a deposit link ready for one-tap approval.
  **CUJ (Critical User Journey)**:
  1. System receives a simulated webhook representing a customer inquiry for a custom service.
  2. The AI Sales Agent generates a `Quote` record with line items and a deposit requirement.
  3. The Owner views the drafted `Quote` in the 375px mobile feed.
  4. The Owner taps "Approve", which updates the `Quote` status to `SENT` and generates a mock payment link.
  5. A simulated payment webhook marks the deposit as `PAID`, triggering the creation of an actionable `Task` for the Operations Agent.

  **Acceptance Criteria**:
  - Create the core `Quote` and `QuoteLineItem` entities with RLS multi-tenant isolation.
  - Implement the AI Sales Agent prompt and tool to translate a raw text inquiry into a structured Quote.
  - Build the 375px mobile-first UX for reviewing and approving the quote, using translucent glass styling.
  - Ensure all states (Draft, Sent, Deposit Paid) are reflected accurately in the UI without hardcoded mock data.
  - Ensure Zero-Trust / Tenant Isolation on all endpoints.
  - Provide an E2E Playwright test covering the full flow from inquiry generation to deposit paid.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, sales-agent]
assignees: []
