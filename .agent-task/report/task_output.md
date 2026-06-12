issue_title: "Agentic Project Intake & Smart Proposal Engine"
issue_description: |
  # Research Report: Agentic Project Intake & Smart Proposal Engine

  ## Problem Statement
  Service-based businesses and small agencies (like Nora, the Agency Principal) spend a disproportionate amount of time on the initial intake, scoping, and proposal generation for new client projects. Traditional CRM and proposal tools (e.g., HoneyBook, HubSpot, PandaDoc) force the owner to manually translate client DMs, emails, and forms into a structured project scope, manually calculate costs/time, and then assemble a proposal document. This manual effort is a bottleneck for growth and often leads to delayed responses and lost leads.

  ## Research Report
  - **HoneyBook / Dubsado**: Excellent workflow and invoicing, but highly manual setup. The owner must still read the inquiry, decide the scope, and select the right template to send.
  - **HubSpot**: Powerful CRM, but not optimized for the rapid, lightweight "DM to Proposal" flow needed by a micro-agency or solo professional.
  - **Notion AI**: Good for drafting content, but lacks native integration with a structured quoting/invoicing engine.
  - **OHC Opportunity**: The OHC platform can differentiate by automating the entire "Intake -> Proposal" journey. When a new inquiry arrives (via form, email, or DM), the **Sales Agent** reads the intent, checks past similar projects (via Knowledge Agent), structures a draft scope, calculates an estimated price, and generates a responsive proposal. The owner simply reviews the drafted proposal in their feed and clicks "Approve to Send".

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Inquiry (Web Form, DM, Email)] --> B[Work Triage: Ingestion & Intent Classification]
      B --> C[Sales Agent: Proposal Drafting]
      C <--> D[Knowledge Agent: Retrieve Past Scopes & Pricing]
      C <--> E[Operations Agent: Check Capacity/Timeline]
      C --> F[Draft Proposal Created]
      F --> G[Owner Feed: Notification & Review Card]
      G -- "Approve" --> H[Proposal Sent to Client]
      H --> I[Finance Agent: Schedule Deposit Invoice]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification Feed**: The owner receives a push notification and sees an action card in their feed: "Draft Proposal Ready for [Client Name]".
  2. **Review Card**: Tapping the card opens a full-screen, translucent glass modal. It shows:
     - **Context**: A 1-sentence summary of the client's request.
     - **Scope & Price**: The AI-generated project phases, timeline, and calculated price.
     - **Confidence Score**: An AI confidence indicator based on how closely this matches past projects.
  3. **Edit/Approve Actions**: Large touch targets (44x44px minimum) for "Edit Details" (opens a simple form), "Discard", or "Approve & Send".
  4. **Client View**: The client receives a beautiful, mobile-optimized link to accept the proposal and pay the deposit via Stripe Checkout.

  ### AI Agent Integration Points
  - **Work Triage**: Unifies messages into a single queue and flags "High Intent" inquiries.
  - **Sales Agent**: The core engine. It uses a specific system prompt to map the unstructured inquiry to standard agency service packages defined in the tenant's catalog.
  - **Knowledge Agent**: Uses RAG on the tenant's historical proposals and completed projects to inform the Sales Agent's pricing estimates.
  - **Distributed Locks**: Uses `ohc:lock:{tenant_id}:proposal:{proposal_id}` to ensure that if the owner is editing the draft on their laptop, they don't accidentally send a stale version from their phone.

  ### Key Design Decisions
  - **No Manual Template Selection**: The system bypasses the step where the user must "Pick a Template". The AI dynamically assembles the proposal blocks based on the exact request.
  - **Invisible Handoffs**: The coordination between Sales, Knowledge, and Finance agents must be invisible to the user. They only interact with the final drafted artifact.

  ## Implementation Prompt
  **Outcome**: Implement the "Agentic Project Intake & Smart Proposal Engine" feature.

  **CUJ (Critical User Journey)**:
  1. An external inquiry is simulated/received (e.g., "I need a 5-page website and a logo").
  2. The backend AI agents process the inquiry and generate a `ProposalDraft` record, linked to the `Customer` and containing line items and a total price.
  3. The drafted proposal appears in the owner's mobile-first UI feed.
  4. The owner clicks "Approve", which updates the proposal state and triggers a simulated "send to client" event.

  **Acceptance Criteria**:
  - Implement the UI components for the Owner Feed Review Card and the Client-facing Proposal Acceptance view, ensuring perfect layout on a 375px width.
  - Implement the backend logic to handle the state transitions of a proposal (Draft -> Pending Approval -> Sent -> Accepted).
  - Integrate with the existing agentic job queue to simulate the proposal generation process.
  - Ensure all new API endpoints respect the `tenant_id` RLS isolation.
  - Add comprehensive Playwright E2E tests covering the owner review and approval flow.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
