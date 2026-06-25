issue_title: "[Backend] Omni-Channel AI Inbox & Triage Action System"
issue_description: |
  # Research Report: Omni-Channel AI Inbox & Triage Action System

  ## Problem Statement
  Small business owners are overwhelmed by communication across multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email). A core value proposition of OneHumanCorp (OHC) is the "Invisible AI Automation"—where agents don't just read messages, but draft replies and prepare operational actions (like creating a booking or sending an invoice). While foundational database migrations exist (Migration 150: `unified_threads`, `unified_messages`, `unified_triage_actions`), there is a critical gap: the backend service layer and AI agent integration to actually process these messages, generate triage actions, and present them for owner approval in a mobile-first feed are not fully realized or are disconnected.

  ## Research Report
  Our competitive analysis (Track 1) shows that traditional tools (Shopify Inbox, Wix Inbox) are purely reactive chat aggregators. To fulfill the OHC "Assistant-First" vision, we must implement an *active* inbox. The AI must act as "The Ambassador" (Customer Success Agent). When a message arrives, it shouldn't just send a notification; it should query the RAG (Retrieval-Augmented Generation) context (inventory, calendar, policies), draft a response or an operational action (e.g., "Customer wants a cake -> Draft Invoice + Reply"), and present it as a `Triage Action`. This architectural gap (Track 3) prevents the mobile app from acting as a true "Approval UI" (Track 4).

  ## Design Doc
  - **Architecture Diagram**:
    - **Ingestion Layer**: Webhooks from Instagram/WhatsApp/Email land at a unified endpoint.
    - **Service Layer (`src/server/services/inbox`)**:
      - Normalizes messages into `unified_messages` and groups by `unified_threads`.
    - **AI Agent Department (`The Ambassador`)**:
      - Triggers asynchronously on new thread/message.
      - Intent Classification (e.g., "Availability Inquiry", "Support Issue").
      - RAG Query (check inventory/calendar based on intent).
      - Generates a `TriageAction` (e.g., `DraftReply`, `CreateBooking`).
      - Inserts into `unified_triage_actions` table.
    - **Frontend API**:
      - Mobile app fetches pending `unified_triage_actions` to render in the Agent Feed.
      - Appends user approval/rejection back to the API.
  - **Mobile UX Flow (375px)**:
    - User opens app to the "Agent Feed".
    - A Glassmorphism card appears: "Maya, 3 customers asked about Vegan Cakes. They are in stock. I drafted replies."
    - User taps the card to expand the thread context.
    - User taps a large (44px+) "Approve & Send" button.
  - **Multi-Tenant & Security**:
    - All queries to `unified_*` tables MUST enforce `app.current_tenant` RLS.
    - Distributed Lock (Redis Redlock) on the thread ID during Agent processing to prevent race conditions if the user replies manually while the AI is drafting.

  ## Implementation Prompt
  Implement the backend Core Service for the Omni-Channel AI Inbox and Triage Actions.
  - **Data Models**: Ensure Go/Rust structs exist for `unified_threads`, `unified_messages`, and `unified_triage_actions`.
  - **Service Logic (`inbox_service`)**:
    - Implement methods to ingest messages, creating threads automatically if they don't exist, respecting tenant boundaries.
    - Implement a method to fetch pending triage actions for the mobile feed.
    - Implement a method to resolve (approve/reject/edit) a triage action, which executes the underlying intent (e.g., sending the actual message via the channel adapter).
  - **AI Integration Point**: Create a background task or hook that is triggered when a new message is ingested, which simulates calling "The Ambassador" agent to generate a draft `TriageAction`.
  - **Testing**: Write unit tests that mock the AI call, ingest a message, verify the triage action is created, and verify the approval flow updates the database correctly. All tests must pass `bazel test //...`.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
