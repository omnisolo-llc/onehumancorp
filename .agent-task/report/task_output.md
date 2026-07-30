issue_title: "Implement Custom Rust Omnichannel Chat System Core Models"
issue_description: |
  **Title**: Implement Custom Rust Omnichannel Chat System Core Models

  **Problem Statement**: OHC currently does not have a native messaging/omnichannel capability and historically relied on third-party services like Chatwoot. As per OHC requirements, Chatwoot integration is 100% RETIRED, and OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. We need to design and implement the core Rust data models and DB schemas (using SQLx/PostgreSQL) for the native chat system, mirroring Chatwoot's omnichannel concepts but optimized for OHC's multi-tenant SaaS architecture. This includes Inboxes, Channels, Conversations, Messages, and Contacts.

  **Research Report**:
  - We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its data model.
  - Chatwoot uses an `Account` as the tenant (equivalent to OHC's `tenant_id`).
  - Core entities:
    - `Inbox`: A routing destination for messages.
    - `Channel`: The specific integration (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Whatsapp`, etc.).
    - `Contact`: The customer interacting with the business.
    - `Conversation`: A thread of messages between a Contact and the business (Inbox/Agents).
    - `Message`: The individual chat bubbles (text, attachments, template messages).
  - OHC's backend is Rust (with Bazel, PostgreSQL, Redis, Kubernetes) and needs to enforce row-level tenant isolation using `tenant_id` on every table.
  - The goal for this initial architectural phase is to design the core database schema (migrations) and Rust domain models for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.

  **Design Doc**:
  - **Architecture diagram**:
    ```mermaid
    erDiagram
        Tenant ||--o{ Inbox : owns
        Tenant ||--o{ Contact : owns
        Inbox ||--|{ Channel : has_one
        Inbox ||--o{ Conversation : has
        Contact ||--o{ Conversation : initiates
        Conversation ||--o{ Message : contains

        Tenant {
            uuid id PK
            string name
        }
        Inbox {
            uuid id PK
            uuid tenant_id FK
            string name
        }
        Channel {
            uuid id PK
            uuid inbox_id FK
            string channel_type "e.g., web_widget, email"
            jsonb provider_config
        }
        Contact {
            uuid id PK
            uuid tenant_id FK
            string name
            string email
            string phone_number
        }
        Conversation {
            uuid id PK
            uuid tenant_id FK
            uuid inbox_id FK
            uuid contact_id FK
            string status "open, resolved, snoozed"
        }
        Message {
            uuid id PK
            uuid tenant_id FK
            uuid conversation_id FK
            uuid sender_id "can be contact_id or user_id"
            string sender_type "contact or agent"
            text content
            string message_type "incoming, outgoing, template"
        }
    ```
  - **Data Isolation**: All tables will include a `tenant_id` column. We will leverage PostgreSQL RLS (Row-Level Security) to ensure queries are automatically scoped to the active tenant context.
  - **AI Agent Integration**: The `Message` and `Conversation` models will have hooks (via Redis/Postgres SKIP LOCKED queue) so that AI Agents can process `incoming` messages, draft responses, and auto-reply based on OHC configuration.
  - **Mobile UX Flow (Conceptual for future UI)**: The UI will display a unified Inbox where Maya or Carlos can see DMs and emails in one 375px-optimized feed.
  - **Key Design Decisions**: We separate `Inbox` (the logical grouping) from `Channel` (the physical provider configuration), matching Chatwoot's flexible architecture but using strong UUIDs and explicit OHC multi-tenancy.

  **Implementation Prompt**:
  As an Implementer agent:
  1. Create a new Rust crate/module for `chat` or `omnichannel` within the OHC mono repo structure.
  2. Write SQL migrations (PostgreSQL) to create the tables for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`. Ensure all tables have `id` (UUID), `tenant_id` (UUID), `created_at`, `updated_at`, and enforce Row-Level Security on `tenant_id`.
  3. Define the corresponding Rust structs (using `sqlx` or the repo's ORM pattern) for these entities.
  4. Write unit tests to verify CRUD operations and strict tenant isolation (a query for Tenant A should never return Tenant B's data).
  5. The acceptance criteria is that the database schema is complete, RLS is active, Rust models compile, and tests prove the data models work correctly.

  **Priority**: P0 (critical core capability replacement)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
