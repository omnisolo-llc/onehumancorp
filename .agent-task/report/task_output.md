issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The system currently relies on an external Chatwoot dependency which is being fully retired. Small-business owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) need a lightning-fast, native, unified inbox to manage customer inquiries across Instagram DMs, WhatsApp, web chat, and SMS from a single interface. External dependencies increase latency, add operational overhead, and break our zero-trust, tightly integrated AI agent coordination model.

  **Research Report**
  As mandated by the Engineering Standards, Chatwoot must be replaced by a native Rust implementation. An analysis of the Chatwoot source code (v3+ architecture) reveals its core components:
  - Multi-tenant Account/Inbox structure.
  - Channels (Web Widget, API, WhatsApp, Email, etc.).
  - Conversations (threading, status: open/resolved/snoozed, assignees, SLAs).
  - Messages (types: incoming, outgoing, template, attachments).
  - Contacts (cross-channel identity resolution).

  OHC's native Rust implementation needs to replicate this core omnichannel data model and the channel adapter pattern, ensuring it integrates natively with our AI Triage Agent and existing `tenant_id` Row Level Security (RLS) policies.

  **Design Doc**

  *Architecture Overview:*
  - **Data Model:** Native Rust representations of `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`. All strongly typed and strictly bound by `tenant_id`.
  - **Real-time Gateway:** Actix/Axum WebSockets (native Rust) to power the frontend inbox and web chat widgets with sub-50ms latency.
  - **AI Agent Integration:** The Triage Agent hooks into the message creation lifecycle via a background job queue (Rust native or Redis-backed). When a message arrives, it evaluates context and optionally drafts a reply or executes an action (e.g., booking a calendar slot).
  - **Multi-Tenancy:** PostgreSQL with RLS (`tenant_id` on every table). Distributed locks (Redis) for concurrent message processing per conversation.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|{ CHANNEL : uses
      TENANT {
          uuid tenant_id PK
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone_number
      }
      CHANNEL {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string provider_type
      }
  ```

  *UX Flow (Mobile-First 375px):*
  - **Home Screen:** "Unified Inbox" card showing unread counts and AI-drafted responses pending approval.
  - **Conversation View:** Standard chat interface. Messages clearly indicate origin (WhatsApp vs. Web). AI drafts appear in a translucent glass container just above the composer, requiring one tap to approve/send.
  - **Offline/Flaky Network:** Optimistic UI updates. Messages queue locally in Flutter and sync when reconnected.

  **Implementation Prompt**
  Implement the core Rust data models and database migrations for the native Omnichannel Chat System.
  1.  **Database Schemas (Migrations):** Create PostgreSQL tables for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure `tenant_id` is present on all tables and RLS policies are applied. Replicate the essential fields from Chatwoot (e.g., status, assignee, channel type).
  2.  **Rust Models & Repositories:** Implement the corresponding Rust structs in `src/server/integrations/chat/domain.rs` (or appropriate module) and the repository layer for CRUD operations, respecting multi-tenancy.
  3.  **Channel Adapter Trait:** Define a Rust trait `ChannelAdapter` with methods for receiving and sending messages, preparing for future specific implementations (WhatsApp, Web, etc.).
  4.  **Tests:** Write comprehensive unit tests for the models and repository layer ensuring RLS and multi-tenant constraints cannot be bypassed.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
