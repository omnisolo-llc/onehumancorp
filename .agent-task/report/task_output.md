issue_title: "Implement Custom Rust Omnichannel Chat System & Retire Chatwoot"
issue_description: |
  ## Task Overview
  OHC is permanently retiring the external Chatwoot dependency in favor of a native, high-performance omnichannel chat system built entirely in Rust. This system must reside in `onehumancorp/mono` and replicate the core capabilities of Chatwoot while adhering to strict multi-tenant isolation rules, ensuring offline-tolerant capabilities, and operating seamlessly within the OHC ecosystem.

  ## Problem Statement
  Currently, OHC relies on Chatwoot for customer support and omnichannel messaging. This external dependency introduces network latency, complicates multi-tenant data isolation, increases operational overhead, and makes deep integration with native AI agents difficult. We need a native Rust solution to unify messaging across all channels (WhatsApp, Instagram, Email, Web Widget) into a single, high-performance, edge-cacheable system.

  ## Research Findings & Chatwoot Source Code Audit
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals several key architectural patterns to replicate natively in Rust:
  *   **Omnichannel Data Models:** Core entities include `Account` (Tenant), `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact`.
  *   **Webhooks & APIs:** A robust webhook system to stream events to clients and external systems, coupled with REST APIs for programmatic control.
  *   **Real-time Messaging:** WebSockets are used for real-time delivery of messages and typing indicators to the web widget and agent dashboards.
  *   **Channel Adapters:** Modular adapters for different networks (e.g., Twilio for WhatsApp/SMS, Meta Graph API for Instagram/Messenger, IMAP/SMTP for Email).
  *   **Automation:** Macros, canned responses, and assignment rules.

  *Competitive Analysis:* Systems like Shopify Sidekick and Wix Inbox deeply integrate messaging with commerce data. Our native system will similarly bridge the gap between conversations and actionable business objects (quotes, orders, bookings) directly in the UI.

  ## Architectural Design

  ### Data Model & Invariants
  Strict row-level security (RLS) and multi-tenant isolation via `tenant_id` must be enforced across all tables.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      TENANT ||--o{ CONTACT : manages

      TENANT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid inbox_id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL_ADAPTER {
          uuid adapter_id PK
          uuid inbox_id FK
          string type "WhatsApp, IG, Web"
          json credentials
      }
      CONVERSATION {
          uuid conversation_id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved"
      }
      MESSAGE {
          uuid message_id PK
          uuid conversation_id FK
          string content
          string sender_type "agent, customer, ai"
          timestamp created_at
      }
      CONTACT {
          uuid contact_id PK
          uuid tenant_id FK
          string name
          string identifier "phone, email, etc."
      }
  ```

  ### System Components
  *   **Native Rust Microservices:** Implementation of the data models using `sqlx` against PostgreSQL.
  *   **WebSocket Gateway:** A high-performance real-time messaging gateway built with Rust (e.g., `axum` + `tokio-tungstenite`) to handle live chat widget connections and agent dashboard updates.
  *   **AI Agent Integration:** The `Customer & Relationship Assistant` will listen to the internal `MESSAGE_CREATED` event stream via the AI Job Queue (PostgreSQL `SKIP LOCKED`) to draft responses, tag conversations, and propose actions (e.g., "Draft a quote for this cake inquiry").
  *   **Mobile-First Web Widget:** A Flutter/PWA chat widget designed for a 375px viewport, utilizing translucent glass styling, offline-tolerant local caching, and WebSockets for real-time delivery.

  ## AI Department Coordination
  *   **Work Triage Agent:** Ingests new `Conversation` and `Message` events, grouping them into the owner's actionable feed.
  *   **Customer Assistant Agent:** Reads historical `Message` context and drafts context-aware replies for owner approval.
  *   **Operations/Sales Agents:** Triggered by specific intents in messages (e.g., "I want to book a session") to draft quotes or schedule tasks, attached as structured metadata to the conversation.

  ## Mobile-First & UX Flow (375px First)
  *   **Unified Inbox Screen:** A clean, Unifi-style list of active conversations. Each row shows the contact name, channel icon (WhatsApp, IG), snippet, and an AI-generated summary/suggested action pill.
  *   **Conversation View:** A familiar chat interface. AI drafted responses appear in a translucent glass container just above the input bar, requiring a single tap to approve and send.
  *   **Offline Mode:** Conversations must be readable offline. Outbound messages sent while offline are queued locally and sync automatically upon reconnection.

  ## Implementation Prompt (For Implementer Agents)
  **Objective:** Implement the backend foundation for the Native Rust Omnichannel Chat System to replace Chatwoot.

  **Steps:**
  1.  Design and implement the PostgreSQL database schemas for `Inbox`, `Conversation`, `Message`, and `ChannelAdapter`. Ensure strict multi-tenant isolation using RLS (`tenant_id`).
  2.  Create the Rust domain models, repository interfaces, and `sqlx` implementations for these entities in `src/server/ohc/domain/chat` (or similar appropriate module).
  3.  Implement a set of REST or gRPC APIs for creating inboxes, listing conversations, and sending/receiving messages.
  4.  Establish a basic WebSocket endpoint in Axum that authenticates via SPIFFE/SPIRE (or current auth mechanism) and allows real-time message broadcasting to connected clients for a specific `conversation_id`.
  5.  Ensure all new Rust code achieves 100% unit test coverage.
  6.  Write a Playwright E2E test that simulates a user creating an inbox, starting a conversation, and sending a message through the API/UI layers.

  **Acceptance Criteria:**
  *   Chatwoot dependencies or API calls are completely bypassed or removed for these core functions.
  *   Database schema enforces tenant isolation.
  *   A message can be successfully sent and retrieved via the native API.
  *   100% unit test coverage on new backend code.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
