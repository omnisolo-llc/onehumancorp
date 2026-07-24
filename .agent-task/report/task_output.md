issue_title: "[Architecture] Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a native, high-performance, multi-tenant omnichannel chat and inbox system. Relying on Chatwoot as an external third-party service is 100% RETIRED. Small business owners (like Maya the Baker or Carlos the Handyman) need a unified inbox that brings together Instagram DMs, WhatsApp, SMS, Web Chat, and Emails without jumping between apps. The architecture must natively support real-time WebSocket messaging, AI agent interventions, and strict multi-tenant row-level security.

  ## Research Report
  - **Market Benchmark**: Chatwoot provides an excellent open-source blueprint for data models (Accounts, Users, Contacts, Inboxes, Channels, Conversations, Messages) and features (Assignment rules, Canned Responses, SLAs, Macros).
  - **Codebase Audit (Chatwoot)**:
    - `Inbox`: Represents a collection channel for messages. Has polymorphic `channel_id` and `channel_type`.
    - `Conversation`: Belongs to `Inbox`, `Account`, and `Contact`. Has status (`open`, `snoozed`, `resolved`), priorities, and assignees.
    - `Message`: Belongs to `Conversation`. Holds content, attachments, and message type (incoming, outgoing, activity).
    - `Channels`: Separated into models like `channel_web_widget`, `channel_email`, `channel_whatsapp`, `channel_instagram`, etc.
  - **Gap Analysis**: OHC's current Rust backend (inside `src/server/ohc` or similar) lacks a comprehensive, native implementation of these models, real-time sync, and channel adapters.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - **PostgreSQL Database**: Multi-tenant tables (`ohc_inboxes`, `ohc_conversations`, `ohc_messages`, `ohc_contacts`, `ohc_channel_configs`). All enforce RLS via `tenant_id`.
    - **Rust Microservice/Module (`ohc_chat`)**: Handles CRUD for models, manages WebHook ingress from providers (Meta, Twilio), and processes AI agent handoffs.
    - **Real-time Engine**: Rust-based WebSocket server (using `axum` or `tungstenite` integrated with Redis Pub/Sub) to push new messages to the Flutter/PWA clients instantly.
  - **Mobile UX Flow**:
    - **Unified Inbox Screen (375px)**: A list of open conversations. Avatar indicates channel (WhatsApp icon, IG icon). Swipe actions to snooze/resolve.
    - **Conversation View**: Chat interface. AI suggestions appear above the composer. Native keyboard integration.
  - **AI Agent Integration**:
    - **Operations Assistant / Work Triage**: Intercepts incoming messages via an asynchronous job queue (PostgreSQL `SKIP LOCKED`). If the assistant can answer based on knowledge base, it drafts a reply (or auto-replies if authorized). Handoffs assign the conversation to the human owner.

  ## Implementation Prompt
  Implement the core multi-tenant backend schema and Rust service layer for the native omnichannel inbox system.
  1. Define the SQL schema migrations for `inboxes`, `channels`, `conversations`, `messages`, and `contacts`, strictly enforcing multi-tenant RLS (Row-Level Security).
  2. Implement the Rust models and basic repository logic for these entities.
  3. Create an internal API (gRPC or REST depending on the current standard) to create an inbox, start a conversation, and send a message.
  4. Ensure 100% unit test coverage for the repository layer.
  *Note: Focus on the backend foundation. Channel-specific webhooks and the WebSocket layer will be separate PRs.*

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, backend]
assignees: []
