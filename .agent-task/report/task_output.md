issue_title: "Architect Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently lacks a native, high-performance solution for managing customer interactions across multiple channels (Instagram, WhatsApp, Email, Web Widget). Small-business owners like Maya (the baker) and Carlos (the handyman) need a unified inbox that brings all customer messages, DMs, and inquiries into a single view. They shouldn't have to juggle 5 different apps to reply to customers. Furthermore, OHC must retire Chatwoot as an external service to guarantee deep data integration, strict multi-tenant isolation, and real-time AI agent capabilities directly within the OHC platform.

  ## Research Report
  I cloned and audited the Chatwoot open-source repository to understand its architecture and map it to OHC's native Rust implementation. Key findings:
  - **Data Models:** Chatwoot uses an `Inbox` model to represent a channel (e.g., Email, Twitter, Facebook, Web Widget). Channels have their own specialized adapters.
  - **Conversations:** A `Conversation` bridges a `Contact` and an `Inbox`. It tracks state (`status: open/resolved/snoozed`), assignee, and last activity.
  - **Messages:** A `Message` belongs to a `Conversation` and has a `message_type` (incoming, outgoing, template) and `content_type`.
  - **Contacts:** A `Contact` stores cross-channel customer information (`email`, `phone_number`, `identifier`).

  **Competitor Analysis:** Platforms like Shopify Inbox, Zendesk, and Intercom all centralize multi-channel messaging into a unified thread. However, OHC's differentiation is that AI Agents (e.g., Customer & Relationship Assistant) will actively participate in these threads, draft replies, and execute operations, all orchestrated natively.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : handles
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          uuid sender_id
          text content
          string message_type
      }
  ```

  ### Mobile UX Flow (375px first)
  - **Inbox Tab:** A unified "Inbox" tab on the bottom navigation bar with a clear unread badge indicator.
  - **Conversation List:** A cleanly spaced vertical list of active conversations. Each row shows the customer name, channel icon (e.g., Instagram), snippet of the last message, and a timestamp. Translucent Glass styling applied.
  - **Conversation View:** A standard chat interface where messages are bubbled. At the bottom, a smart input bar where the AI assistant can suggest replies ("Tap to send AI draft: Yes, we do vegan cakes!"). Native mobile keyboards are fully supported.

  ### AI Agent Integration Points
  - The **Customer & Relationship Assistant** listens to new `Message` events via the PostgreSQL `SKIP LOCKED` job queue.
  - Agent drafts are saved as a special `message_type = 'draft'` and displayed to the owner for one-tap approval.

  ### Key Design Decisions
  - **Native Rust Implementation:** Build using Axum and Tokio for WebSockets to ensure low latency and high concurrency, completely replacing any Chatwoot dependencies.
  - **Strict Multi-Tenancy:** Enforce Row-Level Security (RLS) in PostgreSQL using `tenant_id` on every table (`inboxes`, `conversations`, `messages`, `contacts`) to ensure absolute data isolation.

  ## Implementation Prompt
  Implement the foundational Rust data models and database migrations for the native OHC omnichannel unified inbox.
  - Create PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`. All tables MUST include `tenant_id` and have Row Level Security (RLS) enabled.
  - Implement the corresponding Rust struct models and SQLx query methods for basic CRUD operations.
  - Build the initial Axum REST API endpoints for fetching the conversation list and message history for a specific inbox.
  - **Acceptance Criteria:** A front-end client can fetch a unified list of conversations for a tenant, and the AI agent can insert a draft message into a conversation. Zero Chatwoot dependencies. All endpoints must enforce tenant isolation and have 100% test coverage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
