issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OHC aims to provide an all-in-one work assistant for owners and operators. A core part of this is handling omnichannel communication (web chat, Instagram DMs, SMS, etc.). The previous approach relied on an external Chatwoot service, which is a Ruby on Rails application. This introduces complex deployment, state management, latency, and operational overhead. The goal is to fully retire Chatwoot and implement a high-performance, strictly multi-tenant native Rust omnichannel chat system inside the OHC monolith (`onehumancorp/mono`), guaranteeing tenant isolation, real-time sync, and seamless integration with OHC AI agents.

  ## Research Report
  Based on auditing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture models around:
  - **Account/Tenant**: Root of isolation.
  - **Inbox**: A container for conversations routing from channels.
  - **Channel**: The adapter for external services (WebWidget, Twilio SMS, Email, API, Instagram, FB Page, etc.).
  - **Contact & ContactInbox**: Unifies customer identities across multiple channels.
  - **Conversation**: The thread of communication between a Contact and Agents/Bots.
  - **Message**: Individual payloads (text, attachments, templates) within a conversation.
  - **AgentBot**: Automation/AI hooks intercepting or assisting conversations.

  OHC needs this exact model replicated in Rust with `tenant_id` enforced on every record to align with our PostgreSQL Row Level Security (RLS) model. The system needs to support real-time WebSocket push (replacing ActionCable with `tokio-tungstenite`/Axum WS) and background job processing for webhooks and AI agent integration.

  ## Design Doc

  ### Architecture Overview
  - **Data Layer (PostgreSQL & SQLx)**:
    - New tables (or schema migrations for existing ones in `src/server/services/chat/models.rs`):
      - `chat_inboxes` (`tenant_id`, `name`)
      - `chat_channels` (`tenant_id`, `inbox_id`, `type`, `config_jsonb`)
      - `chat_contacts` (`tenant_id`, `identifier`, `name`, `email`, `phone`)
      - `chat_contact_inboxes` (`tenant_id`, `contact_id`, `inbox_id`, `source_id`)
      - `chat_conversations` (`tenant_id`, `inbox_id`, `contact_id`, `status`, `assignee_id`)
      - `chat_messages` (`tenant_id`, `conversation_id`, `sender_type`, `sender_id`, `content_type`, `content`, `metadata_jsonb`)
  - **Service Layer (Rust)**:
    - Domain services for handling Inbox routing, Channel adapters (Web, API, SMS), and Conversation state.
    - AI Agent Intercept: When a message arrives, a background job (via PostgreSQL SKIP LOCKED queue) is dispatched to trigger the `Customer & Relationship Assistant` agent to draft replies or take automated actions based on AI routing rules.
  - **API & Real-time Layer (Axum & WebSockets)**:
    - REST endpoints for the Flutter/Web UI to fetch inboxes, conversations, and messages.
    - Axum WebSocket routes (`/api/v1/chat/cable` or similar) using Redis Pub/Sub backplane (`server_integrations_pubsub`) to broadcast new messages instantly to connected clients (both mobile and desktop).
  - **Multi-tenant Security**:
    - Every database operation MUST pass through an RLS-enforced connection or explicitly filter by `tenant_id` from the authenticated request context (SPIFFE/SPIRE/JWT).

  ### AI Agent Integration
  - **Event Bus**: The chat service publishes a `ChatMessageCreated` event.
  - **Agent Coordinator**: Listens to new inbound messages. If the conversation is unassigned or assigned to a bot, the agent reads the context (tenant memory, conversation history) and proposes an action (e.g., drafts a reply, updates customer preferences, triggers a booking flow).

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed**: New messages appear as actionable items in the main feed.
  - **Conversation View**: Full-screen view on mobile. Header shows Contact info. Scrollable message list with clear visual distinction between customer messages, agent replies, and AI-drafted (pending approval) messages.
  - **Input Area**: Text input with attachments button. Above the input, chips for "Approve AI Draft" or "Quick Reply" appear when relevant.
  - The UI uses premium OHC Design Tokens (translucent glass, Apple/Ubiquiti style cards).

  ## Implementation Prompt
  1. **Schema Migration**: Implement the SQL migrations to create the core omnichannel data model (Inboxes, Channels, Contacts, ContactInboxes, Conversations, Messages) with strict `tenant_id` isolation.
  2. **Domain Models & Services**: Update `src/server/services/chat/models.rs` and create a comprehensive service layer (`src/server/services/chat/service.rs`) implementing CRUD and routing logic.
  3. **WebSocket Real-time**: Implement a secure Axum WebSocket handler that subscribes to a Redis Pub/Sub topic per `tenant_id` and broadcasts `message.created` and `conversation.updated` events.
  4. **API Endpoints**: Implement the REST API for fetching the chat state.
  5. **Verification**: Write full unit test coverage for the domain logic and at least 3 Playwright E2E tests verifying the real-time chat flow (Customer sends message -> Agent receives it -> Agent replies) using a test channel adapter. Ensure no mocks are used for the database or internal APIs.

  ## Scope & Priority
  - **Priority**: P0 (Critical path for core product capability and deprecating legacy external service).
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
