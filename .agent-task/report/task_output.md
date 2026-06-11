issue_title: "Implement Agent Feed for Actionable AI Workflow"
issue_description: |
  # Research Report: Agent Feed for Actionable AI Workflow

  ## Track 1: Architectural Gap & Scaling Discovery
  The OneHumanCorp (OHC) platform aims to be an AI work assistant where owners act rather than just read dashboards. Currently, OHC lacks the central nervous system to proactively push critical updates, suggested actions, and drafted communications directly to the user's mobile device for review and approval. Small business owners (like Maya the baker or Carlos the handyman) suffer from "now what?" syndrome and context switching across fragmented apps.

  ## Track 2: Selected Architecture Deep Dive
  To solve this, we need to design the **Agent Feed**. The Agent Feed shifts OHC from a reactive dashboard to a proactive, mobile-first notification hub.

  ### Business Journey Mapping:
  - **Capture:** External events (Instagram DMs, Stripe webhook payments, order updates) are ingested.
  - **Process:** An AI agent layer (e.g., The Ambassador for CS) classifies the intent, checks state (e.g., inventory), and generates a drafted action/response.
  - **Action:** The drafted action is placed into the tenant's ActionRequiredQueue.
  - **Review:** The owner opens OHC, sees the pending Action Card in their feed, taps "Approve" (or Edit), and the system dispatches the execution.

  ### Data Model & Invariants:
  - `ActionItem`: Represents a pending or completed item in the feed.
    - Fields: `id`, `tenant_id`, `agent_type`, `context_type` (e.g., message, order, schedule), `payload` (JSON), `status` (pending, approved, discarded), `created_at`.
  - Multi-tenancy rule: Strict row-level security on `tenant_id` for all ActionItems. Lock key pattern for concurrency.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks] -->|Ingest| B(Message Bus)
      C[Internal State Events] -->|Ingest| B
      B --> D[Agent Router]
      D --> E{The Ambassador Agent}
      D --> F{The Operations Agent}
      E -->|Draft Action| G[ActionItem DB Table]
      F -->|Draft Action| G
      G -->|Query| H[Agent Feed API]
      H --> I[Mobile App 375px]
      I -->|Approve| J[Execution Engine]
  ```

  ## Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX Flow (375px):**
    - The first screen upon login is the Feed.
    - Glassmorphism cards showing urgent items.
    - Large 44x44px touch targets for "Approve", "Edit", "Discard".
  - **Performance Targets:** The feed must load in under 200ms using edge-caching or fast DB reads.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, when I open OHC on my phone, I see a prioritized feed of actions. The top card says "Drafted reply for @customer: Yes, we have vegan cake." I click "Approve" and it sends immediately.
  **CUJ & Acceptance Criteria:**
  1. Implement the data model for `ActionItem` with multi-tenant RLS.
  2. Implement an API endpoint to fetch pending ActionItems for the current tenant.
  3. Implement a mutation to approve/discard an ActionItem.
  4. Develop the Agent Feed UI component for the 375px mobile view displaying action cards.
  5. Include E2E Playwright tests verifying the approval flow from UI to backend.

  ## Visual Excellence Mandate
  Use macOS-style Translucent Glass materials and UniFi modular dashboard card layouts.

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
