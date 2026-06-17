issue_title: "Research & Design: Autonomous Agentic B2B Proposal & Client Approval Workflows"
issue_description: |
  ## 1. Problem Statement
  Service-based B2B small businesses and agencies (e.g., Nora the Agency Principal) face significant friction managing client intake, proposing work, and securing approvals. Traditional tools (like HoneyBook or PandaDoc) are disconnected from the core operational workspace and require manual effort to draft documents, track status, and follow up. There is no unified system that leverages an AI assistant to automatically generate proposals from client intake forms, track client views/approvals, and seamlessly transition into actionable project tasks and invoicing.

  ## 2. Research Report
  - **Market Context**: Platforms like HoneyBook, PandaDoc, and Proposify offer robust proposal and contract management but function as standalone silos. Shopify and Wix are tailored for B2C e-commerce and lack native B2B workflow capabilities.
  - **The OHC Opportunity**: By integrating the B2B proposal and approval workflow directly into the OneHumanCorp platform, OHC can eliminate the need for disjointed SaaS tools. AI agents can bridge the gap from initial client intake to a finalized proposal, converting approved proposals directly into project tasks, resource allocations, and invoice schedules.
  - **Competitor Gaps**:
    - *HoneyBook / Proposify*: High monthly costs ($40-$50/mo), siloed from operations and broader team management.
    - *Shopify / Wix*: Non-existent native B2B proposal workflows.
    - *Notion AI*: Great for drafting but lacks native payment integration, client-facing approval states, and automated follow-ups.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `B2BClient`: Extends the Customer model with B2B-specific fields (company, tax ID).
  - `IntakeRequest`: Captures initial project requirements, timeline, and budget.
  - `Proposal`: Linked to `IntakeRequest` and `B2BClient`. Contains line items, terms, and state (draft, sent, viewed, approved, rejected).
  - `ProposalLineItem`: Links to services or custom work blocks.
  - `ApprovalEvent`: Audit log of client interactions with the proposal.

  ### AI Integration
  - **Sales Assistant (The Closer)**: Monitors new `IntakeRequest`s. Uses RAG against previous successful proposals to automatically draft a new `Proposal`. Notifies the owner (Nora) to review and send.
  - **Operations Assistant (The Manager)**: Once a `Proposal` is marked "approved", automatically generates a corresponding `Project`, assigns tasks to contractors, and schedules the initial deposit `Invoice`.
  - **Customer Success Assistant**: Follows up with clients who have "viewed" but not "approved" the proposal after a set period (e.g., 48 hours).

  ### Mobile UX Flow (375px)
  1. **Intake Triage**: Owner Dashboard shows a new card: "New Intake from Acme Corp".
  2. **Proposal Review**: Tapping the card opens a translucent, mobile-optimized view of the AI-drafted proposal. Nora can edit line items using native mobile inputs or tap "Approve & Send".
  3. **Client View**: The client receives an SMS/Email link to a responsive web view. They can review the terms and tap a large "Accept & Pay Deposit" button (via Stripe).
  4. **Project Kickoff**: Upon client approval, Nora receives a push notification, and the dashboard updates to show the new active project with pre-populated contractor tasks.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Client
      actor Nora (Owner)
      participant OHC Platform
      participant Sales Agent
      participant Ops Agent

      Client->>OHC Platform: Submits Intake Request
      OHC Platform->>Sales Agent: Trigger New Request Event
      Sales Agent->>OHC Platform: Drafts Proposal (RAG)
      Sales Agent->>Nora: Push Notification: "Proposal Drafted"
      Nora->>OHC Platform: Reviews (Mobile App) & Approves Send
      OHC Platform->>Client: Emails Proposal Link
      Client->>OHC Platform: Views & Approves (Pays Deposit)
      OHC Platform->>Ops Agent: Trigger Approved Event
      Ops Agent->>OHC Platform: Creates Tasks & Invoice Schedule
      Ops Agent->>Nora: Push Notification: "Project Kickoff Ready"
  ```

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous B2B Proposal & Approval Engine
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora receives a project request via her website. The Sales Agent drafts a detailed proposal. Nora reviews it on her phone and hits send. The client approves it, automatically triggering the Operations Agent to create project tasks and the Finance Agent to issue a deposit invoice.

  **Next Actions**:
  1. Implement the core Data Models (`IntakeRequest`, `Proposal`, `ProposalLineItem`) with strict multi-tenant isolation.
  2. Develop the AI Sales Agent capability to draft proposals based on intake data and historical context.
  3. Build the Mobile-First Owner UI for reviewing and sending proposals, and the Client-Facing UI for approval and deposit payment.
  4. Wire up the transition logic: approved proposal -> project creation -> invoice generation.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
