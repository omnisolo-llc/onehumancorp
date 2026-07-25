issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  As mandated by the engineering standards, OneHumanCorp (OHC) must retire Chatwoot as an external third-party service and implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust. This system must achieve feature parity with Chatwoot, allowing owners like Maya and Carlos to manage all customer communications (Instagram DMs, WhatsApp, SMS, Web Chat) from a single, fast, unified interface on their mobile devices. The current reliance on an external system violates our Zero-Trust and native integration principles, preventing deep AI integration for automatic triage, drafting, and operation coordination.

  ## Research Report
  - **Source Audited**: https://github.com/chatwoot/chatwoot
  - **Key Chatwoot Architecture Components (to replicate)**:
    - **Data Models**: Account, Inbox, Channel (WebWidget, TwilioSms, Email, Whatsapp, API, Facebook, Telegram, Line), Conversation, Message, Contact, AgentBot.
    - **Real-time Messaging**: ActionCable (WebSockets) for real-time pub/sub of conversation events to the web widget and dashboard.
    - **Webhooks & APIs**: Core mechanisms for receiving messages from external channels (Twilio, Meta, etc.) and triggering automations.
    - **Multi-tenancy**: Uses `account_id` as the tenant boundary across all major models.

  - **OHC Competitive Advantage**: By building this natively in Rust within `onehumancorp/mono`, we can:
    - Leverage PostgreSQL Row-Level Security (RLS) and multi-tenant strictness via `tenant_id`.
    - Provide significantly lower latency via Rust's async ecosystem (Tokio/Tonic) compared to Ruby on Rails.
    - Tightly integrate with our existing AI Job Queue and Agent Departments (Customer & Relationship Assistant, Work Triage) without external API overhead.
    - Use Redlock for distributed concurrency when handling simultaneous channel webhooks and agent bot assignments.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : manages
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : owns

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assign
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string channel_type "WebWidget, TwilioSms, Email, WhatsApp, API"
          jsonb credentials
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, pending, snoozed"
          uuid assignee_id "Owner or Agent ID"
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type "incoming, outgoing, template, activity"
          boolean is_private_note
          timestamp created_at
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }
  ```

  ### Core System Components (Rust Implementation)
  1.  **Channel Adapters (Rust Crates/Modules)**:
      - Trait-based architecture for Channel Adapters (`trait ChannelAdapter { async fn receive_message(); async fn send_message(); }`).
      - Implementations for WebWidget, TwilioSms, WhatsApp, Email (IMAP/SMTP or Sendgrid), API.
  2.  **WebSocket Gateway (Rust + Tokio/Axum/Tonic)**:
      - High-performance WebSocket server for real-time bidirectional communication with the OHC Flutter frontend and embedded Web Widgets.
      - Redis Pub/Sub backend for horizontal scaling across Kubernetes pods.
  3.  **Omnichannel Inbox Controller**:
      - Central service handling incoming webhooks from external providers, normalizing them into the `Message` model, and routing them to the correct `Conversation` and `Inbox`.
  4.  **AI Department Integration Hook**:
      - Upon every new `Message` creation (where `message_type == incoming`), emit an event to the AI Job Queue (PostgreSQL `SKIP LOCKED`).
      - The `Customer & Relationship Assistant` agent consumes this queue to draft replies or auto-triage (tag, assign, resolve).

  ### Mobile UX Flow (375px First)
  - **Unified Feed**: The home screen aggregates actionable conversations alongside tasks.
  - **Conversation View**: Clean, translucent glass UI. Messages bubble up. Native keyboard integration.
  - **Agent Drafts**: AI-drafted responses appear inline in the composer with a distinct background color (e.g., subtle blue), requiring one tap to approve/send or tap to edit.
  - **Contact Context**: A drawer accessible via swipe-left reveals customer history, orders, and tags, crucial for Maya and Carlos.

  ## Implementation Prompt
  **Goal:** Implement the foundational database schema, Protobuf definitions, and Rust gRPC service layer for the Native Omnichannel Chat System.

  **Critical User Journey (CUJ):**
  As an owner (Maya), I want to connect my Twilio SMS number and Web Widget so that all customer messages appear in a single unified inbox, allowing my AI assistant to draft replies automatically.

  **Acceptance Criteria:**
  1.  Define the SQL schema (PostgreSQL) for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with strict `tenant_id` Row-Level Security (RLS) enforcement.
  2.  Create gRPC Protobuf definitions (`.proto`) for creating an Inbox, listing Conversations, and sending a Message.
  3.  Implement the core Rust service logic to handle these gRPC requests, ensuring all database interactions are scoped to the authenticated `tenant_id`.
  4.  Provide 100% unit test coverage for the Rust service layer and database queries.
  5.  Implement a Playwright E2E test that creates an inbox, simulates receiving a message, and verifies it appears in the OHC unified feed UI. The test MUST exercise the real backend stack (real API calls) without mocking internal network requests or database behavior.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
