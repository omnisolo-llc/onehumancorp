issue_title: "Automated Agentic Service Proposals & Smart Invoicing"
issue_description: |
  ## 1. Problem Statement
  Service-based and project-based small business owners (e.g., Nora the Agency Principal, Carlos the Field Service Owner) spend an disproportionate amount of time on the pre-sales and post-sales administrative friction: gathering requirements from initial chats, manually writing service proposals/estimates, chasing client approvals, converting approved estimates into invoices, and tracking overdue payments. Traditional platforms separate the CRM inbox, the estimating tool, and the invoicing software, forcing the owner to act as the manual integration layer. This delays the "quote-to-cash" cycle and costs them revenue.

  ## 2. Research Report
  - **Market Context**: Service operators currently rely on a fragmented stack. Tools like Jobber or Housecall Pro dominate field services, while HoneyBook or Bonsai serve agency principals. These tools have robust features but require significant manual data entry to move a lead from an initial inquiry to a paid invoice.
  - **The OHC Opportunity**: OHC's unique advantage is the unified event mesh and the persistent multi-agent context. By tightly coupling the "Sales Assistant" and "Finance Assistant," OHC can autonomously transition a conversational inquiry into a structured proposal, track approval, and seamlessly trigger the invoicing lifecycle—all via zero-touch or one-tap owner approvals on mobile.
  - **Competitor Gaps**:
    - *HoneyBook/Bonsai*: Excellent proposal/invoice templates, but passive. The user must manually parse client emails and build the proposal.
    - *Jobber*: Strong field service management, but lacks conversational AI to ingest raw incoming SMS/DMs and draft the initial quote autonomously.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Proposal`: Represents a drafted or sent service estimate. Links to `Customer`, `Service` items, and has states (`Draft`, `Sent`, `Approved`, `Rejected`).
  - `Invoice`: Represents a billable event. Linked to an approved `Proposal` (or generated ad-hoc). Has states (`Draft`, `Sent`, `Partially Paid`, `Paid`, `Overdue`).
  - `LineItem`: Shared schema for granular service/product lines attached to both Proposals and Invoices.
  - `PaymentSchedule`: Defines deposit requirements and milestone payments.

  ### AI Agent Integration
  - **Sales Agent ("The Closer")**: Monitors the Work Triage unified inbox. When a conversational thread indicates a request for a quote (e.g., "How much to paint my living room?"), the Sales Agent extracts the requirements, queries the `Service` catalog for pricing, and drafts a `Proposal`.
  - **Finance Agent ("The Accountant")**: Monitors `Proposal` state changes. When a client approves a proposal via the customer-facing web link, the Finance Agent automatically generates the corresponding `Invoice`, schedules the deposit payment link dispatch, and sets up automated overdue reminders.

  ### Mobile UX Flow (375px First)
  1. **Owner Triage View**: Nora receives a push notification: "New Proposal Drafted for Client X."
  2. **Review & Send**: She taps the notification, opening a Glassmorphism card displaying the AI-generated proposal summary (scope, line items, total).
  3. **One-Tap Action**: A prominent "Review & Send to Client" button. If she needs changes, a native keyboard text input allows her to tell the agent: "Add a 10% rush fee," and the agent instantly regenerates the draft.
  4. **Client View**: The client receives an SMS/Email link, opening a mobile-optimized web view to review the proposal, digitally sign/approve, and immediately pay the required deposit via Stripe.
  5. **Automated Follow-up**: Once approved, Nora's dashboard updates, and the Finance Agent takes over invoicing seamlessly.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Incoming Client Message] -->|Webhook| B(Work Triage / Inbox)
      B --> C{Sales Agent}
      C -->|Extracts Scope & Pricing| D[Draft Proposal]
      D --> E[Owner Mobile App Feed - 375px]
      E -->|1-Tap Approve| F[Send Proposal Link to Client]
      F --> G{Client Approves via Web}
      G -->|Webhook Event| H(Finance Agent)
      H -->|Generate & Send| I[Invoice & Deposit Link]
      I --> J[Stripe Checkout]
      J -->|Payment Success| K[Update Ledger & Notify Owner]
  ```

  ## 4. Implementation Prompt
  **Feature Name**: Automated Agentic Service Proposals & Smart Invoicing
  **Target Personas**: Nora (Agency Principal) & Carlos (Field Service Owner)
  **Outcome**: When a client asks for a quote via message, the owner finds a fully drafted, accurately priced proposal waiting in their OHC mobile feed. Upon the owner's 1-tap approval, the client receives the proposal, signs it, and the system automatically generates the invoice and collects the deposit without further owner intervention.

  **Next Actions for Engineering**:
  1. Define the PostgreSQL schema and multi-tenant row-level security policies for `Proposal`, `Invoice`, and `LineItem` entities.
  2. Implement the Sales Agent intent classification to detect quote requests and extract structured service scope from natural language.
  3. Build the backend gRPC/REST APIs for managing the proposal lifecycle and linking it to the existing Stripe payment integration for deposits.
  4. Develop the Mobile-First UI (Flutter/PWA) for the owner to review, edit via natural language, and approve drafted proposals.
  5. Create the secure, public-facing web view for clients to review and approve proposals.
  6. Add comprehensive Playwright E2E tests simulating the full quote-to-cash lifecycle starting from an initial customer message.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
