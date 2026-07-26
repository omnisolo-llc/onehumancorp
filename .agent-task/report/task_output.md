issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot as an external dependency. We need a native Rust omnichannel customer support and chat engine inside `onehumancorp/mono`. This new system must achieve 100% feature parity with Chatwoot, providing a high-performance, multi-tenant inbox architecture, real-time WebSocket messaging, and omnichannel integrations natively integrated into OHC. Small-business owners like Maya (baker) and Carlos (handyman) need an integrated inbox that works flawlessly on a 375px mobile screen to manage all customer communications without switching to an external tool.

  ## Research Report
  - **Chatwoot Source Code Audit**: Benchmarked against `https://github.com/chatwoot/chatwoot`. Chatwoot uses a Ruby on Rails backend with PostgreSQL, Redis, and WebSockets.
  - **Core Entities Identified**: `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`, `User` (Agent).
  - **Architecture Parity Requirement**: OHC must replicate the multi-tenant data model, channel adapters (Web Widget, Email, API), WebSocket real-time delivery, and the inbox UI natively in Rust and Flutter/web UI.
  - **Strategic Value**: Eliminating an external dependency reduces complexity, improves latency, ensures data sovereignty within OHC's Zero-Trust architecture, and provides a unified owner experience.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ CONVERSATION_PARTICIPANT : includes
      USER ||--o{ CONVERSATION_PARTICIPANT : acts_as
      MESSAGE }o--|| USER : sent_by_agent
      MESSAGE }o--|| CONTACT : sent_by_contact
  ```

  ### Key Design Decisions
  1.  **Multi-Tenancy**: All database tables (`inboxes`, `conversations`, `messages`, `contacts`) must include `tenant_id` and enforce row-level security (RLS).
  2.  **Native Rust Backend**: The core chat engine will reside in `src/server/services/chat` as a native Rust module leveraging our existing Axum/Tokio/gRPC infrastructure.
  3.  **Real-time Delivery**: WebSocket connections (using `tokio-tungstenite` or Axum WS) will stream new messages, conversation updates, and presence events to the client.
  4.  **Channel Adapters**: A modular trait-based adapter system in Rust to support different channels (Web Widget, Email, API).

  ### Mobile UX Flow (375px First)
  1.  **Inbox List (Command Center)**: A clean, unified list of active conversations. Unread indicators and clear priority marking.
  2.  **Conversation View**: A standard chat interface optimized for 375px. Bottom fixed input area, scrollable message history. Translucent glass effects on headers.
  3.  **Contact Context**: A drawer or slide-over pane revealing the customer's history, previous orders, and notes.

  ### AI Agent Integration Points
  - **Customer Assistant Agent**: Can draft replies, summarize long threads, and automatically tag conversations based on intent.
  - **Operations Agent**: Can automatically link a conversation to a specific booking or order based on context.

  ## Implementation Prompt
  Implement the core native Rust chat engine to replace Chatwoot.
  1.  **Backend (Rust)**:
      - Create the database schema (with `tenant_id` for RLS) for `inboxes`, `conversations`, `messages`, and `contacts`.
      - Implement the `ChatService` in `src/server/services/chat/mod.rs`.
      - Provide gRPC/REST APIs for listing inboxes, fetching conversations, sending messages, and updating contact details.
      - Implement a WebSocket endpoint for real-time message delivery.
  2.  **Frontend (UI)**:
      - Build the Unified Inbox UI using our Flutter/Web framework.
      - Ensure the layout is responsive, starting with a flawless 375px mobile experience.
      - Implement real-time updates via WebSockets.
  3.  **Verification**:
      - Add comprehensive unit tests in Rust for the core logic.
      - Add Playwright E2E tests verifying that a user can create an inbox, start a conversation, send a message, and see it appear in real-time.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
