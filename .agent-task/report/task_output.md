issue_title: "Architecture Design: Native Rust Omnichannel Inbox & Chat System (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol: Omnichannel Inbox & Chat System

  ## 1. Problem Statement
  OHC requires a Tencent Workbuddy-like work assistant where operators (like Maya the baker and Carlos the handyman) can triage messages from multiple channels (WhatsApp, Instagram DMs, Web Chat) seamlessly without switching apps. Previously, this relied on Chatwoot as an external third-party dependency, which introduced latency, security surface area (data leaving the tenant boundary), and complex agent-sync protocols. OHC needs a native, high-performance, embedded omnichannel chat system built in Rust to replace Chatwoot completely.

  ## 2. Research Report
  - **Chatwoot Audit & Benchmarking:**
    - Chatwoot handles conversations via `Channels` (e.g., WhatsApp, Web Widget, API), grouping them into an `Inbox`.
    - Key entities in Chatwoot: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`.
    - Real-time events use ActionCable (WebSockets); OHC will need a Rust-native equivalent (e.g., using `tokio-tungstenite` and `axum` WebSockets) with Redis Pub/Sub for multi-node event broadcasting.
  - **Competitor Systems (Tencent Workbuddy / WeCom / Shopify Inbox):**
    - High-scale systems cache active conversations and use edge nodes to terminate WebSockets quickly.
    - AI integration happens transparently: AI agents intercept messages, draft replies, and append metadata directly to the conversation thread before the owner sees it.

  ## 3. Design Doc

  ### Architecture Overview
  - **Microservice Layer:** Native Rust (`src/server/omnichannel`) exposing gRPC/REST APIs and a WebSocket endpoint for real-time frontend updates.
  - **Data Isolation:** `tenant_id` row-level security in PostgreSQL for all chat entities.
  - **Real-Time Sync:** Redis Pub/Sub (`ohc:events:inbox:{tenant_id}`) to broadcast new messages to connected Flutter/PWA clients.

  ### Data Model (Mermaid ER Diagram)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : participates_in

      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string provider "e.g., whatsapp, instagram, web"
          json credentials
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status "open, resolved, snoozed"
          timestamp created_at
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type "contact, owner, agent"
          timestamp created_at
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone_number
      }
  ```

  ### Mobile UX Flow (375px First)
  - **View 1: The Unified Triage Feed (Home):** Unread messages appear as premium, Apple-style translucent glass cards with an "AI Draft Ready" tag if the Customer Service agent has proposed a reply.
  - **View 2: Conversation Thread:** A sleek chat interface. Large tap targets (44x44px minimum). Swiping right on a message approves the AI draft. Native mobile keyboards are fully supported.
  - **Offline/Flaky Network:** Uses local optimistic UI updates. Messages sent offline are queued and re-synced using idempotency keys when the connection is restored.

  ### AI Agent Integration Points
  - **Customer Service Agent:** Hooks into the incoming message stream via PostgreSQL `SKIP LOCKED` job queue. Automatically drafts replies for inquiries (e.g., "Do you do vegan cakes?") and saves them with `sender_type="agent_draft"`.
  - **Operations Agent:** Monitors conversations for intent (e.g., booking a service) and inserts structured UI cards (quotes, payment links) into the chat stream.

  ## 4. Implementation Prompt
  **To the Implementer:**
  Implement the native Rust omnichannel chat system replacing Chatwoot.
  1. Build the PostgreSQL schema with strict `tenant_id` RLS for `inboxes`, `channel_adapters`, `conversations`, `messages`, and `contacts`.
  2. Create the Rust service in `src/server/omnichannel` using Axum/Tokio.
  3. Implement the WebSocket event gateway for real-time message broadcasting to the Flutter PWA.
  4. Build the Flutter frontend Triage Feed and Conversation Thread using the OHC Premium Token library (translucent glass, clean spacing).
  5. **Acceptance Criteria:** A user (e.g., Maya) can receive a message from a simulated external channel, see it instantly in the Flutter UI (via WebSocket), view the AI-drafted reply, and send a message back. All actions must work cleanly on a 375px mobile viewport. Include complete E2E Playwright tests covering this Critical User Journey. Do not use mocked data—everything must flow end-to-end.

  ## 5. Priority
  P0 (Critical Architecture Gap)

  ## 6. Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
