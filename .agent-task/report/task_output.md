issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OHC must replace external dependencies with a native, high-performance omnichannel chat system built in Rust. This system must handle multi-tenant isolation, real-time WebSocket messaging, omnichannel adapters (Instagram DMs, Web Chat, WhatsApp), and seamless AI agent handoffs. A non-technical owner like Maya (baker) needs a unified inbox where she receives Instagram DMs, AI auto-replies while she sleeps, and can manually intervene effortlessly—all from her phone without knowing about the underlying channels.

  ## Research Report
  - **Context**: The existing project relies on external Ruby on Rails models like `Conversation`, `Message`, `Contact`, `Inbox`, and various `Channel` definitions. Our `src/server/services/chat/models.rs` currently implements a basic skeleton of this but lacks deep integration with real-time events, agent routing, webhook management, and UI multi-tenancy.
  - **Market Validation**: Competitors like Shopify Inbox, GoDaddy Conversations, and Intercom unify channels into one stream. OHC's key differentiation is the native AI agent integration that acts as the "first responder".
  - **System Constraints**: Must be built in Rust inside `onehumancorp/mono`. Row-level tenant isolation via `tenant_id` is mandatory. Real-time updates must be low-latency for mobile-first consumption.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client(Mobile / Web PWA) -- WebSocket / REST --> Gateway
      Gateway -- Routing --> ChatService(Native Rust Chat Service)

      subgraph Replacement Engine
          ChatService --> InboxManager
          ChatService --> ChannelAdapters
          ChatService --> ConversationEngine
      end

      ChannelAdapters -- Webhooks --> External(Instagram, WhatsApp)
      ChannelAdapters -- Web Widget --> GuestUser(Storefront Guest)

      ConversationEngine -- CRUD --> DB[(PostgreSQL)]
      ConversationEngine -- Publish --> Redis(Redis PubSub)
      Redis -- Subscribe --> Gateway

      ConversationEngine -- Event --> AITriage(AI Triage Agent)
      AITriage -- Reply / Handoff --> ConversationEngine
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: A simple list view showing active conversations across all channels, tagged visually by source (e.g., small IG icon) and status (Unread, Bot Handling, Needs Owner).
  2. **Conversation Thread**: Familiar chat bubble interface. Bot replies are subtly differentiated (e.g., distinct background color or "AI" badge).
  3. **Action Bar**: At the bottom, standard composer with attachment support. An "AI Draft" button allows the owner to one-tap generate a response based on context.

  ### Key Design Decisions
  - **Data Model**: Extend `src/server/services/chat/models.rs` to include robust WebSocket event tracking (`last_seen_at`), message delivery status (`sent`, `delivered`, `read`), and `custom_attributes` (JSONB) for extensible channel data.
  - **Multi-Tenancy**: Every table query must enforce `tenant_id`. Database RLS policies will back this up.
  - **Real-Time**: Utilize Axum/Tonic with Tokio websockets for the client connection, backed by Redis PubSub to distribute events across load-balanced Rust instances.

  ## Implementation Prompt
  **Goal**: Implement the core backend controllers and real-time WebSocket scaffolding for the native Rust omnichannel chat system, replacing external dependencies.

  **CUJ (Critical User Journey)**:
  As Maya, I receive an Instagram DM. My OHC app immediately receives a WebSocket event and updates the Unified Inbox. My AI agent drafts a reply. I review the draft, tap "Send", and the message is dispatched through the Rust backend.

  **Acceptance Criteria**:
  1. Define fully featured Axum/Tonic API endpoints for Inbox, Conversation, and Message CRUD, ensuring strict `tenant_id` isolation.
  2. Implement a WebSocket endpoint that authenticated mobile/web clients can subscribe to for real-time `message.created` and `conversation.updated` events.
  3. Integrate basic Redis PubSub to broadcast message creation events to the WebSocket handler.
  4. Ensure 100% unit test coverage for the new handlers and services.
  5. Add at least 5 Playwright E2E tests for the new functionalities.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
