issue_title: "Autonomous Agency Workflow & Proposal Generation Engine"
issue_description: |
  ## Mission Queue Protocol: Autonomous Agency Workflow & Proposal Generation Engine

  ### Problem Statement
  Agencies, freelancers, and independent professionals (like the Nora persona) spend an inordinate amount of time on administrative overhead: capturing client intake, drafting proposals, tracking approvals, and managing invoicing. Current tools (like Dubsado, HoneyBook, or Notion) require heavy manual setup, complex template management, and constant human intervention to move a project from inquiry to paid deposit. There is no unified, AI-native assistant that listens to project intake, auto-drafts a context-aware proposal, breaks it down into actionable tasks for contractors, and manages the invoice lifecycle automatically.

  ### Research Report
  - **Competitive Landscape:** HoneyBook and Dubsado are the primary incumbents for service-based SMBs, offering CRM, invoicing, and proposal generation. However, they rely on static templates and manual data entry.
  - **The Gap:** OHC can differentiate by leveraging the "Sales & Revenue Assistant" to draft proposals based on unstructured project intake (e.g., an email or DM from a client) and the "Operations Assistant" to automatically assign tasks to contractors once the proposal is approved.
  - **Key Needs for Nora:**
    - Unstructured intake (email/form) -> Structured project scope.
    - AI-drafted proposal with tiered pricing.
    - Client portal for approval and deposit payment (Stripe integration).
    - Auto-conversion of an approved proposal into a task board for contractors.

  ### Design Doc

  **Architecture:**
  - **Intake Layer:** Webhooks or email parsers capture lead inquiries and store them in the `Central Ledger (PostgreSQL)` under a new `ProjectIntake` entity.
  - **Agent Coordinator:**
    - *Customer Assistant* drafts an initial response acknowledging the inquiry.
    - *Sales Assistant* analyzes the `ProjectIntake`, retrieves past similar projects from the `Tenant Memory Bank` (pgvector), and generates a `Proposal` draft (pricing, timeline, scope).
  - **Approval & Payment Workflow:** The proposal is served via a lightweight, edge-cached React/Flutter PWA (`ProposalViewer`). Once the client signs/approves digitally, a `Stripe Checkout Session` is generated for the deposit.
  - **Operations Hand-off:** Webhook confirms payment -> *Operations Assistant* decomposes the proposal into a `TaskGraph` and assigns sub-tasks to contractors using `Redis` queue.

  **Mobile UX Flow (375px):**
  1. **Intake Notification:** Nora receives a push notification: "New Web Design Inquiry from Acme Corp."
  2. **Review Draft:** Nora taps to view the AI-generated proposal draft. The interface shows a card-based layout (Scope, Timeline, Cost) using the translucent Glassmorphism design tokens.
  3. **Edit & Send:** Nora taps a specific card to edit details using the native mobile keyboard, then swipes to approve and send to the client. Touch targets are strictly 44x44px minimum.
  4. **Tracking:** The home feed updates with a status chip: "Proposal Sent (Waiting for Client)".

  **AI Agent Integration Points:**
  - **Prompt Architecture:** The Sales Assistant uses a system prompt structured to extract budget, timeline, and deliverables from the intake text. It queries the `pgvector` store for `tenant_id` to find past accepted proposals for pricing calibration.
  - **Distributed Locks:** When a client is viewing/signing a proposal, a lock is placed using Redis Redlock (`ohc:lock:{tenant_id}:proposal:{proposal_id}`) to prevent Nora from accidentally mutating the pricing simultaneously.

  ### Implementation Prompt
  "Implement the end-to-end Agency Proposal Generation feature. Start by defining the `ProjectIntake` and `Proposal` PostgreSQL schemas with multi-tenant row-level security (`tenant_id` on every table). Next, create the gRPC/REST endpoints for submitting an intake and retrieving a proposal. Integrate the Sales Agent to automatically draft the proposal upon intake creation, using the provided MiniMax/Gemini LLM providers. Build a mobile-first (375px) Flutter or PWA UI for the owner (Nora) to review, edit, and send the proposal. Finally, ensure a Stripe payment intent is generated upon client approval. Verify all interactive UI elements with Playwright E2E tests (`src/e2e`), ensuring no mock data is used and the Critical User Journey is fully tested."

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
