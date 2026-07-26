issue_title: "Design OHC Native Rust Omnichannel Chat: Data Model & Repository Layer"
issue_description: |
  # Problem Statement
  OHC relies on a unified, omnichannel support platform to aggregate customer inquiries from Instagram, WhatsApp, SMS, and email. Historically, we explored using Chatwoot for this. However, Chatwoot has been fully retired in favor of a native, tenant-safe Rust implementation as mandated by `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`.

  Currently, OHC lacks the foundational data structures and persistence layer in Rust to model the core primitives of an omnichannel chat system: Conversations, Messages, Contacts, and Channels. This absence blocks all subsequent work on webhooks, AI agent integration, real-time sync, and native UI surfaces. We must implement these entities securely, ensuring strict multi-tenant isolation via `tenant_id` at every level.

  # Research Report
  - **Chatwoot Source Code Audit:** A deep dive into the Chatwoot Ruby on Rails source code (`app/models/`) reveals the core schema requirements:
    - **Conversations:** Need to track `status` (open, closed, snoozed), `contact_id`, `inbox_id`, `assignee_id`, and last activity timestamps.
    - **Messages:** Require `content`, `message_type` (incoming, outgoing, template), `content_type` (text, image, audio), `sender_id`, `sender_type`, and delivery `status`.
    - **Multi-Tenancy:** Chatwoot uses `account_id` aggressively. OHC will map this directly to our `tenant_id` paradigm.
  - **OHC Architecture Mandate:** The native design requires a "Canonical conversation domain" with shared PostgreSQL/SQLite repository contracts, RLS (Row Level Security), and semantic deduplication.
  - **Competitor Insights (Shopify/Wix):** Their inbox solutions often struggle with proactive context. Our data model must support `additional_attributes` (JSONB) to link conversations back to orders, bookings, and full customer history, enabling the LLM (The Ambassador) to act proactively.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns

      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates

      CONVERSATION ||--o{ MESSAGE : contains

      CONTACT {
          uuid id PK
          string tenant_id FK
          string name
          string email
          string phone_number
          string avatar_url
          jsonb custom_attributes
          datetime created_at
          datetime updated_at
      }

      INBOX {
          uuid id PK
          string tenant_id FK
          string name
          string channel_type
          boolean is_active
          datetime created_at
      }

      CONVERSATION {
          uuid id PK
          string tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          uuid assignee_id
          datetime last_activity_at
          jsonb context_metadata
      }

      MESSAGE {
          uuid id PK
          string tenant_id FK
          uuid conversation_id FK
          string sender_type
          uuid sender_id
          string content
          string message_type
          string status
          jsonb attachments
          datetime created_at
      }
  ```

  ### Core Entities & Multi-Tenancy
  The system will introduce four core entities. Strict row-level tenant isolation is the highest priority. Every table and every query MUST include `tenant_id`.

  1.  **Contact:** Represents a customer or lead interacting with the business.
  2.  **Inbox:** Represents a channel (e.g., "WhatsApp Business", "Instagram DMs", "Web Widget").
  3.  **Conversation:** A thread of messages between a Contact and an Inbox.
  4.  **Message:** Individual chat bubbles within a Conversation.

  ### AI Agent Integration Points
  - The `Conversation.context_metadata` and `Contact.custom_attributes` JSONB fields are critical. They will store links to external systems (orders, bookings) that the "Customer Identity Resolution Engine" populate.
  - This structured data enables "The Ambassador" agent to perform Retrieval-Augmented Generation (RAG) and draft accurate replies.

  ### Mobile UX Flow & Performance
  - This foundational layer ensures that when the mobile app (375px viewport) requests the "Agent Feed," the backend can execute highly optimized, indexed queries filtering by `tenant_id` and `status = 'open'` to render the UI instantly.
  - The use of JSONB allows flexibility without requiring expensive schema migrations as we add new channel types.

  # Implementation Prompt
  **User-Facing Outcome:** As an engineer, I can reliably store and retrieve omnichannel conversations, messages, and contacts knowing they are strictly isolated by tenant and ready for the real-time websocket and AI layers.

  **CUJ & Acceptance Criteria:**
  1. Define Rust structs for `Contact`, `Inbox`, `Conversation`, and `Message` in the core domain layer. Ensure all derive serialization and include a `tenant_id`.
  2. Create the corresponding PostgreSQL schema migrations (DDL) utilizing UUID primary keys and `tenant_id` foreign keys. Ensure `ENABLE ROW LEVEL SECURITY` is applied to all new tables.
  3. Implement the Repository layer traits and implementations for PostgreSQL.
  4. **Strict Requirement:** Every single repository method (create, read, list, update) MUST require `tenant_id` as an argument and enforce it in the SQL query.
  5. Provide comprehensive unit tests using the standard repository testing patterns (or fakes) demonstrating successful CRUD operations and proving cross-tenant data leakage is impossible (e.g., attempt to fetch a conversation using Tenant A's ID that belongs to Tenant B and assert a Not Found error).
  6. Ensure all `bazel test //...` run green.

  **Priority**: P0 (critical)

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
