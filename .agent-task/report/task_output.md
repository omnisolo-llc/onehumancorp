issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently lacks a native omnichannel inbox for its users to unify communications (Instagram DMs, WhatsApp, SMS, Web Chat, Email). Chatwoot, an external third-party service previously considered, is fully retired. We need a high-performance, multi-tenant, zero-trust chat and support engine natively built in Rust inside `onehumancorp/mono` to replace Chatwoot's functionality with tighter integration into the OHC agent ecosystem.

  ## Research Report
  Based on an audit of the `chatwoot/chatwoot` source code, key architectural components required for parity include:
  - **Data Models:** Accounts (Tenants), Inboxes, Channels, Conversations, Messages, Contacts, Users, and Agents.
  - **Channel Adapters:** Interfaces for Web Widget, API, Email, Facebook, Twitter, WhatsApp, SMS.
  - **Real-time Engine:** WebSocket server for real-time message delivery and typing indicators.
  - **Automation & Routing:** Macros, canned responses, auto-assignment, and SLA policies.
  - **Multi-tenancy:** Row-level security for all records to ensure strict data isolation.

  ### Competitive Analysis
  - **Chatwoot/Zendesk:** Heavy, non-native to our ecosystem.
  - **OHC's Approach:** A Rust-based microservice within our Bazel monorepo, natively communicating with our internal AI agents via gRPC/Redis queues to power the Customer & Relationship Assistant.

  ## Design Doc
  ### Architecture
  - **Service:** `src/services/omnichannel` (Rust).
  - **API:** gRPC for internal OHC components, REST/GraphQL for external clients and mobile apps.
  - **Database:** PostgreSQL with Row Level Security (`tenant_id`).
  - **Real-time:** WebSocket connections managed via Redis Pub/Sub for scale.
  - **Integration:** The `Work Triage` agent consumes a stream of incoming messages and drafts replies, stored as pending drafts in the omnichannel database.

  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CONVERSATION : contains
    CONTACT ||--o{ CONVERSATION : initiates
    CONVERSATION ||--o{ MESSAGE : contains
    CHANNEL ||--o{ INBOX : linked

    TENANT {
      uuid id PK
      string name
    }
    INBOX {
      uuid id PK
      uuid tenant_id FK
      string name
    }
    CONTACT {
      uuid id PK
      uuid tenant_id FK
      string email
      string phone
    }
    CONVERSATION {
      uuid id PK
      uuid inbox_id FK
      uuid contact_id FK
      string status
    }
    MESSAGE {
      uuid id PK
      uuid conversation_id FK
      string content
      string status
    }
    CHANNEL {
      uuid id PK
      string provider_type
    }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox:** A single scrollable list of conversations, distinctively badged by channel (IG, WA, Email).
  - **Conversation View:** Standard chat interface with quick-action buttons for AI-drafted replies ("Approve & Send", "Edit").
  - **Touch Targets:** 44x44px minimum for all actions (send, attach, close).

  ### AI Agent Integration
  - **Customer Assistant:** Listens to `message.created` events on the Redis event bus.
  - **Drafting:** The AI generates a `Message` record with status `draft` linked to the conversation, which the owner can review in the UI.

  ## Implementation Prompt
  **Goal:** Implement the foundational Rust data models, gRPC API, and basic WebSocket real-time engine for the new Omnichannel Chat service.
  **Tasks:**
  1. Create Rust structs and Diesel/SQLx migrations for: `Tenant`, `Inbox`, `Channel`, `Contact`, `Conversation`, `Message`.
  2. Implement a gRPC service for creating and retrieving Inboxes, Conversations, and Messages.
  3. Implement a basic WebSocket server (using `tokio-tungstenite` or `axum`) that broadcasts new messages to connected clients via a Redis channel.
  4. Write unit and integration tests for all components.
  **Acceptance Criteria:**
  - A client can connect via WebSocket.
  - A message created via gRPC is broadcasted to the connected WebSocket client.
  - Multi-tenant data isolation is enforced at the database level.
  - 100% test coverage.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
