issue_title: "[Research] Architect Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its external dependency on Chatwoot with a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. We need a robust architecture design for this native chat system that provides 100% feature parity with Chatwoot, including its omnichannel data models, controllers, channels, WebSocket real-time messaging, and inbox architecture, all tailored for a multi-tenant environment serving business owners.

  ## Research Report
  Based on an audit of the Chatwoot source code repository and the OHC product requirements, a native Rust implementation must replicate and optimize the following core concepts:

  *   **Omnichannel Architecture:** The system must support various channels (e.g., Email, SMS, Web Widget, API, Meta/Facebook/Instagram, WhatsApp, Telegram, Line) seamlessly.
  *   **Core Entities:**
      *   `Account` (Tenant)
      *   `Inbox` (Channel configuration for an account)
      *   `Contact` (End-users interacting across channels)
      *   `Conversation` (A thread of messages between a Contact and Agents in an Inbox)
      *   `Message` (Individual items in a conversation)
      *   `ChannelAdapter` (The specific implementation for a given channel type)
  *   **Real-time Communication:** WebSocket infrastructure to push real-time updates (new messages, presence, typing indicators) to the frontend.
  *   **Multi-tenancy:** Strict data isolation per account (tenant) using Row Level Security (RLS) in PostgreSQL.
  *   **Agent Automation:** Support for Macros, Canned Responses, and SLA policies.
  *   **Background Jobs:** High-performance queues for webhook delivery, email sending, and long-running integrations.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : owns
      ACCOUNT ||--o{ CONTACT : manages
      ACCOUNT ||--o{ CONVERSATION : tracks
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--|| CHANNEL_ADAPTER : configured_with
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      ACCOUNT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid account_id FK
          string name
          string channel_type
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid account_id FK
          string name
          string email
          string phone_number
          string identifier
      }
      CONVERSATION {
          uuid id PK
          uuid account_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid account_id FK
          uuid conversation_id FK
          uuid sender_id FK
          string sender_type
          string content
          string message_type
      }
  ```

  ### Mobile UX Flow (375px first)
  1.  **Work Command Center:** The owner sees a consolidated "Work Triage" feed on their phone.
  2.  **Unified Inbox:** Tapping a notification opens the unified inbox, combining Instagram DMs, SMS, and Web Widget messages into a single list, prioritized by SLA and urgency.
  3.  **Conversation View:** Tapping a thread opens a clean chat interface. The owner sees the message history and customer context (past orders, notes) seamlessly.
  4.  **AI Assistant Drafting:** The AI agent pre-drafts replies or suggests next actions (e.g., "Send Deposit Link") directly in the conversation view.
  5.  **Seamless Action:** The owner reviews, edits, and hits send. The backend routes the message back through the correct channel adapter.

  ### AI Agent Integration Points
  *   **Work Triage:** AI agents monitor incoming messages across all inboxes to categorize, prioritize, and suggest routing.
  *   **Customer Assistant:** AI drafts contextual replies based on tenant-scoped memory and the specific customer's history.
  *   **Operations Assistant:** AI can detect intent (e.g., "book a repair") and suggest actionable widgets (e.g., a scheduling card) within the chat stream.

  ### Key Design Decisions
  1.  **Rust Backend (SeaORM + PostgreSQL):** Utilize SeaORM for robust database interactions with strict enforcement of `tenant_id` on all queries for RLS.
  2.  **WebSocket (Tokio/Tungstenite):** Implement a scalable WebSocket server for real-time presence and message delivery to the Flutter PWA/App.
  3.  **NATS for Pub/Sub & Jobs:** Use NATS for high-throughput, low-latency internal event routing (e.g., distributing incoming webhooks to worker nodes) and background task queuing.
  4.  **Modular Channel Adapters:** Design a Trait-based adapter pattern in Rust so new channels (e.g., a new social network) can be added by implementing a standard interface without modifying core conversation logic.

  ## Implementation Prompt
  Implement the core database schema and Rust backend domain models for the new Native Omnichannel Chat System in OHC, replacing Chatwoot.
  1.  Create SeaORM entities for `Inbox`, `Contact`, `Conversation`, and `Message`, ensuring strict multi-tenant isolation (`account_id` / `tenant_id`).
  2.  Implement a generic `ChannelAdapter` trait and at least two concrete implementations (e.g., `WebWidget` and `Email`).
  3.  Create the gRPC/REST API endpoints necessary to list inboxes, fetch conversations for an inbox, and send a message.
  4.  The system must support the "Work Triage" use case: a single API call should return a unified feed of active conversations across all inboxes for an account.
  5.  Ensure all database operations are covered by unit tests verifying tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
