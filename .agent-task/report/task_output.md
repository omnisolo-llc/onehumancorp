issue_title: "[Architecture] Native Rust Omnichannel Chat & Support Engine"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot as an external dependency. We need a native, high-performance, multi-tenant Rust architecture to handle omnichannel chat (Instagram, WhatsApp, SMS, Web Widget), conversations, inboxes, and messaging.
  Small business owners like Maya (baker using IG DMs) and Carlos (handyman using SMS) need a unified "Inbox" that brings all these channels into one view, with real-time updates and AI agent coordination.

  ## Research Report
  - **Chatwoot Audit**: Analyzed `app/models/conversation.rb`, `message.rb`, `inbox.rb`, and `channel/*` in Chatwoot.
  - **Chatwoot Data Model**:
    - `Inbox` links a `Channel` (Web Widget, API, FB, IG, Twilio, WhatsApp) to an `Account` (Tenant).
    - `Conversation` links a `Contact` with an `Inbox` and `Assignee`.
    - `Message` belongs to a `Conversation` and supports various `message_type`s (incoming, outgoing, template).
  - **Competitors**: Shopify Inbox, Meta Business Suite, Wix Inbox unify messaging.
  - **OHC Architecture Fit**: We need this built in Rust, leveraging `tonic` for gRPC APIs or Axum for REST/WebSockets, backed by PostgreSQL with strict Row-Level Security (RLS) on `tenant_id`.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--o| INBOX : configures

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb channel_config
          boolean is_active
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK "nullable"
          string status "open, snoozed, resolved"
          timestamp last_activity_at
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id FK "nullable"
          string sender_type "contact, agent, bot"
          string content
          string message_type "incoming, outgoing"
          jsonb external_source_ids "for webhook mapping"
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: A unified list view of conversations. A single tab that shows IG DMs, SMS, and Web chat together.
  2. **Conversation View**: Similar to iMessage/WhatsApp. Sticky input at the bottom, auto-expanding text area.
  3. **Real-time**: WebSockets pushing new messages instantly without pull-to-refresh.
  4. **AI Agent Drafts**: Ghost-text or distinct UI blocks showing proposed AI replies (e.g. for Maya's vegan cake inquiries).

  ### AI Agent Integration Points
  - **Operations Assistant / Work Triage**: Analyzes incoming `MESSAGE` to suggest a reply or create a task.
  - **Customer Assistant**: Listens to new `CONVERSATION` creation to pull CRM data and propose a first response.
  - **Agent Handoff**: `CONVERSATION.assignee_id` can be set to an AI Bot ID initially, then handed off to a human user when confidence is low or user intervenes.

  ### Key Design Decisions
  - **Native Rust Axum/Tonic Service**: High concurrency for WebSockets and webhook parsing.
  - **Strict Multi-Tenancy**: `tenant_id` on every table, enforced by Postgres RLS.
  - **Channel Adapters as Traits**: Define a Rust `ChannelAdapter` trait that Twilio, Meta, and Webhooks implement, transforming provider payloads into our standard `Message` struct.
  - **WebSockets**: Redis Pub/Sub for real-time delivery across scaled instances.

  ## Implementation Prompt
  Implement the native Rust omnichannel chat backend.
  1. Create the database migrations for `inboxes`, `conversations`, and `messages` with `tenant_id` RLS.
  2. Implement the CRUD gRPC/REST endpoints for Inboxes and Conversations.
  3. Implement the `Message` creation endpoint that handles Webhook ingestion (stub out the Meta/Twilio parsing, focus on the core data persistence first).
  4. Implement a WebSocket endpoint for real-time message streaming to the frontend.
  5. Ensure 100% unit test coverage for the services and database operations.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
