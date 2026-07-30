issue_title: "Migrate external Chatwoot dependency to Native Rust Omnichannel Inbox"
issue_description: |
  # Problem Statement
  Small business owners need to unify communications (Instagram DMs, WhatsApp, SMS, Email, Web Chat) into a single inbox where AI agents can proactively draft context-aware replies. Currently, the platform conceptualizes this via third-party integrations or heavy dependencies like Chatwoot. However, relying on an external Chatwoot service introduces latency, complex multi-tenant data syncing, disjointed identity resolution, and prevents deep, real-time AI agent integration (The Ambassador) directly at the database level. For non-technical owners like Maya (baker) or Carlos (handyman), the system must "just work" invisibly behind the scenes, drafting replies based on historical context without them needing to configure external webhook pipelines.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot (External):** Excellent omnichannel capabilities and data models, but running it as a separate service creates a data silo. Syncing tenant customer data, orders, and interactions back and forth is brittle.
  - **Shopify Inbox:** Aggregates chat but lacks deep, proactive AI drafting based on cross-channel history.
  - **Wix Inbox:** Similar aggregation, but AI is limited to tone improvement rather than autonomous agentic workflows.
  - **OHC Opportunity:** As mandated by the engineering standards, we must 100% RETIRE Chatwoot as an external dependency. We need a native Rust implementation of a high-performance omnichannel inbox that mirrors Chatwoot's proven data model (Inboxes, Conversations, Messages, Contacts, Channel Adapters) but runs directly inside the `onehumancorp/mono` architecture. This allows zero-latency access to the unified customer graph for our AI agents.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Channels: IG, WA, Email] -->|Webhooks| B(Native Rust Channel Adapters)
      B --> C{Conversation Engine}
      C --> D[(Unified Postgres DB: Inboxes, Conversations, Messages, Contacts)]
      D --> E{Customer Identity Resolution}
      C --> F[Event Bus / PubSub]
      F --> G[The Ambassador Agent]
      G -->|Reads context & Drafts| D
      D --> H[Rust API Layer]
      H --> I[OHC Mobile App / Web UI 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The "Unified Inbox" isn't just a list of raw messages; it's a prioritized feed. Card: "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens the conversation view. Top half displays CRM context (past orders, lifetime value). Bottom half displays the chat history, with a pre-filled AI-drafted reply sitting in the composer if applicable.
  - **Action:** Primary button: "Send Draft". Secondary: "Edit".
  - **Visual Design:** Translucent Glass materials, clean typography, distinct message bubbles (user vs. customer), clear indication if a message is an AI draft vs. human sent. Native keyboard integration.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Listens to new `MessageCreated` events. If the message requires a response, it queries the newly native Rust database tables for past context, drafts a reply, and inserts it as a `Draft` message type, alerting the owner UI.

  ### Key Design Decisions
  - **Native Rust Port:** Implement the core domain models of Chatwoot in Rust (using SQLx/Diesel or similar ORM as per repo standards) to live inside our unified Postgres database.
  - **Schema Alignment:** We will model tables heavily inspired by Chatwoot (`inboxes`, `conversations`, `messages`, `contacts`, `channel_web_widgets`, etc.) but strictly enforce our row-level security `tenant_id` pattern.
  - **Real-time:** Implement WebSocket endpoints in Rust for real-time delivery of new messages to the UI, replacing Chatwoot's ActionCable implementation.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, when I receive a WhatsApp message from a customer, I see it instantly in the OHC app. The system has already linked it to their past purchase history and drafted a contextual reply, without relying on any external third-party chat software. I tap "Send Draft" and get back to work.

  **CUJ & Acceptance Criteria:**
  1.  **Backend (Rust):** Implement the core data models for the native inbox: `inboxes`, `contacts`, `conversations`, `messages`. Ensure strict multi-tenant (`tenant_id`) isolation.
  2.  **API (Rust):** Create REST (or gRPC) endpoints to list inboxes, fetch conversations for an inbox, and fetch messages for a conversation.
  3.  **Real-time (Rust):** Set up a basic WebSocket handler to broadcast `message_created` events to connected clients for a specific tenant/conversation.
  4.  **Frontend (Flutter/Web):** Build the 375px mobile-first UI for the Inbox list and the Conversation view, integrating with the new native Rust APIs.
  5.  **Agent Trigger:** Ensure that when a new message is inserted, an event is emitted that the AI job queue can pick up to draft a response.
  6.  **E2E Testing:** Write a Playwright test where a simulated webhook creates a new message, a user logs into the UI, navigates to the inbox, sees the message, and sends a reply back through the native API.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
