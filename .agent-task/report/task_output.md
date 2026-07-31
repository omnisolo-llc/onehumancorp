issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The system currently relies on an external chat dependency which is being fully retired. Small-business owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) need a lightning-fast, native, unified inbox to manage customer inquiries across Instagram DMs, WhatsApp, web chat, and SMS from a single interface. External dependencies increase latency, add operational overhead, and break our zero-trust, tightly integrated AI agent coordination model.

  **Research Report**
  As mandated by the Engineering Standards, the external chat provider must be replaced by a native Rust implementation. An analysis of the core omnichannel data model reveals its core components:
  - Multi-tenant Account/Inbox structure.
  - Channels (Web Widget, API, WhatsApp, Email, etc.).
  - Conversations (threading, status: open/resolved/snoozed, assignees, SLAs).
  - Messages (types: incoming, outgoing, template, attachments).
  - Contacts (cross-channel identity resolution).

  OHC's native Rust implementation needs to replicate this core omnichannel data model and the channel adapter pattern, ensuring it integrates natively with our AI Triage Agent and existing `tenant_id` Row Level Security (RLS) policies.

  **Design Doc**

  *Architecture Overview:*
  - **Data Model:** Native Rust representations of `Inbox`, `Conversation`, `Message`, `Contact`, and channel abstraction. All strongly typed and strictly bound by `tenant_id`.
  - **Real-time Gateway:** Native Rust WebSockets implementation to power the frontend inbox and web chat widgets with sub-50ms latency.
  - **AI Agent Integration:** The Triage Agent hooks into the message creation lifecycle via a background job queue. When a message arrives, it evaluates context and optionally drafts a reply or executes an action (e.g., booking a calendar slot).
  - **Multi-Tenancy:** PostgreSQL with RLS (`tenant_id` on every table). Distributed locks for concurrent message processing per conversation.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|{ CHANNEL : uses
  ```

  *UX Flow (Mobile-First 375px):*
  - **Home Screen:** "Unified Inbox" card showing unread counts and AI-drafted responses pending approval.
  - **Conversation View:** Standard chat interface. Messages clearly indicate origin (WhatsApp vs. Web). AI drafts appear in a translucent glass container just above the composer, requiring one tap to approve/send.
  - **Offline/Flaky Network:** Optimistic UI updates. Messages queue locally in Flutter and sync when reconnected.

  **Implementation Prompt**
  Implement the core Rust data models and database migrations for the native Omnichannel Chat System.
  1.  **Database Schemas (Migrations):** Create database tables for the unified inbox data model. Ensure multi-tenant tracking is present on all tables and RLS policies are applied. Replicate the essential fields required for the feature functionality.
  2.  **Rust Models & Repositories:** Implement the corresponding data structs and the repository layer for CRUD operations, respecting multi-tenancy rules and avoiding strict coupling with database engines.
  3.  **Channel Abstraction:** Create a mechanism for receiving and sending messages that allows multiple providers (WhatsApp, Web, etc.).
  4.  **Tests:** Write comprehensive unit tests for the models and repository layer ensuring RLS and multi-tenant constraints cannot be bypassed.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
