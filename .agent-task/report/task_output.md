issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) is replacing its reliance on external third-party services like Chatwoot for omnichannel customer support and messaging. The business needs a native, high-performance, multi-tenant Rust-based chat and inbox system integrated directly into the `onehumancorp/mono` platform. The previous external dependency created latency, data silo issues, and friction for business owners like Carlos (handyman) and Maya (baker) who need instant, context-aware AI agent coordination on incoming DMs.

  ## Research Report
  ### Key Findings from Chatwoot Source Code Benchmarking
  - **Data Models:** Chatwoot uses `Account`, `User`, `Inbox`, `Conversation`, `Message`, and various channel-specific models (e.g., `Channel::WebWidget`, `Channel::Email`). OHC needs to replicate this core conversational hierarchy.
  - **Controllers & APIs:** Exposes robust REST endpoints for agent interfaces and a public API for web widgets. OHC will use gRPC internally and REST+JSON externally for these.
  - **Real-time:** Depends heavily on WebSockets (ActionCable) for real-time message delivery and typing indicators. OHC will need a high-performance Rust asynchronous WebSocket implementation (e.g., Tokio + Tungstenite or Axum WebSockets).
  - **Extensibility:** Uses webhooks and a plugin architecture for integrations. OHC's architecture should natively support AI Agent injection points as first-class citizens instead of simple webhooks.

  ### Competitive Analysis
  Leading platforms (Intercom, Zendesk, Shopify Inbox) tightly integrate commerce context with support. By building natively, OHC can instantly provide the `Customer Assistant` AI with live cart, order, and calendar booking context without API latency.

  ## Design Doc
  ### High-Level Architecture
  The new system will live under `src/server/ohc/chat/` and use a layered architecture.

  **Components:**
  1.  **Channel Adapters:** Rust traits implementing integrations for Web (Widget), Email, SMS, Instagram, WhatsApp.
  2.  **Conversation Engine:** Core logic managing `Conversation` state (Open, Resolved, Snoozed), assignments, and SLA tracking.
  3.  **Real-time Gateway:** Axum-based WebSocket server handling live bidirectional communication for both the Owner app (Flutter) and Web Widgets.
  4.  **AI Orchestrator Link:** Deep integration with `Customer Assistant` to automatically draft replies or fully handle triage based on tenant settings.

  ### Data Model & Invariants
  All tables MUST include `tenant_id` and utilize PostgreSQL Row Level Security (RLS) to guarantee strict isolation. The cache layer (Redis/Valkey) must namespace keys using the `ohc:chat:{tenant_id}:...` pattern.

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : has
      Inbox ||--o{ Channel : has

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
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
          string content
          string message_type
      }
      Channel {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type
      }
  ```

  ### Sequence Diagram

  ```mermaid
  sequenceDiagram
      participant OwnerApp as Flutter Owner App
      participant WebWidget as Customer Web Widget
      participant Gateway as Real-time Gateway (Axum)
      participant Engine as Conversation Engine (Rust)
      participant AI as Customer Assistant (AI Orchestrator)
      participant DB as PostgreSQL (RLS)

      WebWidget->>Gateway: Connect WebSocket
      Gateway-->>WebWidget: Ack
      WebWidget->>Gateway: Send Message ("Do you have vegan cakes?")
      Gateway->>Engine: ProcessMessage
      Engine->>DB: Save Message
      Engine->>AI: Emit Event (OnNewMessage)
      AI->>Engine: Draft Reply (status: pending_approval)
      Engine->>DB: Save Draft
      Engine->>Gateway: Broadcast Draft
      Gateway->>OwnerApp: Show Draft
      OwnerApp->>Gateway: Approve Draft
      Gateway->>Engine: Send Approved Message
      Engine->>DB: Update Message Status
      Engine->>Gateway: Broadcast Message
      Gateway->>WebWidget: Receive Reply
  ```

  ### Mobile UX Flow (375px First)
  -   **Triage View:** The owner sees a unified "Inbox" tab. Conversations are displayed in a clean, high-density list (Ubiquiti style).
  -   **Conversation View:** Minimalist chat bubbles. A translucent floating action bar allows the owner to quickly select "Approve AI Draft", "Type Manually", or "Quick Action" (e.g., Send Quote, Request Deposit).
  -   **Performance Target:** The initial inbox load must happen in under 300ms, heavily utilizing local SQLite caching on the Flutter client.

  ### AI Agent Integration Points
  -   **OnNewMessage:** Event emitted to the AI Job Queue. The Customer Assistant parses intent, fetches CRM context, and either drafts a reply or executes an action (if trusted).
  -   **Draft Approval:** If the AI drafts a reply, it is stored as a special `Message` type (`status: pending_approval`). The owner UI highlights this for one-tap approval.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core database schema and Rust service layer for the native OHC Omnichannel Chat System based on the architecture doc.

  **Requirements:**
  1.  Create PostgreSQL migration for the core tables: `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Ensure `tenant_id` and RLS policies are applied to all tables.
  2.  Implement the Rust data models and Repository layer in `src/server/ohc/chat/domain/`. Use `sqlx` for database interactions.
  3.  Build a gRPC service (`src/proto/ohc/chat.proto` and `src/server/ohc/chat/service.rs`) exposing operations for creating inboxes, starting conversations, and sending messages.
  4.  Provide 100% unit test coverage for the repository and service layers.
  5.  Do NOT implement the WebSocket or external API layer in this initial PR. Focus on core structural integrity and data persistence.

  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
