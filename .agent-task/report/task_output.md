issue_title: "Implement Agent-Driven Work Triage & Unified Action Feed Orchestration"
issue_description: |
  ## Title
  Agent-Driven Work Triage & Unified Action Feed Orchestration

  ## Problem Statement
  Currently, small business owners (like Maya the baker and Carlos the handyman) suffer from "Notification Fatigue" and fragmented operations. When a customer DMs on Instagram, books a service slot, or completes a deposit payment, these events exist in separate silos. The owner has to manually triage these events, figure out the context, and decide what to do next. Traditional platforms (like Shopify or Wix) provide dashboards but do not synthesize these events into a single, actionable work feed. Non-technical owners need an assistant that ingests all demand and operational signals, categorizes them, drafts the next step (e.g., a reply or a booking confirmation), and presents them in a unified 375px mobile feed for 1-tap approval.

  ## Research Report
  - **Market Context**: Legacy platforms (Shopify, Wix) rely on complex admin panels and third-party apps to manage communications and operations. Shopify's "Sidekick" is a reactive chatbot rather than a proactive workflow engine.
  - **Competitor Gaps**: Link-in-bio tools (Linktree, Stan Store) offer simplicity but lack operational depth (like inventory and service dispatch). Traditional helpdesks (Zendesk, Intercom) are too complex for micro-SMBs.
  - **Product-Use Evidence**: Running the OHC local docker-compose stack and interacting via the Tauri/Next.js prototype reveals that while individual services (pos, chat, ops) exist, there is no centralized message bus or unified intent resolution pipeline. A user checking the app currently sees fragmented data rather than a unified "What needs my attention right now?" feed.
  - **Conclusion**: We need an `Agent Feed` orchestrator that acts as the central nervous system. This component will implement the "Invisible AI Automation" vision by turning raw events into synthesized `Action Cards`.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      subgraph Ingestion Layer
          IG[Instagram Webhook] --> EventBus
          Stripe[Stripe Webhook] --> EventBus
          Book[Booking API] --> EventBus
      end

      subgraph AI Orchestration (Work Triage)
          EventBus[Redis Pub/Sub Event Bus] --> Router[Event Router]
          Router --> Intent[LLM Intent & Context Resolution]
          Intent --> DB[(Tenant RAG Memory)]
          DB --> Intent
          Intent --> ActionGenerator[Action Card Generator]
      end

      subgraph Mobile Feed & Action
          ActionGenerator --> Feed[PostgreSQL Agent Feed Table]
          Feed --> App[Flutter / PWA 375px Mobile App]
          App -- "1-Tap Approve" --> Execution[Service Layer Execution]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Screen (The Feed)**: The user opens the OHC app. Instead of a dashboard, they see a vertical scrolling list of `Action Cards`.
  2. **Card Structure**: Each card has a unified glassmorphism design (OHC Premium Tokens) with at least 44x44px touch targets.
  3. **Interaction**:
     - *Card Example*: "Maya, you received 3 new DMs about custom cakes. I've drafted replies based on our Saturday availability."
     - *Actions*: [Review & Approve] | [Edit] | [Dismiss]
  4. **Approval Flow**: Tapping "Approve" triggers an optimistic UI update, immediately removing the card from the feed and dispatching the execution event to the respective service.

  ### AI Agent Integration Points
  - **Event Router**: Subscribes to tenant-scoped Redis channels.
  - **Intent Classifier**: Uses `OHC_LLM_PROVIDER` (Gemini Pro/GPT-4o) to evaluate incoming payloads and determine the urgency and required action.
  - **Departmental Agents**: The orchestrator routes the classified intent to the appropriate sub-agent (e.g., Customer Assistant for DMs, Operations Assistant for bookings) to draft the action card.

  ### Key Design Decisions
  - **Event-Driven & Asynchronous**: Using Redis Pub/Sub and PostgreSQL `SKIP LOCKED` job queues to ensure the UI remains snappy and background generation doesn't block critical paths.
  - **Tenant Isolation**: All events, RAG queries, and Action Cards must strictly enforce `tenant_id` at the database and memory boundaries.

  ## Implementation Prompt
  **User-Facing Outcome**: As an owner (Maya or Carlos), when I open the OHC mobile app, I see a prioritized feed of action cards (e.g., drafted customer replies, required booking confirmations). I can approve or edit these actions with a single tap on my phone.

  **Critical User Journey (CUJ)**:
  1. Ensure the backend services (chat, booking) emit standardized events to the Redis event bus.
  2. Implement the `Agent Feed Orchestrator` service that listens to these events, invokes the LLM to classify intent and draft an action, and persists an `ActionCard` record in the database.
  3. Create an API endpoint (`GET /api/v1/feed`) to serve these action cards for a given tenant.
  4. Implement the frontend Mobile Feed UI (375px viewport) displaying these cards.
  5. Implement the execution endpoint (`POST /api/v1/feed/:id/execute`) that processes the user's approval.

  **Acceptance Criteria**:
  - The Event Router correctly triggers LLM draft generation for a mocked incoming customer message.
  - The generated Action Card appears in the Mobile Feed UI.
  - Tapping "Approve" successfully executes the drafted action.
  - Must include Playwright E2E tests verifying the end-to-end flow from event ingestion to feed approval.
  - 100% unit test coverage for new backend orchestrator logic.
  - Zero mock data in the UI; all feed items must come from the actual database.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
