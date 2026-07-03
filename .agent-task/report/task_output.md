issue_title: "Autonomous B2B Proposal & Invoicing Engine"
issue_description: |
  # Research Report: AI-Powered Autonomous Proposal & Invoicing Engine

  ## 1. Problem Statement
  Service-based B2B operators and agencies (like Nora the Agency Principal) experience significant friction transitioning from lead intake to a signed proposal and paid invoice. Existing platforms treat these as disjointed steps: one tool for CRM (HubSpot), another for proposals (PandaDoc), and a third for invoicing (Stripe/QuickBooks). This fragmentation requires manual data entry and follow-up, slowing down time-to-revenue and increasing the cognitive load on the owner.

  ## 2. Research Report
  - **Market Context**: Most SMB platforms (Shopify, Wix) are heavily optimized for B2C physical or digital goods. For B2B services, owners are forced to bolt together disjointed apps. AI competitors like Durable can generate initial websites but fail to handle end-to-end B2B operations.
  - **The OHC Opportunity**: By deeply integrating the Knowledge/Sales Agent (for drafting proposals based on past successful projects via RAG) and the Finance Agent (for automated invoice scheduling), OHC can collapse the B2B sales cycle from days to minutes.
  - **Startup Blocker Observation**: During the mandatory live-service gap audit via `docker compose up -d --build`, the stack failed to start locally due to an extraction error in the `valkey` image: `failed to extract layer ... failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`. This environment issue must be documented and addressed to unblock local E2E verification.

  ## 3. Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Client
      participant Nora (Owner)
      participant SalesAgent
      participant FinanceAgent
      participant Database

      Client->>SalesAgent: Submit intake form
      SalesAgent->>Database: Query past proposals (RAG)
      SalesAgent->>Nora: Push Notification: "Draft Proposal Ready"
      Nora->>SalesAgent: Approve & Send
      SalesAgent->>Client: Send Proposal Link
      Client->>Database: Accept Proposal
      Database->>FinanceAgent: Trigger Invoice
      FinanceAgent->>Client: Send Stripe Invoice for Deposit
  ```

  ### Mobile UX Flow (375px)
  - **Intake Alert**: Nora receives a rich push notification summarizing a new lead.
  - **Proposal Review Card**: Tapping the alert opens a 375px Glassmorphism card detailing the AI-drafted proposal (Scope, Timeline, Price). Nora can tap "Edit" or "Approve & Send".
  - **Automated Tracking**: The Owner Feed proactively updates the status of the proposal (Sent -> Viewed -> Accepted -> Paid).

  ### AI Agent Integration Points
  - **Sales Agent**: Classifies the incoming lead, uses RAG against tenant-scoped memory of previous successful proposals, and generates the draft document.
  - **Finance Agent**: Observes state changes (e.g., Proposal Accepted) and automatically generates/dispatches the Stripe Payment Link for the initial deposit.
  - **Zero Trust & Security**: Proposal generation and invoice creation strictly adhere to tenant boundaries (`tenant_id`) enforced by PostgreSQL RLS.

  ## 4. Implementation Prompt

  **Feature Name**: Autonomous B2B Proposal & Invoicing Engine
  **Target Persona**: Nora (Agency Principal)
  **Outcome**: When a new project inquiry arrives, the AI automatically drafts a customized proposal based on past work. Upon owner approval, it is sent to the client, and upon client acceptance, the initial invoice is automatically generated.

  **Critical User Journey (CUJ)**:
  1. Client submits a project request via the OHC storefront contact form.
  2. The Sales Agent drafts a proposal and pushes a notification to Nora's mobile app.
  3. Nora reviews the proposal draft on a 375px viewport and taps "Approve & Send".
  4. The client receives the proposal, reviews it, and clicks "Accept".
  5. The Finance Agent detects the acceptance and immediately issues a Stripe invoice for the deposit.

  **Acceptance Criteria**:
  - Proposal drafting must utilize a RAG pipeline to pull context from previous tenant proposals.
  - Mobile UI must provide a 1-tap approval flow for generated proposals that functions perfectly on a 375px screen without horizontal scrolling.
  - Playwright E2E tests must verify the entire flow from form submission to invoice generation without mocked internal APIs.
  - The local docker compose startup issue (`valkey` extraction failure) must be mitigated to ensure developers can run the stack locally.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
