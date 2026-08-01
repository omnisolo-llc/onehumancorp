issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  The legacy integration has been fully removed from the OHC ecosystem to reduce operational overhead, external dependencies, and complexity. However, we now lack a unified omnichannel inbox for our owners. Our personas (like Maya who gets Instagram DMs, Carlos who gets service requests, and Fatima who receives WhatsApp pre-orders) need to be able to see all their communications in a single, lightning-fast unified inbox powered by AI.

  We need to replicate the core value of the old system (omnichannel unified inbox, webhook ingestion, channel adapters) but implemented natively in Rust within `onehumancorp/mono`. This system must enforce strict multi-tenant Row Level Security, integrate seamlessly with the AI Agent Triage, and run natively within our existing infrastructure.

  ## Research Report
  - We analyzed the source code repository of the old system to understand its data models, including `accounts` (tenants), `inboxes`, `conversations`, `messages`, `contacts`, and channel adapters (e.g., `channel_web_widgets`, `channel_whatsapp`).
  - By replicating these core models and their relationships natively in Rust with PostgreSQL, we can maintain feature parity while ensuring zero-trust multi-tenant isolation and integrating directly into our existing Bazel build pipeline.
  - OHC's backend will handle webhook ingestion from platforms like Meta (WhatsApp, Instagram) directly.
  - We will implement a Web Widget channel that uses WebSockets for real-time website chat.
  - All messages will flow through an outbox pattern for reliable delivery and AI triage processing.

  ## Design Doc
  ### Architecture Diagram

  ```mermaid
  erDiagram
      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string channel_type
          string name
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone_number
          string email
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id
          string content
          string message_type
      }

      Tenant ||--o{ Inbox : "has"
      Tenant ||--o{ Contact : "has"
      Tenant ||--o{ Conversation : "has"
      Tenant ||--o{ Message : "has"
      Inbox ||--o{ Conversation : "handles"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"
  ```

  ### Architecture
  1.  **Ingestion Layer:** Rust `axum` routes for incoming Meta Webhooks (WhatsApp/Instagram) and a WebSocket handler for the Web Widget.
  2.  **Service Layer:** Channel adapters that parse incoming provider payloads into a canonical `Message` format.
  3.  **Data Models (PostgreSQL + RLS):**
      -   `tenant_id` on every table.
      -   `inboxes`: Represents a channel endpoint (e.g., a specific WhatsApp number or a website widget).
      -   `contacts`: End users communicating with the business.
      -   `conversations`: A thread between a contact and an inbox.
      -   `messages`: Individual messages within a conversation.
  4.  **AI Triage:** Newly inserted messages trigger asynchronous processing by the `OperationsAssistant` or `CustomerAssistant` via the job queue to draft replies or take action.

  ### Mobile UX Flow
  -   The OHC Flutter app will feature an "Inbox" tab.
  -   The UI will use a unified thread view, displaying messages from all channels with clear badging indicating the source (WhatsApp, Web, etc.).
  -   The interface must work beautifully on a 375px viewport, utilizing native keyboards and fast scrolling.

  ### Estimated Scope
  Large

  ## Implementation Prompt
  -   **Implement the Core Data Models:** Create the Rust structs, Diesel/SQLx schema, and PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` is present on all and RLS is enabled.
  -   **Implement the Webhook Ingestion API:** Create an Axum route `POST /api/v1/webhooks/meta` to receive and verify WhatsApp Cloud API payloads, mapping them to the canonical `Message` model and persisting them.
  -   **Implement the Web Widget WebSocket:** Create an Axum WebSocket route for the real-time website widget, allowing anonymous or identified visitors to send and receive messages.
  -   **Ensure 100% Test Coverage:** Write unit tests for all models and API endpoints, and create a Playwright E2E test simulating a user sending a message via the Web Widget and the owner seeing it in the inbox.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
