issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native, high-performance, and multi-tenant omnichannel chat and customer support engine. To serve our core owner/operator personas (Maya the baker, Carlos the handyman, Priya the boutique owner, etc.), we need an embedded system that unifies messaging from Instagram, Facebook, WhatsApp, Email, Web Widgets, and SMS into a single owner feed. As mandated, we are 100% retiring external reliance on Chatwoot and must implement its equivalent functionality natively in Rust within the OHC repository.

  ## Research Report
  I performed a comprehensive audit of the `chatwoot/chatwoot` source repository (specifically `app/models/*`). Key findings for the native replication include:
  1. **Inboxes & Channels**: Chatwoot abstracts integrations via `Inbox` mapping to specific `Channel::Api`, `Channel::WebWidget`, etc. This allows standardizing multi-channel input.
  2. **Conversations & Messages**: `Conversation` tracks the lifecycle, SLA, and assignee. `Message` stores the content, type (text, attachment), and handles threaded replies.
  3. **Contacts**: The `Contact` model unifies customer identity across channels via phone, email, and identifier.
  4. **Multi-Tenancy**: All models strictly enforce multi-tenancy with `account_id` (equivalent to OHC's `tenant_id`).

  We need to replicate this core omnichannel data model and the channel abstraction layer in Rust using our existing `sea-orm` and `sqlx` stack.

  ## Design Doc
  ### High-Level Architecture
  - **Core Entities**:
    - `Tenant` (existing)
    - `Inbox` (Unifies channel configurations)
    - `ChannelConfig` (Polymorphic configuration for WebWidget, Email, API, WhatsApp, Meta)
    - `Contact` (The human customer)
    - `Conversation` (The session linking a Contact to an Inbox/Tenant)
    - `Message` (The actual chat lines, supporting attachments and rich content)
  - **Multi-Tenant Isolation**: Strict row-level security using `tenant_id` on every table.
  - **AI Agent Integration Points**: The Work Triage and Customer & Relationship Assistant will monitor the `Message` stream to draft replies and categorize intents automatically.

  ### Mobile UX Flow
  - 375px viewport first: A unified "Inbox" tab.
  - List of conversations ordered by SLA and priority.
  - Conversation view with bubble layout, quick-reply AI drafts, and attachment support.

  ## Implementation Prompt
  Implement the core database schema, SeaORM entities, and repository layer for the Native Rust Omnichannel Chat System.
  1. Define the SeaORM entities for `Inbox`, `ChannelConfig`, `Contact`, `Conversation`, and `Message` in `src/server/ohc/domain/chat/`.
  2. Ensure every new table includes `tenant_id` and is strictly scoped to the tenant.
  3. Create the database migration applying these schemas.
  4. Write comprehensive unit tests for the repository methods demonstrating multi-tenant isolation.
  5. Provide a basic gRPC service definition (proto) for fetching conversations and sending messages.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
