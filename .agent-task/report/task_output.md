issue_title: "Implement Native Omnichannel Chat Architecture"
issue_description: |
  ## Problem Statement
  The current implementation of the omnichannel inbox relies on an incomplete foundation with three competing persistence models (`inbox_messages`, `omni_inbox_messages`, and `unified_*`), incomplete delivery state, detached outbound tasks, uneven channel coverage, insecure development fallbacks on some webhooks, incomplete attachment handling, and no unified operational contract. Additionally, the external dependency on Chatwoot has been retired, but the native Rust replacement is not fully implemented across the core domain entities and multi-tenant constraints.

  ## Research Report
  - **Market Context**: Platforms like Shopify, Stripe, and modern support CRMs unify messaging through a single robust domain model handling multi-tenancy, deterministic state machines for message delivery, and real-time synchronization.
  - **Codebase Context**: Chatwoot integrations have been fully removed per the `docs/superpowers/plans/2026-07-13-chatwoot-removal.md` plan. The target architecture is defined in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`.
  - **Current Implementation**: The existing `InboxService` (`src/server/services/inbox/service.rs`) uses `unified_threads`, `unified_messages`, and `unified_triage_actions`. This needs to be expanded to match the canonical domain model specified in the design document (Inbox, ChannelConnection, Contact, ContactIdentity, Conversation, Participant, Message, Attachment, Receipt).
  - **Reference**: Chatwoot's core data models (`app/models/`) were analyzed (Inbox, Conversation, Message, Contact, ContactInbox, Channel::*) to understand the necessary features to replicate natively in Rust, including multi-tenant routing, SLA tracking, agent assignment, and message content typing.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ ChannelConnection : has
      Tenant ||--o{ Contact : owns
      Contact ||--o{ ContactIdentity : has
      Inbox ||--o{ Conversation : routes
      Contact ||--o{ Conversation : initiates
      Conversation ||--o{ Participant : includes
      Conversation ||--o{ Message : contains
      Message ||--o{ Attachment : attaches
      Message ||--o{ Receipt : tracks
  ```

  ### System Components
  1. **Canonical Database Schema**: Implement the canonical domain entities (Inbox, ChannelConnection, Contact, ContactIdentity, Conversation, Participant, Message, Attachment, Receipt) in PostgreSQL using `uuid` primary keys and strict `tenant_id` references for Row-Level Security (RLS).
  2. **Rust Domain Layer (`src/server/services/inbox`)**: Implement repositories and services for managing the lifecycle of these entities. Ensure all queries filter by `tenant_id` or utilize PostgreSQL's `set_config('app.current_tenant', ...)` for RLS.
  3. **Message Ingestion Pipeline**: Refactor the current `ingest_message` logic to map to the new `Conversation` and `Message` entities, handling `Contact` creation/lookup via `ContactIdentity`.
  4. **State Machine**: Implement a delivery outbox and receipt state machine for outbound messages.

  ### AI Agent Integration
  - The `trigger_ai_triage` function should interact with the new `Conversation` and `Message` structures, drafting replies or applying labels/assignments based on tenant-specific `AutomationRule`s.

  ### Mobile UX & Security
  - The API layer mapping to these services must support local-first synchronization via PowerSync, ensuring the Next.js and Tauri clients have low-latency access to the `Conversation` list and `Message` history.
  - All endpoints and database queries strictly enforce `tenant_id` isolation.

  ## Implementation Prompt
  Implement the native omnichannel chat domain model and repository layer in Rust.
  1. **Database Migrations**: Create SQL migrations to define the new canonical tables (`omni_inboxes`, `omni_channel_connections`, `omni_contacts`, `omni_contact_identities`, `omni_conversations`, `omni_participants`, `omni_messages`, `omni_attachments`, `omni_receipts`). Ensure all tables have a `tenant_id` column, appropriate indexes, and RLS policies enabled.
  2. **Rust Models**: Define the corresponding Rust `struct`s with `sqlx::FromRow` and `serde` serialization in `src/server/services/inbox/models.rs`.
  3. **Repository Updates**: Update `src/server/services/inbox/service.rs` to use the new canonical tables instead of the legacy `unified_*` tables. Implement methods for creating inboxes, resolving contacts, creating conversations, and persisting messages.
  4. **Testing**: Write comprehensive unit tests for the updated `InboxService` and ensure integration tests verify tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
