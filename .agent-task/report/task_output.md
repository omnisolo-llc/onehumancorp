issue_title: "Implement AI-Automated Proposal Drafting and Client Approval Workflows"
issue_description: |
  **Title**: Implement AI-Automated Proposal Drafting and Client Approval Workflows

  **Problem Statement**:
  Nora (Agency Principal) spends disproportionate time transcribing client intake notes into formal proposals, seeking approvals, and transitioning those approvals into actionable project tasks and invoices. She requires an assistant-led workflow that captures client demand, automatically drafts a project proposal, generates a mobile-optimized client approval link, and orchestrates the transition into active work upon client sign-off.

  **Research Report**:
  * Competitive Context: Platforms like HoneyBook, Dubsado, and Notion AI handle proposals but often require complex manual setup, feeling like a heavy CRM rather than an assistant. Shopify and Wix lack service-based proposal capabilities.
  * OHC Opportunity: By leveraging our AI Job Queue and LLM RAG capabilities (using Nora's past successful proposals), OHC can instantly provide a drafted proposal in the owner feed.
  * Key findings: Small service businesses lose 20-30% of leads due to slow proposal turnaround. Instant, mobile-first approval links dramatically improve conversion compared to PDF email attachments.

  **Design Doc**:
  *Architecture Diagram*:
  ```mermaid
  graph TD
      A[Client Intake Form/DM] --> B[AI Triage Agent]
      B --> C[Draft Proposal Job]
      C --> D[PostgreSQL - Proposals Table]
      D --> E[Owner Feed - Needs Review]
      E -->|Owner Approves| F[Generate Client Link]
      F --> G[Client Approves & Pays Deposit]
      G --> H[Create Project Tasks & Invoice]
  ```
  *Mobile UX Flow (375px)*:
  1. **Owner Feed**: A UniFi-style translucent glass card appears: "Drafted proposal for [Client Name] based on recent intake."
  2. **Review Screen**: The owner taps the card to view the AI-drafted proposal. Actions available: "Edit", "Approve & Send".
  3. **Client View**: Client receives SMS/email with a clean, branded link. They see the proposal, tap "Approve", and are immediately prompted for a deposit via Stripe.

  *AI Agent Integration Points*:
  - **Customer & Relationship Assistant**: Drafts the initial proposal content using past context and client notes.
  - **Sales & Revenue Assistant**: Prices the proposal based on agency historical data and configures the Stripe deposit link.
  - **Operations Assistant**: Converts the approved proposal into actionable tasks in Nora's active work queue.

  *Key Design Decisions*:
  - Proposals are treated as stateful entities (`Draft`, `Sent`, `Approved`, `Rejected`).
  - Strict row-level security (`tenant_id`) enforced for all proposal reads/writes in PostgreSQL.
  - Client viewing is handled via a secure, tokenized public route (no login required for the client).

  **Implementation Prompt**:
  Implement the AI-Automated Proposal Drafting feature.
  1. Define the PostgreSQL schema and Go/gRPC models for `Proposal` with multi-tenant row-level security.
  2. Create the AI background job (using PostgreSQL SKIP LOCKED) that takes intake notes and generates the proposal text.
  3. Build the Flutter frontend UI for the owner to review and approve the draft in their Agent Feed.
  4. Implement the client-facing public approval and deposit page (mobile-first, 375px optimized).
  5. Ensure 100% unit test coverage for the new services and at least one Playwright E2E test verifying the flow from drafting to client approval.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
