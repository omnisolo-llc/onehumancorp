issue_title: "Implement OHC Native Omnichannel Inbox Architecture"
issue_description: |
  # Native Omnichannel Inbox Architecture

  ## Problem Statement
  Currently, OneHumanCorp relies on external chat providers (or unused legacy Chatwoot integration stubs) rather than a native, tenant-isolated, offline-capable omnichannel inbox. Non-technical owner/operators (like Maya the baker or Carlos the handyman) need a single, reliable assistant-led feed that unifies Instagram DMs, SMS, WhatsApp, and web widget inquiries, without juggling multiple apps or learning complex helpdesk tools. The current fragmented approach lacks a canonical data model, transactional delivery outbox, and strict multi-tenant isolation required to ensure that business owners see a trustworthy, unified customer conversation history.

  ## Research Report
  - **Market Context**: Platforms like Shopify Sidekick, Tencent Workbuddy, and Wix unify merchant communications into a single interface. Chatwoot's source code (cloned during research) relies heavily on Ruby on Rails abstractions, ActionCable for WebSockets, and Sidekiq for background jobs.
  - **OHC Requirement**: We require a high-performance, strictly multi-tenant Rust backend with PostgreSQL (cloud) and SQLite (desktop/local) support. Chatwoot as an external service is officially retired, per architectural mandates (`2026-07-13-native-omnichannel-chat-design.md`).
  - **Core Findings**: The native foundation exists in OHC but suffers from fragmented persistence (`inbox_messages`, `omni_inbox_messages`), incomplete delivery state machines, and a lack of a transactional outbox. A unified canonical domain model must be built from the ground up, utilizing PostgreSQL Row Level Security (RLS) for multi-tenant isolation.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Tenant ||--o{ Conversation : owns
      Inbox ||--o{ ChannelConnection : has
      Contact ||--o{ ContactIdentity : has
      Conversation ||--o{ Message : contains
      Message ||--o{ Attachment : has
      Message ||--o{ Receipt : tracks
      ChannelConnection }|--|| Conversation : routes
      Contact }|--|| Conversation : participates
  ```

  ### Core Data Entities & Invariants
  - **Tenant**: Row-level isolation using `tenant_id` on every table (PostgreSQL RLS).
  - **Inbox**: A tenant-owned routing boundary.
  - **ChannelConnection**: Encrypted provider configurations (Meta, Twilio, etc.).
  - **Contact & ContactIdentity**: Canonical customer records mapping to channel-specific IDs.
  - **Conversation**: The support thread linking Contacts to an Inbox.
  - **Message**: Immutable inbound/outbound content with ordering, delivery state (`draft`, `committed`, `redacted`, `deleted`), and attachments.
  - **Delivery Outbox & Receipts**: Transactional outbox pattern for outbound messages to ensure at-least-once delivery with idempotency keys.

  ### Mobile UX Flow (375px First)
  1. **Triage Feed**: Owner opens the app and sees a unified list of active conversations, prioritized by recent activity or AI-flagged urgency.
  2. **Conversation View**: Clean, translucent glass UI (Apple/Ubiquiti style). Messages are distinct bubbles.
  3. **AI Assist**: A persistent "Magic Wand" floating action button suggests drafting a reply based on business context (e.g., "Draft quote for vegan cake").
  4. **Status Indicators**: Truthful delivery statuses ("Sending...", "Delivered", "Read") based on actual provider receipts, not local enqueuing.

  ### AI Agent Integration
  - **Customer Assistant**: Listens to the `Conversation` feed, retrieves relevant `KnowledgeArticle` or previous `Contact` history via tenant-scoped memory, and prepares draft replies in the `Message` table with state `draft`.
  - **Work Triage**: Analyzes incoming messages and can auto-route to specific Inboxes or escalate urgent issues to the owner's push notifications.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the canonical data model and repository layer for the Native Omnichannel Inbox in Rust.
  1. Define the core structs: `Inbox`, `ChannelConnection`, `Contact`, `ContactIdentity`, `Conversation`, `Message`, `Attachment`, and `Receipt`.
  2. Implement the persistence layer ensuring strict multi-tenant isolation (all queries MUST include `tenant_id`).
  3. Implement the transactional outbox pattern for `Message` creation, ensuring that a message and its corresponding delivery job are committed atomically.
  4. Create the necessary gRPC/REST API definitions (Protobuf/OpenAPI) for the frontend to fetch conversations and send messages.
  5. **Acceptance Criteria**:
     - Unit test coverage is 100% for the new domain models and repository logic.
     - Integration tests verify that queries for Tenant A cannot access Tenant B's data.
     - The transactional outbox successfully queues a background job upon message creation.
     - Playwright E2E tests (if UI is included) verify the conversation feed rendering with truthful delivery states.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []