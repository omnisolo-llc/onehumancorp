issue_title: "[Native Chat] Implement Rust Omnichannel Chat Models and Database Schema"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. To provide an AI assistant (The Ambassador) that can seamlessly draft responses considering full customer history, we need a high-performance, native omnichannel chat architecture, as Chatwoot has been retired as a third-party dependency. We need a native Rust implementation of the foundational models for an Omnichannel Chat System to replicate and improve upon the retired Chatwoot dependency.

  # Research Report

  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit**: Investigated `chatwoot/chatwoot` source. Key models include `Message`, `Conversation`, `Inbox`, and `Contact`. They use complex, heavily relational PostgreSQL structures with JSONB fields for flexibility (`additional_attributes`, `custom_attributes`).
  - **OHC Architecture Shift**: The mandate requires full retirement of Chatwoot in favor of a native Rust implementation inside `onehumancorp/mono`. This allows tighter integration with OHC's multi-tenant architecture and AI agent workflows (like The Ambassador).
  - **Data Model Translation**: We need to translate Chatwoot's ActiveRecord models into native Rust models (structs) backed by a PostgreSQL schema optimized for OHC's strict multi-tenancy (`tenant_id` on every table, Row Level Security) and performance goals.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ MESSAGE : owns

      TENANT {
          uuid id PK
      }
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
          string identifier
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
          int message_type
      }
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  - **Home Feed**: Although this ticket focuses on the backend domain models, these models directly back the "1 New Message" notification card on the owner's 375px home feed.
  - **Message View**: When tapped, the `Conversation` and related `Message` models are serialized and presented in a translucent glassmorphism chat UI, showing context pulled from the `Contact` model.

  ### Core Entity Invariants (Multi-Tenancy)
  Every entity (`Inbox`, `Contact`, `Conversation`, `Message`) MUST include a `tenant_id` (UUID). Row Level Security (RLS) policies must be designed around this `tenant_id` to ensure absolute data isolation.

  ### AI Agent Integration Points
  - These foundational models are required for the Event Mesh to trigger The Ambassador agent when a new `Message` is inserted into a `Conversation`.
  - The `Contact` model serves as the root for the Unified Customer Graph DB, allowing AI agents to query past history across different `Inboxes` (channels).

  ### Estimated Scope
  - **Scope:** Medium. This sets the foundation for the entire Native Chat feature.

  # Implementation Prompt

  **User-Facing Outcome**: As a system architecture foundation, this enables the OHC backend to natively store, retrieve, and manage multi-channel customer communications seamlessly without relying on third-party services like Chatwoot.
  **CUJ & Acceptance Criteria**:
  1. Define Rust structs (domain models) for `Inbox`, `Contact`, `Conversation`, and `Message` within a new module `src/server/ohc/domain/chat/`.
  2. The models must include `tenant_id` for multi-tenant isolation.
  3. Include JSONB mapping fields for extensibility (e.g., `custom_attributes`).
  4. Write comprehensive unit tests verifying model instantiation, serialization, and deserialization.
  5. The PR must be focused on domain models and their immediate unit tests. Database migrations and service layer implementation can follow in subsequent PRs, but the domain structs must clearly reflect a PostgreSQL-backed design.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
