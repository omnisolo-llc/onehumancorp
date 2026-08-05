issue_title: "Architecture: Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a robust, high-performance omnichannel communication system to allow owners (like Maya the Baker or Carlos the Handyman) to manage interactions with customers seamlessly. Currently, OHC relies on Chatwoot as an external dependency. This adds infrastructure complexity, latency, limits tight integration with our internal AI agents (like Work Triage and Customer Assistant), and violates the Zero-Trust multi-tenant data boundaries we require.

  We need to replace the external Chatwoot dependency with a native, high-performance, and multi-tenant-safe omnichannel chat engine built in Rust. It must seamlessly handle real-time WebSockets, multiple channel integrations (Instagram, WhatsApp, Email, Web Widget), and unify conversations into an intuitive mobile-first Inbox.

  ## Research Report
  - **Context**: The `AGENTS.md` mandate explicitly dictates the complete retirement of Chatwoot as a third-party service and requires a native Rust implementation achieving 100% feature parity.
  - **Chatwoot Architecture Audit**: We analyzed the `chatwoot` open-source repository's data model and system design.
      - **Key Entities**: `accounts` (tenants), `inboxes` (channel entry points), `conversations` (threads), `messages` (items within a thread), `contacts` (customers), and `channel_*` (adapter specific configurations).
      - **Communication**: Heavy reliance on WebSockets (ActionCable) for real-time updates and sidekiq for background processing (webhooks, email processing).
  - **OHC Persona Needs**:
      - **Maya**: Receives Instagram DMs for cake orders. Needs an inbox that pulls these in, allows the AI agent to draft replies or handle FAQs, and lets her step in natively from her phone.
      - **Carlos**: Needs a web widget on his site for quote requests, routing to a unified inbox on his Android phone.

  ## Design Doc

  ### 1. Data Model & Invariants (Rust / PostgreSQL)
  We will introduce a new Rust crate or module in the backend for the `chat` domain, utilizing our existing PostgreSQL database and Redis for Pub/Sub.

  - **`Tenant` Isolation**: Every table MUST include `tenant_id` and utilize PostgreSQL Row Level Security (RLS).
  - **Core Entities**:
      - `Inbox`: Configuration for a specific channel (e.g., "Main Website Widget", "Maya's Bakery IG").
      - `ChannelAdapter`: Polymorphic configuration linking an `Inbox` to a specific platform (e.g., WhatsApp, Instagram, Email, WebWidget).
      - `Contact`: A unified customer profile across channels.
      - `Conversation`: A thread of messages between a `Contact` and the business (Tenant), tied to an `Inbox`.
      - `Message`: Individual messages within a `Conversation`. Supports attachments, rich text, and AI-generated drafts.

  ### 2. Real-time Architecture (Rust)
  - **WebSockets**: We will use a Rust async WebSocket framework (e.g., `axum` + `tokio-tungstenite`) to handle real-time client connections.
  - **Pub/Sub (Redis)**: When a new message arrives via webhook (e.g., from Instagram), the Rust backend persists it and publishes an event to Redis (`ohc:chat:events:{tenant_id}:{conversation_id}`). WebSocket handlers subscribed to these topics will push updates to the connected owner's mobile/web app instantly.

  ### 3. AI Agent Integration
  - **Work Triage**: Subscribes to new `Conversation` events. Evaluates intent, tags the conversation, and prioritizes it in the owner's feed.
  - **Customer Assistant**: Listens for new `Message` events in unassigned conversations. Drafts proposed replies (stored as a specific message type or draft state) and notifies the owner for approval, or auto-replies if confidence is high and policy allows.

  ### 4. Mobile-First UX Flow
  - **375px Viewport**:
      - **Inbox List**: Clean, translucent-glass styled list of active conversations, prioritized by Work Triage. Badges indicate unread status or pending AI drafts.
      - **Conversation View**: Familiar chat interface. A prominent, bottom-anchored input area. AI drafts appear seamlessly within the stream with a distinct "AI Draft" visual indicator (e.g., slight tint or sparkle icon) and quick "Approve & Send" or "Edit" actions.
      - **Interactions**: Swipe actions on the inbox list to resolve, archive, or snooze conversations.

  ### 5. Mermaid Architecture Diagram
  ```mermaid
  graph TD
      Client[Mobile/Web App 375px] -->|WebSocket/REST| API[Rust API / WebSocket Gateway]
      API -->|RLS Protected| DB[(PostgreSQL)]
      API <-->|Pub/Sub Events| Redis[(Redis)]

      WebhookExt[External Webhooks: IG, WA, Email] -->|REST| WebhookHandler[Rust Webhook Ingestion]
      WebhookHandler --> DB
      WebhookHandler -->|Publish| Redis

      Redis -->|Subscribe| AIAgents[AI Agent Workers]
      AIAgents -->|Read Context, Draft Reply| DB
      AIAgents -->|Publish Draft Event| Redis
  ```

  ## Implementation Prompt
  **Goal**: Implement the foundational data models, API endpoints, and a basic WebSocket echo/broadcast system for the native Rust omnichannel chat system.

  **Tasks**:
  1. Define PostgreSQL schema (using migrations or ORM definitions) for `inboxes`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` RLS is strictly applied.
  2. Implement Rust struct models and repository methods (CRUD) for these entities.
  3. Create REST API endpoints to list inboxes, conversations for an inbox, and messages for a conversation.
  4. Setup a basic WebSocket endpoint in the Rust backend that allows a client to authenticate (via tenant context), subscribe to a conversation, and broadcast new messages to connected clients using Redis Pub/Sub.

  **Acceptance Criteria**:
  - 100% Unit test coverage on models and API endpoints.
  - E2E Playwright test proving a user can open a conversation, send a message, and receive a real-time update via WebSocket.
  - All database interactions correctly enforce multi-tenant isolation.

  ## Priority & Scope
  - **Priority**: P0 (Blocks critical customer interaction capabilities)
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
