issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is an AI work assistant for owners and operators. A core requirement is unifying messages across multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email) into a single inbox for the owner. Previously, this capability might have relied on third-party services like Chat system, which is now 100% RETIRED according to the OHC Engineering Standards.

  Relying on external systems for core communication introduces latency, fractures multi-tenant isolation, complicates AI agent coordination, and fails to meet OHC's offline-tolerant and strict Zero-Trust requirements. Our users (like Maya the baker, who manages custom-order inquiries via Instagram DMs, or Carlos the field service owner, who needs missed-lead recovery via SMS) require a deeply integrated, high-performance, and native omnichannel communication system.

  This task plans out the implementation of a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside `onehumancorp/mono`.

  ## Research Report
  - **Goal**: Implement a fully native omnichannel chat engine in Rust, replacing any reliance on Chat system or external services.
  - **Capabilities Required**:
      - Unified Inbox for organizing conversations.
      - Omnichannel support (Web widget, Email, SMS, WhatsApp, Instagram, FB Messenger).
      - Multi-tenant isolation (strict row-level security per tenant).
      - Real-time WebSocket messaging.
      - Agent assignment (human and AI agents).
      - Canned responses, macros, and SLA policies.
      - Integration with OHC AI agents for automated responses and triage.
  - **Competitor/Reference Architecture (Chat system)**: Chat system uses a relational model with Accounts (Tenants), Users, Inboxes, Channels (polymorphic associations for different platform integrations like `Channel::WebWidget`, `Channel::Email`), Conversations, and Messages. It heavily uses WebSockets (ActionCable in Rails) for real-time updates and Sidekiq for background jobs (webhooks, email processing).
  - **OHC Translation**: We need to translate these concepts into high-performance Rust:
      - PostgreSQL for relational data with `tenant_id` and RLS on every table.
      - Axum for REST APIs and WebSocket handling.
      - Redis (Valkey) for pub/sub across WebSocket nodes and distributed locking.
      - Tokio/PostgreSQL `SKIP LOCKED` for reliable background job processing (webhooks, SLA breaches).
      - gRPC for internal service-to-service communication if split into microservices, or within the monolith boundary.

  ## Design Doc
  ### Architecture
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ AGENT_ASSIGNMENT : has
      TENANT ||--o{ MACRO : has
      TENANT ||--o{ SLA_POLICY : has
  ```

  **Core Data Entities (PostgreSQL):**
  - `chat_inboxes`: Grouping mechanism (e.g., "Support", "Sales").
  - `chat_channels`: Specific integration instances (e.g., "WhatsApp Number A", "Web Widget B"). Polymorphic or JSONB configuration depending on channel type.
  - `chat_contacts`: Customers communicating with the business. Unified across channels via phone/email matching.
  - `chat_conversations`: A thread of messages between a contact and the business within an inbox.
  - `chat_messages`: Individual messages (incoming or outgoing). Supports rich media attachments.
  - *All tables MUST include `tenant_id` and enforce Row Level Security (RLS).*

  **System Components:**
  1. **API Layer (Axum)**: REST endpoints for managing inboxes, channels, sending messages, and Webhook receivers for external providers (e.g., Meta Graph API).
  2. **Real-time Layer (Axum + WebSockets + Valkey Pub/Sub)**: Handles connected clients (mobile/web frontend). Subscribes to Valkey channels formatted as `ohc:tenant:{tenant_id}:chat:events` to push new messages and conversation updates instantly.
  3. **Job Queue (PostgreSQL SKIP LOCKED)**:
      - Processing incoming webhooks asynchronously to avoid dropping payloads.
      - Sending outgoing messages to external APIs with retries.
      - Evaluating SLA policies and triggering AI agents for auto-replies.
  4. **AI Agent Integration**: A specific job type or internal event triggers the "Customer Assistant" AI to draft a reply or triage a new conversation based on tenant context.

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: Bottom nav "Inbox". Shows a list of recent conversations, sorted by last activity. Badges for unread counts. Clear visual indicator of channel source (WhatsApp icon, Web icon).
  2. **Conversation View**: Tap a conversation. Classic chat interface. Scroll up to load older messages. Input area at the bottom with options to attach files, use a canned response (macros), or toggle "Internal Note" vs "Reply to Customer".
  3. **AI Drafts**: If the AI Assistant has drafted a reply, it appears as a distinct card above the input area with "Approve", "Edit", or "Discard" actions.
  4. **Contact Context**: A drawer or slide-over showing contact details, past orders, and tags, critical for the owner to have full context without leaving the chat.

  ### AI Agent Integration Points
  - **Triage**: New conversation created -> Event emitted -> AI Agent analyzes first message -> Applies tags (e.g., "urgent", "custom order"), assigns to specific inbox/human.
  - **Auto-Drafting**: New incoming message -> AI Agent checks context (past messages, knowledge base) -> Generates a draft reply -> Saves as a pending message for human review (or auto-sends if configured).
  - **Summarization**: Long conversation -> AI Agent generates a brief summary for the owner when they open it.

  ## Implementation Prompt
  Implement the foundation of the Native Rust Omnichannel Chat System. This is Phase 1, focusing on the core data models, REST API for conversation management, and the Web Widget channel backend.

  **Critical User Journey (CUJ):**
  A business owner (Maya) configures a new Web Widget channel in her OHC workspace. A customer visits her public storefront (served via OHC) and sends a message through the widget. The message is saved, and Maya can view the new conversation and reply to it via the OHC backend API (simulating the mobile app action).

  **Acceptance Criteria:**
  1.  **Database Schemas (Migrations)**: Create PostgreSQL tables for `chat_inboxes`, `chat_channels` (support type: `web_widget`), `chat_contacts`, `chat_conversations`, and `chat_messages`. MUST include `tenant_id` and strict RLS policies on all tables.
  2.  **Rust Service Layer**: Implement core CRUD operations and business logic in `src/server/services/chat` (or similar appropriate module). Ensure strict multi-tenant isolation in all queries.
  3.  **API Endpoints (Axum)**:
      - Create Inbox & Channel config (Admin).
      - Public endpoint to receive a message from the Web Widget (authenticates via channel token, not tenant auth).
      - Admin endpoints to list conversations, view messages in a conversation, and send a reply.
  4.  **Unit Tests (100% Coverage)**: Thoroughly test the service layer, especially multi-tenant data isolation and channel token validation.
  5.  **E2E Playwright Test**: Implement a test covering the CUJ: Create tenant -> setup inbox/web channel -> simulate public client sending message -> verify message appears in admin API/UI flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
