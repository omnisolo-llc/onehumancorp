issue_title: "Native Omnichannel Chat: Data Model & Architecture Design"
issue_description: |
  # Native Omnichannel Chat: Architecture & Data Model Design

  ## Problem Statement
  OHC needs to fully replace Chatwoot with a native, high-performance omnichannel support system written in Rust to support the personas (e.g., Maya, Carlos, Priya) effectively without relying on an external third-party integration. The personas rely on varied channels (Instagram, WhatsApp, Web widgets) and require a seamless, tenant-safe inbox experience that integrates deeply with OHC's AI agents. We must implement matching native Rust microservices and data models reflecting Chatwoot's core abstractions, but optimized for OHC's `tenant_id` Row Level Security patterns.

  ## Research Report
  - We audited the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`), particularly its Active Record models: `Conversation`, `Message`, `Inbox`, `Channel`, `Contact`, `Account`.
  - Chatwoot handles multi-tenancy via `account_id` integer. OHC will handle this natively via PostgreSQL Row-Level Security (RLS) and a `tenant_id` column for rigorous tenant isolation.
  - Chatwoot relies on Ruby on Rails + PostgreSQL + Redis. OHC leverages Rust + PostgreSQL + Valkey (Redis) ensuring extremely high throughput and low resource footprints suitable for edge or limited footprint deployments.

  ## Design Doc

  ### Data Model & Invariants
  1. **Tenancy:** Every table MUST have a `tenant_id` column to enforce strict Row Level Security (RLS).
  2. **Inbox:** Represents a unified grouping for channels. `id`, `tenant_id`, `name`, `greeting_message`, `channel_type`.
  3. **Contact:** End customer. `id`, `tenant_id`, `name`, `email`, `phone_number`, `avatar_url`, `custom_attributes`.
  4. **Conversation:** A thread with a customer. `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `assignee_id`.
  5. **Message:** A message within a conversation. `id`, `tenant_id`, `conversation_id`, `sender_type` (User, Contact, AgentBot), `sender_id`, `content`, `content_type`, `message_type` (incoming, outgoing, template), `private` (internal note).

  ### Architecture Blueprint (Mermaid.js)
  ```mermaid
  erDiagram
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : has
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
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
          uuid sender_id
          string sender_type
          string content
          boolean private
      }
  ```

  ### Mobile-First UX Flow (375px)
  - **Unified Inbox List View:** Clean list of active conversations sorted by most recent activity. Unread indicators prominently displayed (UniFi-style badge).
  - **Conversation Thread View:** Translucent glass header containing customer name and channel source icon. Sticky bottom input bar with auto-resizing text area, attachment button, and a prominent "Send" button.
  - **AI Agent Integration:** AgentBot drafts are displayed in a distinct color (e.g., subtle blue background) with a "Review & Send" action for the human owner to approve before dispatching.

  ### Zero Trust & Security
  - Multi-tenant data access is protected by standard OHC Row Level Security policies. Cross-tenant pollution is structurally impossible at the Postgres schema level.

  ## Implementation Prompt
  **Mission:** Implement the core database schema migrations and Rust proto definitions for the native Omnichannel Chat models (`Inbox`, `Contact`, `Conversation`, `Message`), ensuring strict multi-tenant RLS isolation via `tenant_id`.

  **Acceptance Criteria:**
  1. Write SQL migrations to create the core tables mirroring Chatwoot's primary entities but adapted to OHC standards (`uuid`, `tenant_id`, `RLS`).
  2. Implement protobuf definitions (`.proto`) for gRPC communication.
  3. Ensure all tests (`bazel test //...`) pass with 100% coverage on the new schemas.

  ## Priority & Scope
  **Priority:** P0
  **Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
