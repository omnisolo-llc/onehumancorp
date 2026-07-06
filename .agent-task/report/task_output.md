issue_title: "Agentic Project Milestone & Autonomous Invoicing System"
issue_description: |
  # Agentic Project Milestone & Autonomous Invoicing Architecture

  ## Title
  Agentic Project Milestone & Autonomous Invoicing System

  ## Problem Statement
  Service professionals and agency owners (like Nora the agency principal or Carlos the handyman) manage projects that span multiple days or weeks. They need to track client approvals, collect deposits, and send invoices upon milestone completion. Traditional tools (like Freshbooks or generic Shopify invoicing) require the owner to manually remember to send the invoice, track unpaid bills, and awkwardly follow up with clients. This leads to delayed cash flow, administrative burden, and uncomfortable client conversations.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Geared heavily toward immediate checkout. Invoicing exists but is disconnected from project management or operational milestones.
  - **Freshbooks/Quickbooks:** Strong on accounting, but completely separate from the work itself (intake, tasks, communication). Sending a reminder is manual or based on a dumb time-delay.
  - **HoneyBook/Dubsado:** Good at service workflows but complex to set up. Requires heavy manual configuration of workflows.
  - **OHC Opportunity:** By combining our Operations Assistant and Finance Assistant, OHC can autonomously link project milestones to invoicing. When Nora marks a design phase as "Approved", the Finance Assistant automatically drafts the milestone invoice and pushes an "Action Card" to Nora's mobile feed: "Send 50% milestone invoice to Acme Corp?" If unpaid after 3 days, the Customer Success agent drafts a polite follow-up.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Operations: Milestone Approved] -->|Event| B(Internal Event Bus)
      B --> C{Agent Orchestrator}
      C -->|Trigger| D[Finance Assistant: The Accountant]
      D -->|Query Project/Contract| E[(Tenant DB: Project & Ledger)]
      D -->|Draft Invoice via Stripe| F[Payment Gateway]
      F -->|Webhook Pending| D
      D -->|Push Action| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I -->|Email/SMS| J[Client]
      C -->|Schedule Check| K[Customer Assistant: The Ambassador]
      K -->|If Unpaid| L[Draft Polite Follow-up]
  ```

  ### Mobile UX Flow (375px)
  1. **Owner Feed:** Nora opens OHC on her phone. An Action Card appears: "Milestone 'Design Approval' reached. Drafted $1,500 invoice for Acme Corp."
  2. **Card Details:** Shows the invoice amount, due date, and the drafted email message.
  3. **Interaction:** Large "Send Invoice" (primary) and "Edit" (secondary) touch targets (≥ 44x44px).
  4. **Post-Action:** Card transitions to a subtle "Sent & Awaiting Payment" state.

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Listens to milestone completion events. Drafts the invoice, calculates tax/totals, and interfaces with the underlying payment intent system.
  - **Customer Success Agent ("The Ambassador"):** Monitors aging invoices and drafts polite, context-aware follow-up messages based on the client's past communication style.
  - **Operations Agent ("The Manager"):** Tracks the project state and emits milestone events.

  ### Key Design Decisions
  - **Event-Driven Coupling:** The project management module and invoicing module must not be tightly coupled in code. They communicate via the internal Event Bus to allow agents to orchestrate the workflow.
  - **Owner-in-the-Loop:** Invoices are never sent completely autonomously without owner approval, to prevent embarrassing mistakes. They are always presented as Action Cards.
  - **Unified Ledger:** All generated invoices must write to the same central Postgres multi-tenant ledger used by e-commerce checkouts for unified reporting.

  ## Implementation Prompt
  **User-Facing Outcome:** Agency and service owners can link project milestones to payments. When a milestone is completed, an invoice is automatically drafted and presented in their mobile feed for 1-tap approval, followed by automatic smart follow-ups for unpaid invoices.

  **Critical User Journey (CUJ):**
  1. Owner logs in and views their active project.
  2. Owner taps to mark a project milestone as "Complete".
  3. The system's agent intercepts this and immediately pushes an Action Card to the owner's feed with a drafted invoice.
  4. Owner reviews the drafted invoice on a 375px screen and taps "Approve & Send".
  5. The system records the invoice, sends it via email/SMS, and schedules a dunning check.

  **Acceptance Criteria:**
  - Build the event pub/sub linkage between the project module and the finance module.
  - Create the Agent logic that generates the Action Card upon milestone completion.
  - Implement the 375px mobile-friendly Action Card UI with translucent glass styling.
  - Ensure 100% Playwright E2E coverage for the CUJ starting from milestone completion to invoice approval.
  - Do not mock the database or API layer; use the real local stack.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
