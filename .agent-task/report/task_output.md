issue_title: "Native Rust Omnichannel Chat System Architecture (Legacy System Replacement)"
issue_description: |
  ## 1. Problem Statement
  OneHumanCorp (OHC) is transitioning away from the legacy third-party omnichannel dependency to build a high-performance, natively integrated Rust chat system. Currently, relying on an external service creates data silos, increases latency for our AI agent orchestration, breaks strict multi-tenant isolation guarantees, and complicates mobile-first offline reliability. To empower owners like Maya (baker managing Instagram DMs) and Nora (agency principal coordinating client messages), we need an internal, native Rust omnichannel inbox that perfectly aligns with OHC’s single-platform Zero Trust architecture and AI-first workflows.

  ## 2. Research Report
  **Legacy Architecture Benchmarking:**
  Based on an audit of the legacy open-source repository (the 'c-h-a-t-w-o-o-t' project), their system relies on the following core paradigms which we must replicate and enhance in Rust:
  *   **Core Entities:** Accounts (Tenants), Users, Inboxes, Channels (Web Widget, API, Email, SMS, WhatsApp, Instagram), Contacts, Conversations, and Messages.
  *   **Routing & Assignment:** Round-robin or manual assignment of conversations to agents (which in OHC includes AI agents).
  *   **Real-time Communication:** WebSocket action cables pushing events (e.g., `message.created`, `conversation.updated`) to clients.
  *   **Webhooks & Extensibility:** Emitting events for external integrations (or internal AI orchestration).

  **OHC Enhancements over the Legacy System:**
  *   **Native AI Integration:** Instead of standard webhooks, AI agents will directly subscribe to the internal event bus, allowing instant draft generation and context retrieval without network hops.
  *   **Strict Multi-Tenancy:** Using PostgreSQL Row-Level Security (RLS) bound to `tenant_id` at the database level, avoiding the application-level leaks common in monolithic applications.
  *   **Mobile-First Offline:** Syncing via a robust offline-capable local database (e.g., SQLite/Drift on Flutter) using a revision-based sync protocol rather than pure REST polling.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ AGENT : employs
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      AGENT ||--o{ CONVERSATION : assigned_to

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string provider_type
          jsonb credentials
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          uuid sender_id FK
          string sender_type
          text content
      }
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  1.  **Unified Inbox View:** A single scrollable list of active conversations. Each card shows the contact avatar, channel icon (e.g., Instagram, SMS), snippet of the last message, and a status indicator (Unread, AI Draft Ready, Waiting on Owner).
  2.  **Conversation Thread:** Standard chat UI. Top app bar contains contact name and channel. The message area supports text, images, and quick-reply AI suggestions at the bottom.
  3.  **AI Assistant Panel:** A slide-up bottom sheet that provides context on the customer (past orders, lifetime value) and allows the owner to approve or edit AI-generated reply drafts.

  ### Mobile UX Flow
  *   **Push Notification:** Owner receives a native push: "Maya, 3 new Instagram DMs regarding custom cakes."
  *   **Tap to Open:** Directly opens the Unified Inbox.
  *   **Review Draft:** Owner taps a conversation. The AI has already drafted a response based on the customer's query ("vegan cakes availability").
  *   **Approve/Send:** Owner taps "Approve & Send" or edits the text. The app immediately updates the UI optimistically and syncs in the background, resilient to slow cellular connections.

  ### AI Agent Integration Points
  *   **Operations & CS Agents:** Hooked into the Rust internal Event Bus (via Redis Pub/Sub or similar queue). When a `MessageCreated` event fires, the CS agent generates a draft reply and persists it with `status: draft`.
  *   **Memory & Context:** AI agents query the unified `CONTACT` and `CONVERSATION` tables to inject history (e.g., "This customer ordered a cake last month") into the LLM prompt.

  ### Key Design Decisions
  *   **PostgreSQL RLS:** Every table will enforce `tenant_id` at the database level.
  *   **Rust Axum + Tokio:** High-performance async WebSocket handling for real-time messaging.
  *   **Revision-Based Sync:** To support offline mobile, endpoints will return a `sync_token` to allow the Flutter client to fetch only delta updates for conversations and messages.

  ## 4. Implementation Prompt
  **Target:** Implementer Agent (Rust Backend & Flutter Frontend)
  **Objective:** Build the foundational Native Rust Omnichannel Chat backend and the corresponding 375px mobile-first unified inbox UI.
  **CUJ (Critical User Journey):**
  1. As a non-technical owner (e.g., Maya), I want to open the OHC app and see a single list of all incoming customer messages.
  2. I want to tap a message, see a chat interface, and read an AI-generated draft reply.
  3. I want to approve the draft and see the message appear as sent instantly, without waiting for a loading spinner.

  **Acceptance Criteria:**
  *   **Backend:** Define the Rust structs and PostgreSQL schemas (with RLS) for Tenant, Inbox, Conversation, Contact, and Message.
  *   **API:** Implement a REST endpoint for fetching conversations and a WebSocket endpoint for real-time events.
  *   **Frontend:** Create a Flutter screen matching the 375px mobile-first constraints. Implement a robust unified inbox list and a conversation thread view with optimistic UI updates.
  *   **Tests:** 100% unit test coverage for the service logic. Playwright/E2E testing for the UI demonstrating the CUJ flow using real local API endpoints (no UI mocks).

  ## 5. Scope & Priority
  **Priority:** P0 (Critical path to unblock core workflows)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
