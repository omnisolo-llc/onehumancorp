issue_title: "Implement The Operations Manager Agent & Unified Action Feed (Mobile-First)"
issue_description: |
  # Research Report: The Operations Manager Agent & Unified Action Feed (Mobile-First)

  ## 1. Problem Statement
  Small business owners and operators (like Maya the baker and Carlos the field service owner) suffer from cognitive overload when managing multiple streams of demand (DMs, emails, booking requests, inventory alerts). Traditional dashboards present static data that requires the owner to interpret and act upon it. Owners need an assistant that not only unifies these inputs but also proactively analyzes them, drafts the next best actions, and presents them in an easy-to-approve mobile feed. The gap is transitioning from a "reactive dashboard" to a "proactive AI action feed".

  ## 2. Research Report
  ### Market Mapping & Competitor Discovery
  - **Traditional Dashboards (Shopify, Wix):** Excellent at displaying metrics, but require the user to dig into menus to execute actions (e.g., fulfilling orders, replying to customer inquiries, updating inventory). Mobile apps often act as companions, redirecting users to the desktop for complex tasks.
  - **AI-Native Assistants (HubSpot Breeze, Shopify Sidekick):** These are steps in the right direction but often function as conversational chatbots requiring the user to prompt them (e.g., "What should I do today?").
  - **The OHC Opportunity:** OHC must implement a **Unified Action Feed** powered by the Operations Manager Agent. Instead of waiting for prompts, the AI proactively ingests events across the business, categorizes them, drafts responses/actions, and pushes them as 1-tap approval cards on a 375px mobile feed.

  ## 3. Design Doc
  ### Business Journey Mapping
  - **Acquisition/Work Intake:** A lead comes in via Instagram DM or web form.
  - **Activation/Triage:** The Operations Agent parses the intent, checks inventory/calendar, and drafts a reply or quote.
  - **Revenue/Retention:** The agent presents an "Action Card" to the owner on the mobile feed. The owner taps "Approve". The agent sends the response, updates the CRM graph, and schedules any necessary follow-ups.

  ### Data Model & Invariants (PostgreSQL)
  - `AgentActionCard`: Represents a drafted action requiring owner approval.
    - Fields: `id`, `tenant_id`, `source_event_id`, `card_type` (e.g., `draft_reply`, `inventory_alert`, `booking_request`), `status` (pending, approved, discarded), `draft_payload` (JSON of the proposed action).
  - **Multi-Tenant Invariants:** Strict row-level security ensuring `tenant_id` matches the authenticated owner.

  ```mermaid
  erDiagram
      TENANT ||--o{ AGENT_ACTION_CARD : "owns"
      EVENT_MESH ||--o{ AGENT_ACTION_CARD : "triggers"
      AGENT_ACTION_CARD ||--o{ CUSTOMER_GRAPH : "relates to"
      AGENT_ACTION_CARD {
          uuid id
          uuid tenant_id
          string card_type
          jsonb draft_payload
          string status
          timestamp created_at
      }
  ```

  ### AI Department Coordination
  - **Work Triage (Event Ingestion):** Captures DMs, emails, and system alerts.
  - **Customer & Relationship Assistant:** Drafts replies based on customer history and current intent.
  - **Operations Assistant (The Manager):** Verifies inventory availability or calendar slots before the draft is finalized.

  ### Mobile-First UX Flow (375px)
  1. **The Command Center (Home Screen):** The primary view is a vertical, scrollable feed of `AgentActionCard`s, styled with macOS-style Translucent Glass.
  2. **Card Layout:** Each card contains:
     - Context Header: "New Cake Inquiry from @sarah (Insta DM)"
     - Summary Body: "Sarah is asking about vegan chocolate availability for Saturday."
     - Draft Preview: "Hi Sarah! Yes, we have 3 vegan chocolate cakes available..."
     - 1-Tap Actions: Large (>= 44x44px) "Approve & Send", "Edit", and "Dismiss" buttons.
  3. **Interaction:** Tapping "Approve" triggers a fast optimistic UI update (card slides away), sending a confirmation to the backend.

  ### Performance & Security Targets
  - **Zero Trust:** Multi-tenant isolation verified on every feed query. Identity managed via SPIFFE/SPIRE for inter-agent communication.
  - **Offline Tolerance:** Pending approvals are queued locally if offline and synced when reconnected.

  ## 4. Implementation Prompt
  **Feature Name:** Unified Agent Action Feed
  **Target Persona:** Maya the Home Baker

  **User-Facing Outcome:** When Maya opens the OHC app, she sees a prioritized list of tasks (Action Cards) drafted by her AI assistant. She can approve customer replies, accept bookings, and trigger inventory restocks with a single tap, without ever navigating complex menus.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external event (e.g., new DM) is ingested.
  2. The backend Operations Agent classifies the event and generates an `AgentActionCard` in the database.
  3. Maya logs into the mobile app (375px viewport).
  4. The feed displays the translucent Glassmorphism card for the drafted reply.
  5. Maya taps "Approve". The optimistic UI immediately dismisses the card.
  6. The backend receives the approval, updates the card status, and triggers the simulated external dispatch.
  7. **Playwright E2E Tests:** Must cover the full flow from event ingestion to UI rendering, tap interaction, and backend state update. Assertions must verify the 375px responsiveness and the absence of mock data in the UI components.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
