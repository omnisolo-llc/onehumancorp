issue_title: "Architecture Design: Native Rust Omnichannel Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat Engine

  ## Problem Statement
  OneHumanCorp (OHC) has strictly retired Chatwoot as an external dependency. Our small business owners (Maya, Carlos, Priya, Leo, Fatima) need a unified, multi-tenant inbox to receive and respond to inquiries from Instagram DMs, WhatsApp, SMS, and web chat. Currently, OHC lacks a native chat system that provides these unified capabilities. Without this, owners have to context-switch between apps or rely on disconnected tools, breaking the "owner clarity" promise.

  ## Research Report
  ### Benchmarking Chatwoot
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals its core architecture:
  - **Data Models**: `Account` (tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `ContactInbox`.
  - **Channels**: Modular adapters (`Channel::Api`, `Channel::Email`, `Channel::FacebookPage`, `Channel::Whatsapp`, `Channel::WebWidget`, etc.).
  - **Real-time**: WebSockets pushing events to a Vue frontend.
  - **Automation**: AgentBots, Macros, and automated SLA rules.

  ### OHC Architecture Alignment
  OHC requires replacing this Ruby/Rails implementation with a high-performance **Native Rust Microservice/Crate** (in `onehumancorp/mono`) that integrates natively with our gRPC APIs, PostgreSQL (Row Level Security enabled via `tenant_id`), and Redis (for pub/sub WebSocket events).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONTACT_INBOX : linked_via
      INBOX ||--o{ CONTACT_INBOX : provides
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT_INBOX ||--o{ CONVERSATION : initiates

      TENANT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid inbox_id PK
          uuid tenant_id FK
          string channel_type "e.g., WHATSAPP, INSTAGRAM, WEB"
      }
      CONTACT {
          uuid contact_id PK
          uuid tenant_id FK
          string name
          string phone
      }
      CONTACT_INBOX {
          uuid contact_inbox_id PK
          uuid contact_id FK
          uuid inbox_id FK
          string source_id "e.g., WhatsApp phone number"
      }
      CONVERSATION {
          uuid conversation_id PK
          uuid inbox_id FK
          uuid contact_inbox_id FK
          string status "open, resolved, snoozed"
      }
      MESSAGE {
          uuid message_id PK
          uuid conversation_id FK
          string content
          string message_type "incoming, outgoing, template"
      }
  ```

  ### Key Design Decisions
  - **Tenant Isolation**: Every database operation MUST enforce `tenant_id` at the row level.
  - **Rust Ecosystem**: Use `axum` or `tonic` for gRPC/REST APIs, `sqlx` for database, and `tokio-tungstenite` or similar for WebSocket management.
  - **Redis Pub/Sub**: Message creation publishes to Redis (`ohc:chat:{tenant_id}:{conversation_id}`), triggering WebSocket broadcasts to active frontend clients.
  - **Swappable Channel Adapters**: Define a Rust trait `ChannelAdapter` to handle inbound webhooks and outbound API calls for WhatsApp, IG, SMS, etc.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Home)**: A clean, scrolling list of conversations. Unread badges are prominent. Translucent Glass top bar.
  - **Conversation Thread**:
    - Tap a conversation -> Slide in thread view.
    - Floating action button (FAB) or fixed bottom input bar for quick replies.
    - AI Draft suggestions float above the input bar.
  - **Contact Context**: A swipe-left drawer (or tap on avatar) reveals customer context (past orders, lifetime value, preferences).

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Hooks into the `Message` creation lifecycle. If `status` is open and no human reply in 5 mins, the AI agent queues a background job (using PostgreSQL `SKIP LOCKED`) to generate a draft reply based on `tenant_id` context and contact history.
  - **Work Triage**: Analyzes incoming messages to tag them (e.g., "quote request", "complaint") and escalate to the owner's Daily Prioritized Feed.

  ## Implementation Prompt
  **Target Implementer:** Backend & Frontend Engineers
  **Objective:** Build the foundational Native Rust Omnichannel Chat System inside `onehumancorp/mono` to replace Chatwoot.
  **Requirements:**
  1. Define the Protobuf schemas for Inbox, Conversation, Contact, and Message.
  2. Implement the Rust gRPC server and `sqlx` PostgreSQL models using strict `tenant_id` Row-Level Security.
  3. Create the WebSocket gateway for real-time message delivery.
  4. Build the Flutter frontend `ConversationList` and `ConversationThread` widgets. They must support 375px mobile layouts perfectly, implementing the Translucent Glass design tokens.
  5. Ensure zero mock data is used; the UI must bind directly to the real Rust gRPC API and WebSockets.
  6. Achieve 100% unit test coverage for the Rust crates and Flutter widgets, plus at least 3 Playwright E2E tests validating the end-to-end send/receive journey.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
