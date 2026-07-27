issue_title: "Native Rust Omnichannel Chat: Data Model & Repository Layer"
issue_description: |
  # Mission Queue Protocol Report

  ## Title
  Native Rust Omnichannel Chat: Data Model & Repository Layer

  ## Problem Statement
  The external dependency on Chatwoot for omnichannel messaging has been entirely retired to reduce cost, simplify operations, and increase tenant data locality, aligning with our mobile-first, single-platform architecture. Business owners (like Carlos the handyman and Maya the baker) require a highly reliable, natively integrated unified inbox to handle customer requests from WhatsApp, Instagram, and web chat. To fulfill this, we must replace Chatwoot's core data models and repository layers with a native Rust implementation inside the `ohc-mono` backend.

  ## Research Report
  - **Context:** The `chatwoot` legacy system was officially removed per `docs/superpowers/plans/2026-07-13-chatwoot-removal.md`.
  - **Source Audit:** Based on reviewing Chatwoot's models (`inbox.rb`, `conversation.rb`, `message.rb`, `contact.rb`), the core domains involve tenant-isolated inboxes, channels, conversations, messages, and contacts.
  - **Current OHC Stack:** We use a Go and Rust hybrid backend (multi-tenant PostgreSQL/SQLite). The spec mandates deterministic tenant isolation and native implementation.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX {
          string id
          string tenant_id
          string name
          string channel_type
      }
      CONTACT {
          string id
          string tenant_id
          string name
          string identifier
      }
      CONVERSATION {
          string id
          string tenant_id
          string inbox_id
          string contact_id
          string status
      }
      MESSAGE {
          string id
          string tenant_id
          string conversation_id
          string content
          string sender_type
      }
  ```

  ### Mobile UX Flow
  - While this task focuses on the backend data model, these structures directly power the 375px mobile inbox view where an owner sees conversations sorted by recent message timestamp.

  ### AI Agent Integration Points
  - **Operations/Customer Success Agent:** The data models must include metadata fields (or extension tables) to track AI drafting status, so that when a webhook inserts a message, the AI agent can transition the message state to `draft_pending_approval`.

  ### Key Design Decisions
  - **Row-Level Security (RLS) & Multi-Tenancy:** Every entity (`Inbox`, `Contact`, `Conversation`, `Message`) MUST include a `tenant_id` column.
  - **Language Choice:** Rust is specified for the native chat engine implementation to ensure high-performance concurrency.
  - **Idempotency & Auditing:** Message insertion must be idempotent based on provider message IDs to handle webhook retries gracefully.

  ## Implementation Prompt
  Implement the core database schema (SQL migrations) and Rust repository interfaces (structs and traits) for the Native Omnichannel Chat system.
  1. Create the SQL migrations for `inboxes`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` is present on all tables for strict tenant isolation.
  2. Implement the Rust data models/structs reflecting these tables.
  3. Implement the Repository traits (e.g., `InboxRepository`, `MessageRepository`) providing standard CRUD operations, specifically ensuring every query requires and filters by `tenant_id`.
  4. Write comprehensive unit tests for the repository layer demonstrating successful insertion, querying, and multi-tenant isolation (preventing cross-tenant data access). Do NOT implement API endpoints or business logic yet; focus entirely on the robust data layer.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
