issue_title: "Agentic Automated Quoting, Proposal, and Invoicing Workflow"
issue_description: |
  ## Title
  Agentic Automated Quoting, Proposal, and Invoicing Workflow

  ## Problem Statement
  Service-based businesses and agencies (e.g., Nora the Agency Principal, Carlos the Handyman) suffer from disconnected workflows for client intake, proposal creation, and invoicing. They often rely on manual data entry across multiple tools (email for intake, Word/Google Docs for proposals, separate accounting software for invoicing). This fragmentation leads to delayed proposals, missed follow-ups, delayed payments, and uncaptured revenue. Small business owners need an integrated assistant that turns an initial inquiry into a professional quote, converts approved quotes into active projects/tasks, and automatically schedules and tracks invoice reminders without requiring a specialized CRM or ERP system.

  ## Research Report
  - **Market Context**: Platforms like HubSpot and Salesforce are too complex, expensive, and require significant configuration for a small business owner. Specialized tools like HoneyBook or Jobber offer strong vertical solutions but trap users in closed ecosystems with poor multi-channel integration. Generic website builders (Wix, Squarespace) offer basic form intake but lack intelligent, agent-driven quoting and automated financial follow-ups.
  - **The OHC Opportunity**: OHC can uniquely solve this by leveraging its core unified communication (Agent Feed), integrated billing (Stripe), and autonomous AI agents. By tying intake directly to quoting, and quoting directly to task creation and invoicing, OHC provides an end-to-end "quote-to-cash" pipeline natively.
  - **Competitor Gaps**:
    - *HoneyBook/Jobber*: Feature-rich but siloed; require manual drafting of proposals and lack deep generative AI integration for adapting to conversational context (e.g., extracting project scope from an Instagram DM).
    - *Shopify/Wix*: Primarily built for standard products or simple bookings; very weak at custom multi-phase service quoting and milestone billing.
    - *Notion AI*: Great for document creation but disconnected from actual billing/payment execution and task assignment.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Inquiry via Form/DM] -->|Omnichannel Mesh| B(Work Triage Agent)
      B -->|Classifies as Lead| C[Proposal/Quote Entity Creation]
      B -->|Drafts Quote| D(Sales/Revenue Agent)
      D -->|Owner Review & Approve| E[Send to Client]
      E -->|Client Accepts| F(Operations Agent)
      F -->|Creates Tasks| G[Project/Task Tracker]
      F -->|Triggers Billing| H(Finance Agent)
      H -->|Generates Invoice via Stripe| I[Invoice Entity]
      I -->|Auto-Reminders| J[Client Payment]
      J -->|Payment Webhook| K[Update Project State]
  ```

  ### Data Model (PostgreSQL)
  - `Proposal`: Tracks the state of a quote (Draft, Sent, Accepted, Rejected, Expired), linked to a `Customer` and `Tenant`. Contains line items for services.
  - `Project`: Created upon Proposal acceptance. Links to the `Proposal` and tracks overall project status.
  - `Task`: Individual work units linked to a `Project`, assigned to internal staff or the owner.
  - `Invoice`: Financial record linked to a `Project` and `Customer`, tracking payment status and integrated with Stripe.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Monitors incoming messages (DMs, emails) and form submissions. Extracts project requirements, timelines, and budgets.
  - **Sales/Revenue Agent**: Automatically drafts a `Proposal` based on the extracted requirements and the tenant's predefined service catalog or past similar proposals (via RAG). Presents the draft to the owner in the Agent Feed.
  - **Operations Agent**: Upon proposal acceptance, automatically generates a structured list of `Tasks` for the new `Project`.
  - **Finance Agent**: Automatically schedules and sends `Invoices` based on proposal terms (e.g., 50% deposit upfront, 50% on completion) and handles gentle follow-up reminders for unpaid invoices.

  ### Mobile UX Flow (375px First)
  1. **Triage Feed**: Owner sees an Action Card: "New inquiry from Sarah. Drafted Quote attached."
  2. **Quote Review**: Tapping the card opens the Quote Draft. The UI is a clean, native mobile view of line items, total cost, and a summary. Owner can easily edit quantities, prices, or the AI-generated cover letter.
  3. **1-Tap Send**: Owner taps "Approve & Send".
  4. **Acceptance Notification**: Later, Owner receives a push notification: "Sarah accepted the quote! Project and initial deposit invoice created."
  5. **Project Overview**: A simple project tracker screen showing the checklist of automatically generated tasks and current payment status.

  ### Key Design Decisions
  - **Agent-Led Workflow**: The user is guided by the agents. The system *suggests* the quote; the user approves it. The system *creates* the tasks; the user checks them off.
  - **Unified State**: A single thread connects the initial DM to the final paid invoice, visible in the customer's history.
  - **Mobile Native Editing**: Quoting must be simple on a phone. Avoid complex document editors; use structured data entry for line items and simple text fields for descriptions.

  ## Implementation Prompt
  **Feature Name**: Agentic Quoting & Invoicing Workflow
  **Target Personas**: Nora (Agency Principal), Carlos (Handyman)
  **User-Facing Outcome**: Nora receives a project inquiry. She opens the OHC app to find a pre-drafted proposal ready for review. Upon sending and client acceptance, OHC automatically creates the project tasks and sends the deposit invoice via Stripe.
  **CUJ & Acceptance Criteria**:
  1. Implement the database schema for `Proposals`, `Projects`, `Tasks`, and `Invoices` with strict RLS multi-tenant isolation.
  2. Implement the Work Triage and Sales Agent capability to extract scope from a text inquiry and generate a structured Draft Proposal.
  3. Build the mobile-first UI for the owner to review, edit, and send the proposal.
  4. Implement the state transition logic: When a proposal is marked 'Accepted' (simulated via an API call), the Operations Agent creates a basic Project and Task list, and the Finance Agent creates a pending Invoice.
  5. Provide Playwright E2E tests: A user logs in, sees a drafted proposal in their feed, edits a line item, sends it, simulates client acceptance, and verifies that the corresponding Project and Invoice are created and visible in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []