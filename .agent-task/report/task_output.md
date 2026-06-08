issue_title: "OHC Work Feed Architecture Design & Agent Workflows"
issue_description: |
  # OHC Work Feed Architecture Design & Agent Workflows

  ## Problem Statement
  Small business owners face "omnichannel chaos" - incoming demand and action items are scattered across emails, Instagram DMs, forms, texts, payments, and calendar bookings. They lack a unified view of what needs attention right now, leading to lost sales, missed bookings, and delayed replies. They don't have time to seek out information across multiple tools; they need the system to proactively coordinate and surface actionable items.

  ## Research Report
  Based on `docs/business/market_research/agent_feed_deep_dive.md`, `[research]_ohc_smb_mobile_first_agentic_workflows.md`, and `ohc_smb_platform_research_report.md`:
  - **The Gap:** Legacy platforms (Shopify, Wix) focus on dashboards and reporting, requiring users to log in and interpret data to know what to do next. They fail micro-SMEs who need proactive coordination.
  - **Persona Evidence:** Maya (Baker) misses sales because she can't reply to Instagram DMs fast enough while baking. Carlos (Handyman) needs service requests and bookings presented in one place.
  - **Competitive Differentiation:** OHC's unique value is the shift from "tools" to "invisible autonomous agents." The platform must move from passive reporting to proactive action suggestions via an "Agent Feed".

  ## Design Doc
  The core of the solution is the "Agent Feed" - a unified, mobile-first Inbox/Feed where AI agents surface actionable items to the owner.

  ### 1. Data Model & Sync Protocol (Centralized Inbox)
  - **Work Item Entity (PostgreSQL):** A unified table to store incoming items (Messages, Booking Requests, Payment Confirmations). Every item has a `tenant_id`, `status` (pending, drafted, completed, archived), `type` (message, task, alert), and a structured `payload`.
  - **AI Job Queue:** Integration with the PostgreSQL `SKIP LOCKED` job queue to process incoming events and trigger agents.
  - **Memory & Context:** Integration with tenant-scoped memory to provide agents with context (e.g., product catalog, past customer interactions, business policies).

  ### 2. Architecture Diagram

  ```mermaid
  graph TD
      A[External Webhooks] -->|Ingest| B[AI Job Queue]
      C[Internal Events] -->|Ingest| B
      B -->|Dequeue| D[Work Triage Agent]
      D -->|Create| E[(PostgreSQL: Work Item)]
      D -->|Trigger| F[Customer Success Agent]
      D -->|Trigger| G[Operations Agent]
      F -->|Read RAG| H[(Tenant Memory)]
      F -->|Draft Reply| E
      G -->|Draft Action| E
      E -.->|Sync| I[Mobile App UI 375px]
      I -->|User Approve| J[Action Execution]
  ```

  ### 3. AI Agent Coordination
  - **Work Triage Agent:** Ingests incoming webhooks (e.g., Instagram Graph API, new form submissions), categorizes the intent, and creates a `Work Item`.
  - **Customer Success Agent ("The Ambassador"):** Detects new message items. Queries RAG/Memory for business context. Drafts a response. Attaches the draft to the `Work Item` and sets status to `drafted`.
  - **Operations Agent ("The Manager"):** Detects operational items (low stock, missed bookings) and drafts "Suggested Actions" (e.g., "Approve Restock", "Send Follow-up").

  ### 4. Mobile-First UX Flow (375px)
  - **The Feed View:** The default view upon opening the app. A vertically scrollable list of "Action Cards."
  - **Action Card UI:**
    - Clear title indicating the source and intent (e.g., "Instagram DM: Pricing Inquiry").
    - A preview of the customer's message or the system alert.
    - An AI-drafted response or action plan shown in a distinct, translucent "Agent Draft" styled block.
    - Prominent, touch-friendly primary buttons (>= 44x44px): **Approve**, **Edit**, **Discard**.
  - **Interaction:**
    - Tapping "Approve" dispatches the action (sends message, updates inventory) and removes the card from the feed.
    - Tapping "Edit" opens a native text input or detailed view to adjust the draft.

  ### AI Agent Integration Points
  - **Trigger:** Webhook receivers map external events to the AI Job Queue.
  - **Execution:** Worker nodes dequeue jobs, execute the specific agent (Triage, Customer, Ops) using the Gemini Pro provider, and update the `Work Item` record with the result (draft).

  ## Implementation Prompt
  **User-Facing Outcome:** The user opens the OHC app and sees a feed of actionable items. They see a customer inquiry from Instagram alongside a drafted reply from the AI assistant. They can tap "Approve" to send the reply immediately, or "Edit" to modify it.

  **Critical User Journey (CUJ):**
  1. The system simulates an incoming customer inquiry (e.g., via a test webhook endpoint).
  2. The Work Triage Agent processes the inquiry and creates a feed item.
  3. The Customer Success Agent drafts a response based on the simulated business context.
  4. The owner logs into the OHC web app (simulating mobile view).
  5. The owner sees the new item in their feed with the drafted response.
  6. The owner clicks "Approve".
  7. The system marks the item as resolved/sent.

  **Acceptance Criteria:**
  - Define the `Work Item` entity schema in the backend with proper multi-tenant isolation.
  - Implement a basic API endpoint to ingest test events and trigger the AI drafting flow.
  - Build the frontend "Agent Feed" view in Flutter/PWA, adhering to the 375px mobile-first constraint and using the translucent glass design tokens.
  - Implement the "Action Card" component with Approve/Edit/Discard buttons.
  - Write Playwright E2E tests covering the complete CUJ, starting from the simulated event ingestion to the owner's approval in the UI.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
