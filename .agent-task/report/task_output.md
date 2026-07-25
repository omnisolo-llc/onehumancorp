issue_title: "[Architecture] Native Rust Omnichannel Inbox & Real-time Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC aims to provide an owner-centric work assistant. Previously, customer support and messaging relied on Chatwoot, which has now been fully retired as an external service. Small business owners (like Maya the baker and Carlos the handyman) need a unified, real-time omnichannel inbox (Instagram DMs, WhatsApp, SMS, Web Chat) tightly integrated with OHC's multi-tenant architecture and AI Assistant agents. A native Rust implementation is required to ensure Zero-Trust isolation, seamless AI agent handoffs, and a mobile-first UI with offline-tolerant capabilities.

  ## Research Report & Chatwoot Benchmarking
  We conducted an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to baseline the data models, real-time messaging, and inbox architecture necessary for OHC's matching native Rust chat system.

  **Key Findings from Chatwoot Architecture:**
  - **Data Models:** Deep hierarchical structure linking `Accounts` (Tenants), `Inboxes`, `Channels` (Adapters for WhatsApp, Web, SMS), `Conversations`, `Messages`, and `Contacts`.
  - **Real-time Engine:** ActionCable (WebSocket) used for pushing real-time events (`message.created`, `conversation.updated`) to the frontend clients.
  - **Agent Routing & AI:** Round-robin or manual assignment, macros, canned responses, and SLAs.

  **Competitor Insights (Shopify Inbox, WeCom):**
  - **Unified Threading:** Merging customer identity across channels (e.g., matching a WhatsApp lead to an existing Web Chat customer).
  - **Agent-First Workflows:** The AI agent acts as the primary responder, with rules to escalate to human operators (e.g., for complex pricing queries or complaints).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      MESSAGE ||--o{ ATTACHMENT : includes
      TENANT ||--o{ CONTACT : manages
  ```

  **Core Components:**
  1. **Omnichannel Core (Rust):** High-performance Rust gRPC microservice handling CRUD for Inboxes, Conversations, and Messages. Uses PostgreSQL with strict Row-Level Security (RLS) bound to `tenant_id`.
  2. **Channel Adapters (Rust):** Extensible traits/interfaces connecting to WhatsApp Cloud API, Instagram Graph API, and native Web Chat.
  3. **Real-time WebSocket Gateway (Rust/Axum):** Axum-based WebSocket server subscribing to Redis Pub/Sub channels (`ohc:chat:events:{tenant_id}`) for instant event fan-out to active clients.
  4. **AI Agent Integration:** Integration with OHC AI Job Queue for the "Customer & Relationship Assistant" to automatically draft replies or execute triage upon `message.created` events.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen:** Apple/Ubiquiti-style clean list view. Avatars indicate channel source (small badge for WhatsApp/Insta). Unread dot and AI-draft indicator.
  - **Conversation Screen:** Native feeling chat interface. Sticky bottom input with "AI Draft" toggle. Quick actions (Quote, Book, Request Payment) integrated directly into the chat header.
  - **Offline Resilience:** Local SQLite cache for read-only viewing of recent chats while offline. Optimistic UI updates for sending messages.

  ## Implementation Prompt
  **Goal:** Implement the backend foundation and mobile-first UI for the Native Rust Omnichannel Chat System to replace Chatwoot.
  **CUJ:** Maya (Baker) receives a new Instagram DM. The AI assistant automatically drafts a reply. Maya opens the OHC app (Unified Inbox), taps the conversation, reviews the AI draft, and taps "Send".

  **Acceptance Criteria:**
  1. **Rust Backend:** Implement the database schema (PostgreSQL RLS) and Rust service layer for `Conversations`, `Messages`, and `Contacts`.
  2. **WebSocket Gateway:** Implement an Axum WebSocket endpoint for pushing `message.created` events to the frontend.
  3. **Tauri/Flutter UI:** Build the 375px mobile-first Unified Inbox and Conversation views using OHC Premium Tokens (Translucent Glass).
  4. **AI Integration:** Plumb a hook where a new incoming message triggers an AI background job to generate a draft reply.
  5. **Tests:** 100% unit test coverage for the Rust service and at least 3 Playwright E2E tests verifying the inbox list and chat send flow with ZERO mock data.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
