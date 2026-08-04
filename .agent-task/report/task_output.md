issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**
  The current OHC architecture relies on the external Chatwoot service for omnichannel messaging and customer support functionalities. This external dependency introduces complexity, potential latency, security concerns, and breaks the core principle of a unified, native backend. We need to implement a native Rust omnichannel chat system within `onehumancorp/mono` to replace Chatwoot completely and achieve 100% feature parity. This system must handle multi-tenant isolation, real-time WebSocket communication, various channel adapters (e.g., WhatsApp, Meta, SMS), and seamless integration with our AI orchestration layer.

  **Research Report**
  1.  **Chatwoot Architecture Audit:** An analysis of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals a complex architecture centered around several key models:
      *   `Account` (Tenant)
      *   `Inbox` (Channel aggregation point)
      *   `Channel` (Specific integration like Twilio, Meta)
      *   `Conversation` (Thread of messages)
      *   `Message` (Individual piece of content)
      *   `Contact` (Customer profile)
  2.  **Current OHC State:** We have initial structs defined in `src/server/services/chat/models.rs` (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) and a basic service structure in `src/server/services/chat/service.rs`. However, this is incomplete. We lack the WebSocket infrastructure for real-time updates (similar to Chatwoot's ActionCable usage), comprehensive channel adapters, and the deep AI integration required by the OHC vision.
  3.  **Missing Capabilities:**
      *   **WebSocket/Real-time:** A robust mechanism to push message updates to the client (Tauri/PWA) instantly.
      *   **Channel Adapters:** While we have some integrations (e.g., `src/server/integrations/twilio_webhook.rs`, `meta_webhook.rs`), they need to be unified under the new chat architecture.
      *   **Omnichannel Inbox:** A centralized view aggregating messages from all channels for a given tenant.
      *   **Agent (AI/Human) Routing:** Logic to assign conversations to specific agents (human or AI) based on rules or SLA policies.

  **Design Doc**
  1.  **Architecture Diagram (Mental Model)**
      *   `External Webhooks (Twilio, Meta, etc.)` -> `Channel Adapters` -> `Unified Inbox Service` -> `Database (PostgreSQL)`
      *   `Database` -> `WebSocket Broadcast` -> `Client (Tauri/PWA)`
      *   `Unified Inbox Service` <-> `AI Orchestrator (Department Coordination)`
  2.  **Mobile UX Flow (375px First)**
      *   **Unified Inbox View:** A clean list of active conversations, clearly indicating the source channel (icon) and unread status.
      *   **Conversation View:** A familiar chat interface. Messages from the customer on the left, agent (AI or human) on the right.
      *   **AI Drafting:** An AI "thought bubble" indicating the AI is drafting a response, with options for the owner to "Send," "Edit," or "Discard."
  3.  **AI Agent Integration Points**
      *   **Message Ingestion:** When a new message arrives, the `Unified Inbox Service` notifies the `AI Orchestrator` (specifically the Operations or Customer Service department).
      *   **Draft Generation:** The AI agent analyzes the context and generates a draft response.
      *   **Approval Workflow:** The draft is presented to the owner in the UI. Upon approval, the `Unified Inbox Service` sends the message via the appropriate `Channel Adapter`.
  4.  **Key Design Decisions**
      *   **Rust Native:** Everything built in Rust for performance and tight integration.
      *   **PostgreSQL as Source of Truth:** All messages, conversations, and contacts are stored in Postgres with strict Row Level Security (RLS) for tenant isolation.
      *   **Axum WebSockets:** Utilize Axum's WebSocket capabilities for real-time bidirectional communication between the server and the Flutter/Tauri clients.

  **Implementation Prompt**
  **Goal:** Build the native Rust Omnichannel Chat System to replace Chatwoot.
  **CUJ:** Maya (the baker) receives an Instagram DM asking about vegan cakes. The message appears in her OHC Unified Inbox instantly. The AI Customer Service Assistant drafts a reply based on her knowledge base ("Yes, we offer several vegan options!"). Maya reviews the draft on her phone, taps "Send," and the message is delivered back via Instagram.

  **Acceptance Criteria:**
  1.  **Data Models & Migrations:** Complete the PostgreSQL schema for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with strict RLS (tenant isolation). Update existing migrations or create new ones as necessary.
  2.  **Service Layer:** Expand `src/server/services/chat/service.rs` to handle complex queries (e.g., fetching a paginated unified inbox).
  3.  **WebSocket Infrastructure:** Implement an Axum-based WebSocket server to broadcast real-time events (new message, conversation updated) to connected clients for a specific tenant.
  4.  **Channel Integration:** Unify existing webhooks (Twilio, Meta) to route incoming messages into the new `ChatService`.
  5.  **AI Orchestration:** Connect the `ChatService` to the `DepartmentOrchestrator` so that incoming messages trigger AI draft generation.
  6.  **E2E Tests:** Write comprehensive Playwright E2E tests simulating an incoming webhook, the message appearing in the UI, AI draft generation, and agent approval/sending.

  **Priority:** P0 (Critical - Blocks Chatwoot retirement)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
