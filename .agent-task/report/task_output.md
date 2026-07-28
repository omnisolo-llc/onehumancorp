issue_title: "[Native Chat] Core Rust Omnichannel Inbox Data Model"
issue_description: |
  # Native Chat System Architecture - Core Inbox

  ## Business Problem
  OHC relies on external Chatwoot as an omnichannel customer support tool. According to the strategic directive, Chatwoot dependency is fully RETIRED. We need to build a native omnichannel customer support system built natively in Rust that scales seamlessly and ensures complete multi-tenant data isolation. The primary step is replicating the core data models, specifically the Inbox, Conversation, Contact, and Message structures.

  ## Research Findings
  After cloning and analyzing the `chatwoot` source code (`app/models`), the key architectural invariants of an omnichannel system are:
  - **Account/Tenant Isolation**: Everything belongs to an `account` (tenant).
  - **Inboxes**: Represent a specific channel (e.g., WhatsApp, Email, Web Widget).
  - **Contacts**: The customer/visitor. Has a type (visitor/contact) and identifiers.
  - **Conversations**: Tie a `Contact` (customer) to an `Inbox` (channel) and track state (open, resolved).
  - **Messages**: Individual units of conversation. Can be incoming/outgoing, public/private notes, and reference the conversation and sender.

  ## System Design & Architecture
  ### Multi-Tenant Rust Data Model (PostgreSQL)
  All new Rust data models must strictly enforce `tenant_id` at the database schema level.

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ Conversation : "has"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type "e.g., 'web_widget', 'whatsapp'"
          boolean enable_auto_assignment
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          string contact_type
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          uuid assignee_id
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          text content
          string message_type "incoming, outgoing, template"
          boolean private
      }
  ```

  ### Implementation Prompt for Engineer Agents
  1. Create database migrations for `inboxes`, `contacts`, `conversations`, and `messages` inside the Rust `src/server/db` environment.
  2. Ensure every table has `tenant_id` and Row-Level Security (RLS) enabled.
  3. Implement the corresponding Rust struct definitions (SeaORM/SQLx or whatever ORM is used).
  4. Ensure complete 100% test coverage using the standard `bazel test //...` suite for these models.
  5. The API layer is NOT required for this exact PR. Focus purely on robust, tested multi-tenant native data structures.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, rust-chat]
assignees: []
