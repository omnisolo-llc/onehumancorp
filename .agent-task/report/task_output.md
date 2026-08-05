issue_title: "Native Rust Omnichannel Chat System: Implement Core Data Models, Channel Adapters, and Inbox Architecture"
issue_description: |
  **Universal Core Design & Research Protocols:**
  This issue strictly follows the Phase -2 to Phase 0 protocols. As a researcher, I have audited the codebase. The `bazel test //...` currently fails with network timeouts when fetching dependencies (`protoc-gen-validate`), which violates the Phase -2 protocol (Main Branch Test Repair First). The implementer MUST fix these bazel fetch/build issues before executing this architecture proposal.

  **Problem Statement:**
  OneHumanCorp (OHC) currently lacks a fully native, lightning-fast omnichannel chat system to replace external dependencies (e.g., Chatwoot). The business owners (Maya, Carlos, Priya, Leo, Fatima) need a unified inbox that consolidates customer communications from various channels (Instagram DMs, WhatsApp, Web Widget, etc.) without leaving the OHC platform. They need this system to be highly performant, handle real-time communications efficiently, and maintain strict multi-tenant isolation so their data is completely secure.

  **Research Report:**
  As mandated by the engineering standards, Chatwoot integration is 100% RETIRED. OHC must implement its own high-performance omnichannel chat system natively in Rust.

  An audit of typical omnichannel platforms (like Chatwoot, Twilio Flex, Zendesk) reveals that a robust system requires:
  1.  **Unified Inbox Model:** A centralized view of conversations across all channels.
  2.  **Channel Adapters:** Modular integrations for specific platforms (WhatsApp, Web Widget, Email, Meta/Instagram).
  3.  **Real-time Event Bus:** WebSockets for instantaneous updates.
  4.  **Multi-Tenancy:** Absolute data segregation per tenant (Row Level Security).
  5.  **Agent/Bot Handoff:** Seamless transition between AI agents and human operators.

  Currently, `src/server/integrations/chat/README.md` exists but lacks the full Rust backend implementation.

  **Design Doc:**

  *Architecture Overview:*
  The Native Rust Omnichannel Chat System will be a set of microservices/crates within the OHC mono-repo.

  *   **Data Models (PostgreSQL + RLS):**
      *   `Tenant`: The overarching account.
      *   `Inbox`: A logical grouping of conversations (e.g., "Customer Support", "Sales").
      *   `Channel`: The specific medium (e.g., "WhatsApp", "Web Widget").
      *   `Contact`: The external user communicating with the tenant.
      *   `Conversation`: A thread of messages between a Contact and a Tenant via a Channel.
      *   `Message`: The individual communication unit.
  *   **Channel Adapters (Rust Crates):**
      *   `channel_whatsapp`: Handles Meta webhook payloads and API interactions.
      *   `channel_web_widget`: Manages WebSocket connections for real-time website chat.
  *   **Real-time Engine (Rust + WebSockets/Redis):**
      *   Pub/Sub system (likely leveraging Redis as already used in OHC) to broadcast new messages to connected UI clients and AI Agent workers.

  *Architecture Diagram (Mermaid.js)*
  ```mermaid
  graph TD
      Client[Customer - Web/WhatsApp] -->|Webhook/WS| API[Rust Chat API]
      API --> Adapters[Channel Adapters]
      Adapters --> DB[(PostgreSQL)]
      Adapters --> PubSub[Redis Pub/Sub]
      PubSub --> UI[Owner Mobile/Web App]
      PubSub --> AIAgent[AI Triage Agent]
      UI -.->|Replies| API
      AIAgent -.->|Drafts/Replies| API
  ```

  *Mobile UX Flow (375px first):*
  1.  **Unified Feed:** The home screen (Triage) shows new messages aggregated from all channels, clearly marked with source icons (e.g., a WhatsApp icon).
  2.  **Conversation View:** Tapping a message opens a familiar chat interface.
  3.  **Context Panel:** A collapsible top/side panel (or an info icon on mobile) shows the Contact's history, previous orders, and AI-generated summary.
  4.  **Smart Replies:** Above the keyboard, AI-drafted responses are suggested for one-tap sending.

  *AI Agent Integration Points:*
  *   **Triage Agent:** Subscribes to the `message.created` event. Categorizes intent (e.g., "Support", "Sales", "Refund") and assigns priority.
  *   **Drafting Agent:** Generates suggested replies based on tenant context (e.g., checking inventory if the customer asks about cake availability).

  **Implementation Prompt:**
  As an Implementer Agent, your task is to build the core backend infrastructure for the Native Rust Omnichannel Chat System inside `onehumancorp/mono`.

  1.  **P0 REQUIREMENT:** Ensure the main branch builds happens successfully. Diagnose and fix any fetch/timeout issues (e.g., `protoc-gen-validate`) before writing feature code.
  2.  **Data Models:** Define the core Rust structs and database schemas (with strict RLS) for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  3.  **Channel Adapters:** Implement the foundational interfaces for `ChannelAdapter` and concrete implementations for a mock "Web Widget" (WebSocket) and "WhatsApp" (Webhook skeleton).
  4.  **API Layer:** Expose basic REST/gRPC endpoints to create inboxes, send messages, and fetch conversation history.
  5.  **Testing:** Write comprehensive unit tests for all models and adapters. Ensure 100% coverage for the new code.

  Do not worry about the frontend UI implementation in this PR; focus on building a robust, high-performance, and secure backend foundation.

  **Priority:** P0 (Critical Path for OHC)
  **Estimated Scope:** Large

issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: [agent-report]
assignees: []
