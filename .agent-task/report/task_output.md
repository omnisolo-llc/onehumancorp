issue_title: "Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)

  ## Mission Queue Protocol Brief
  **Problem Statement:**
  OneHumanCorp (OHC) currently lacks a fully featured, highly scalable native omnichannel chat system. Historically, platforms often rely on third-party integrations like Chatwoot, but OHC mandates 100% retirement of external Chatwoot dependencies. Our core personas—Maya (baker managing Instagram DMs), Carlos (handyman handling service inquiries), and Nora (agency principal coordinating client emails)—need a unified, AI-augmented inbox. They need to seamlessly triage messages, manage conversations across multiple channels (Web Widget, Email, Instagram, WhatsApp), and utilize AI for auto-drafting replies without managing multiple separate tools.

  **Research Report:**
  - **Codebase Audit:** OHC's backend includes Rust microservices (`src/server/ohc/domain`). There are existing Postgres migrations (e.g., `150_unified_inbox_triage.sql` and `20260701_omnichannel_tables.sql`) which introduce basic concepts like `unified_threads`, `unified_messages`, `work_item`, and `agent_draft`, but a comprehensive domain model reflecting a true omnichannel contact center (like Chatwoot) in Rust is missing.
  - **External Benchmarking:** An audit of Chatwoot's source code (`chatwoot/app/models/conversation.rb`, `inbox.rb`, `contact.rb`, `message.rb`) reveals a robust architecture supporting multi-tenancy (`account_id`), status management (`open`, `resolved`, `snoozed`), priority handling, SLA tracking, and assignment policies.
  - **Gap:** OHC needs to build equivalent robust data models, repository layers, and service boundaries in Rust that leverage the Postgres row-level security (`tenant_id`) and integrate with the AI agent workflow (e.g. `agent_draft` tables).

  ## Architecture Design

  **1. High-Level Architecture (Rust `ohc` microservice)**
  - **Domain Module:** `src/server/ohc/domain/chat`
  - **Data Entities:**
    - `Conversation` (matches Chatwoot's core Conversation, extending `unified_threads`)
    - `Message` (extends `unified_messages`)
    - `Inbox` (channels setup: Web, Email, Social)
    - `Contact` (Customer profiles)
  - **Multi-Tenancy:** Guaranteed via PostgreSQL Row-Level Security (`tenant_id`) as established in our migrations.
  - **State Management:** Enums for Conversation Status (`Open`, `Resolved`, `Pending`, `Snoozed`) and Priority (`Low`, `Medium`, `High`, `Urgent`).

  **Mermaid Diagram (Conceptual ER)**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ AGENT_DRAFT : has

      CONVERSATION {
          UUID id
          UUID tenant_id
          UUID inbox_id
          UUID contact_id
          String status
          String priority
          DateTime created_at
      }

      MESSAGE {
          UUID id
          UUID tenant_id
          UUID conversation_id
          String sender_type
          String content
      }
  ```

  **2. Mobile UX Flow (375px First)**
  - **Unified Inbox View:** A sticky bottom nav. The main screen is a list of open conversations sorted by `last_activity_at`. High contrast unread indicators.
  - **Conversation Thread View:** Standard chat bubbles. The AI draft appears dynamically at the bottom as a "Proposed Reply" card with "Approve" (swipe right) or "Edit" (tap) actions.
  - **Translucent Glass UI:** Follows OHC Premium Token library with Apple/Ubiquiti aesthetics. Overlays for assigning or snoozing a chat use a bottom-sheet modal.

  **3. AI Agent Integration Points**
  - **Customer Assistant Agent:** Listens to new `Message` inserts via Postgres/event stream. Automatically drafts replies based on the `Conversation` context and tenant memory.
  - **Operations Agent:** Parses messages for intent (e.g. "I want a vegan cake") and creates associated `work_item`s or updates the `Conversation` priority.

  ## Implementation Prompt (For Implementer Agent)
  **Objective:** Implement the core Rust domain layer for the new Omnichannel Chat system within `src/server/ohc/domain/chat`.

  **Tasks:**
  1. Create the module structure `src/server/ohc/domain/chat` containing `mod.rs`, `models.rs`, `repository.rs`, and `service.rs`.
  2. In `models.rs`, define the Rust structs for `Conversation`, `Message`, `Inbox`, and `Contact`. Ensure they map conceptually to the database schemas defined in migrations (e.g., `unified_threads`, `unified_messages`, `customer_profile`). Include Enums for `ConversationStatus` (Open, Resolved, Pending, Snoozed).
  3. In `repository.rs`, define the trait/interface for database operations (e.g., `create_conversation`, `get_messages_for_conversation`) ensuring `tenant_id` is a required parameter for all operations to enforce RLS isolation.
  4. In `service.rs`, implement a basic service struct that orchestrates creating a conversation and adding a message, triggering an event (or returning a result) that the AI agent system can later consume.
  5. Write comprehensive unit tests (100% coverage) for the domain logic in these new files.

  **Acceptance Criteria:**
  - Code compiles cleanly with `bazel build //...`.
  - Unit tests pass with `bazel test //...`.
  - The domain design supports multi-tenancy and maps to the expected non-technical owner capabilities (triaging messages, viewing AI drafts).

  ## Priority & Scope
  - **Priority:** P0 (Critical foundational architecture for removing Chatwoot dependency)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
