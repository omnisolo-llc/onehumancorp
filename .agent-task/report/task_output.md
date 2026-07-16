issue_title: "Implement Agentic Proposal & Project Invoicing Workflow"
issue_description: |
  ## Title: Implement Agentic Proposal & Project Invoicing Workflow

  ## Problem Statement
  Service professionals, independent contractors, and small agency owners (like Nora, the Agency Principal persona) spend a disproportionate amount of time switching between tools to manage the client lifecycle. Currently, taking a new client from inquiry to paid invoice involves jumping between email (for intake), a document editor (for proposals), a project management tool (for task tracking), and an accounting platform (for invoicing). This fragmented experience causes delayed responses, lost leads, and overdue invoices. Existing platforms either focus solely on the financial side (QuickBooks) or the proposal side (Dubsado/HoneyBook), lacking an integrated AI assistant that proactively moves the work forward autonomously.

  ## Research Report
  - **Market Context**: The SMB market for professional services relies heavily on "all-in-one" CRMs like HoneyBook or 17hats, which cost $20-$40/month. However, these platforms are static; they require the owner to manually trigger every workflow step (e.g., clicking "send invoice" after a project is marked complete).
  - **Competitor Gaps**:
    - *Shopify*: Has B2B features, but they are geared towards wholesale products, not service-based project proposals.
    - *Wix/Squarespace*: Offer basic invoice generation but lack project lifecycle management and AI-driven automation.
    - *HoneyBook/Dubsado*: Good for service businesses, but they are passive tools. They wait for owner input rather than having an AI agent draft the proposal based on a DM conversation and auto-schedule the invoice.
  - **The OHC Opportunity**: OHC can differentiate by introducing an "Agentic Workflow." When a client inquiry comes in, the Sales/Customer Success Agent drafts the proposal automatically based on the owner's service catalog. Once the client approves, the Operations Agent automatically provisions the project tasks, and the Finance Agent schedules the invoice reminders.

  ## Design Doc
  ### High-Level Architecture & Data Model (PostgreSQL)
  To support this workflow, we need strict multi-tenant row-level security (RLS) across the following entities:
  - `Proposal`: Represents a quoted scope of work, linked to a `Customer` and `Tenant`. State can be `draft`, `sent`, `approved`, `rejected`.
  - `Project`: Represents the approved body of work, created automatically upon Proposal approval.
  - `Invoice`: Links to a `Project` and `Customer`, handling payment requests via Stripe.
  - `Task`: Individual actionable items linked to a `Project`, assigned to the owner or staff.

  ### AI Agent Integration Points
  - **Work Triage / Customer Assistant**: Captures the initial lead from connected channels (Email, IG DMs) and extracts the project requirements.
  - **Sales & Revenue Assistant**: Receives the extracted requirements and drafts a `Proposal` using the tenant's service catalog pricing and past successful proposals as RAG context.
  - **Operations Assistant**: Subscribes to the `proposal.approved` event. Upon approval, it creates a `Project` record and populates standard `Task`s based on the proposal scope.
  - **Finance & Decision Assistant**: Subscribes to project milestones. Automatically drafts and schedules `Invoice` emails (e.g., 50% deposit upfront, 50% on completion) and follows up on overdue payments.

  ### Mobile UX Flow (375px First)
  - **Owner Dashboard (Feed)**: The owner sees a card: *"New inquiry from Acme Corp. Proposal drafted."*
  - **Review Screen**: Tapping the card opens a translucent glass modal. The owner sees the AI-drafted proposal. Large 44x44px touch targets allow them to edit line items or tap **"Approve & Send"**.
  - **Client View**: The client receives a clean, mobile-optimized web link to review the proposal, sign digitally, and pay the deposit via Stripe Checkout (Apple Pay/Google Pay enabled).
  - **Project View**: Post-approval, the owner's feed updates with a "Project Kicked Off" card showing the auto-generated tasks.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Client
      actor Nora (Owner)
      participant TriageAgent as Work Triage Agent
      participant SalesAgent as Sales Agent
      participant OpsAgent as Operations Agent
      participant FinanceAgent as Finance Agent

      Client->>TriageAgent: Sends project inquiry (DM/Email)
      TriageAgent->>SalesAgent: Extracts requirements & requests proposal
      SalesAgent->>Nora: Drafts Proposal & notifies owner
      Nora->>SalesAgent: Reviews and taps "Approve & Send"
      SalesAgent->>Client: Sends Proposal Link
      Client->>SalesAgent: Approves Proposal
      SalesAgent->>OpsAgent: Emits `proposal.approved` event
      OpsAgent->>Nora: Creates Project & initial Tasks
      OpsAgent->>FinanceAgent: Triggers Deposit Invoice
      FinanceAgent->>Client: Sends Deposit Invoice (Stripe)
  ```

  ## Implementation Prompt
  **Feature Name**: Agentic Proposal & Project Invoicing Workflow
  **Target Persona**: Nora (Agency Principal)

  **Outcome**:
  Implement the backend data models and the owner-facing mobile UI to support the proposal lifecycle. When an inquiry is received, the system should allow the owner to review an AI-drafted proposal, send it to the client, and seamlessly convert it into a project with an associated deposit invoice upon client approval.

  **Acceptance Criteria**:
  1. Define the SQL schema (PostgreSQL) for `proposals`, `projects`, and `invoices` with strict `tenant_id` RLS constraints.
  2. Implement the backend API (REST/gRPC) to create, update, and approve proposals.
  3. Build the mobile-first (375px) UI in Flutter (or the relevant frontend stack) using OHC Premium Tokens (translucent glass materials, clean hierarchy) to view and approve a drafted proposal.
  4. Ensure zero mock data is used in the UI; all state must flow from the backend API.
  5. Include full Playwright E2E test coverage for the Critical User Journey (CUJ): Login -> View Draft Proposal -> Send Proposal -> Simulate Client Approval -> Verify Project Created.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []