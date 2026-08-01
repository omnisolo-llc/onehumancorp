issue_title: "[Native Chat] Implement Rust-Native Omnichannel Chat & Messaging Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OmniSolo (OHC) currently relies on a legacy mandate to "Retire Chatwoot" but has not yet fully mapped or implemented its native replacement. Non-technical owners (like Maya the Baker or Carlos the Handyman) need a unified inbox where they can receive and reply to Instagram DMs, SMS, WhatsApp, and Web Widget messages without leaving their OHC assistant shell. We must build a high-performance, multi-tenant Rust backend capable of supporting omnichannel conversations, unified contacts, and real-time messaging, replicating Chatwoot’s core data model but doing it within OHC’s strict zero-trust, hybrid architecture.

  ## Research Report
  - **Context**: The `AGENTS.md` and Day 1 directives require total replacement of Chatwoot with a native Rust system in `onehumancorp/mono`.
  - **Source Code Audit (Chatwoot v3.3+)**:
    - *Conversations*: Track state (`open`, `resolved`), `assignee_id`, `contact_id`, and custom attributes.
    - *Messages*: Track `content`, `content_type`, `message_type` (incoming/outgoing), and `sender_type`.
    - *Inboxes*: Act as the bridge between Channels (e.g., WhatsApp, Web Widget) and OHC Accounts (Tenants).
    - *Contacts*: Unified profile containing `phone_number`, `email`, `identifier`, and cross-channel `contact_inbox` links.
  - **OHC Implementation Gap**: OHC requires row-level tenant isolation, so all models must tightly bind to `tenant_id` and integrate with our PostgreSQL schema and Redis/Valkey event bus for real-time WebSocket delivery.

  ## Design Doc
  ### Data Model & Invariants
  1. **Tenants (Implicit)**: All models must include `tenant_id` for strict multi-tenant isolation.
  2. **Inbox**: `id`, `tenant_id`, `name`, `channel_type` (Enum: `WebWidget`, `WhatsApp`, `Sms`, `Instagram`, etc.), `config` (JSONB).
  3. **Contact**: `id`, `tenant_id`, `name`, `email`, `phone_number`, `identifier`.
  4. **ContactInbox (Bridge)**: Links a unified `Contact` to a specific `Inbox` (e.g., Carlos’s WhatsApp number vs. his email). `id`, `contact_id`, `inbox_id`, `source_id` (channel-specific ID).
  5. **Conversation**: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (Open, Resolved, Snoozed), `assignee_id`.
  6. **Message**: `id`, `tenant_id`, `conversation_id`, `content` (Text), `message_type` (Incoming, Outgoing, Template), `status` (Sent, Delivered, Read).

  ### System Architecture
  - **Core Service**: A new Rust microservice/crate `ohc-chat-engine`.
  - **Ingestion**: Webhooks from external providers (Twilio/Meta) hit `ohc-chat-engine`, which normalizes the payload and creates/updates `Message`, `Conversation`, and `Contact`.
  - **Real-Time Delivery**: Updates are published via Redis Pub/Sub (`ohc:events:{tenant_id}`) to the WebSocket gateway, pushing the new message to the Flutter frontend.
  - **AI Integration**: The "Customer Assistant" AI subscribes to new `Conversation` events to draft replies and update CRM notes.

  ### Mobile UX Flow (375px)
  - **Unified Inbox View**: A bottom navigation tab "Messages". Shows a list of recent `Conversations` sorted by `last_activity_at`.
  - **Conversation View**: Clean, translucent glass UI. Left-aligned bubbles for incoming, right-aligned for outgoing. A smart input bar at the bottom with "AI Draft" suggested replies.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Scaffold the Rust data models and PostgreSQL schemas for the native OHC Chat Engine.
  **Tasks**:
  1. Define the SQL migrations for `inboxes`, `contacts`, `contact_inboxes`, `conversations`, and `messages` ensuring `tenant_id` is present on all tables.
  2. Implement the Rust structs (models) mapping to these tables using the existing ORM/DB patterns (e.g., SQLx or Diesel).
  3. Create basic CRUD repository functions for `Conversation` and `Message`.
  4. Ensure 100% unit test coverage for the repository layer.
  **Acceptance Criteria**: The database schema is successfully applied via Bazel, and Rust unit tests pass proving that a message can be created and linked to a conversation and contact under a specific tenant.

  ## Priority & Scope
  - **Priority**: P0 (Blocks all communication capabilities)
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
