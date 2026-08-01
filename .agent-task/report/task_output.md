issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  OHC currently lacks a high-performance, multi-tenant omnichannel customer support & chat engine built natively. Relying on Chatwoot as an external third-party service, dependency, or integration has been 100% RETIRED to reduce complexity, eliminate third-party dependencies, and maintain strict multi-tenant isolation internally. We need to implement a matching native Rust chat system in `onehumancorp/mono` that achieves 100% feature parity with Chatwoot, tailored specifically for our non-technical owner/operator personas (Maya, Carlos, Priya, Leo, Fatima).

  # Research Report
  Based on an audit of the [Chatwoot source code](https://github.com/chatwoot/chatwoot) (specifically data models, channel adapters, web chat widget, WebSocket events, APIs, webhooks, SLA policies, macros, canned responses, and agent routing) and industry standards for modern customer support systems (Intercom, Zendesk):
  - **Data Models**: Chatwoot relies on `Account` (tenant), `User` (agents/admins), `Contact` (customers), `Inbox`, `Conversation`, `Message`, and `Channel::*` models. OHC needs a Rust equivalent mapping to our existing tenant structure (PostgreSQL with RLS).
  - **Omnichannel Support**: Support for channels like Web Widget, Email, API, WhatsApp, Instagram DMs, etc., via polymorphic channel adapters.
  - **Real-time Messaging**: Chatwoot uses ActionCable (WebSockets). OHC needs an asynchronous WebSocket server (using `tokio-tungstenite` and `axum`) integrated with Redis/NATS pub-sub for horizontal scaling and real-time event broadcasting to the Flutter PWA clients.
  - **Automation & Routing**: SLA policies, macros, and agent routing logic must be natively supported. Our AI Operations & CS Departments will seamlessly plug into this natively, allowing AI agents to auto-respond, triage, and route conversations.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ AGENT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o| CHANNEL_ADAPTER : configured_with
  ```
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_WebWidget
      participant Axum_WebSocket
      participant Redis_PubSub
      participant OHC_Core_App
      participant AI_CS_Agent

      Customer->>OHC_WebWidget: Sends message
      OHC_WebWidget->>Axum_WebSocket: WSS Push Message
      Axum_WebSocket->>OHC_Core_App: Process Message (Rust)
      OHC_Core_App->>Redis_PubSub: Publish Event (message.created)
      OHC_Core_App-->>AI_CS_Agent: Trigger AI Triage
      AI_CS_Agent->>OHC_Core_App: Generate Draft/Reply
      OHC_Core_App->>Axum_WebSocket: Broadcast updates to Owner/Agent
      Axum_WebSocket->>OHC_WebWidget: Send AI reply to Customer
  ```

  ## Mobile UX Flow (375px First)
  - **Unified Inbox View**: A unified feed showing messages across all channels (IG DMs, Web Chat, Email).
  - **Conversation View**: Clean, translucent glass UI showing chat history. Bottom sheet or sticky input area for typing replies, using native mobile keyboards.
  - **AI Assist**: A distinct, subtle "AI Sparkle" button in the composer to generate draft replies based on customer history and business context.
  - **Offline/Flaky Network Tolerance**: Optimistic UI updates. Messages show a translucent "sending" state and retry automatically in the background via local cache.

  ## AI Agent Integration Points
  - **AI Triage & Auto-Reply**: The Customer Service AI agent listens to `conversation.created` and `message.created` events via the internal event bus. It evaluates intent, consults business knowledge/inventory, and can immediately reply or draft a response for the owner.
  - **Smart Summarization**: If an owner opens a long thread, the AI provides a 1-sentence summary of the context (e.g., "Customer wants to change their cake pickup time to 4 PM").

  ## Key Design Decisions
  - **Native Rust Axum + WebSockets**: Using `axum` and `tokio-tungstenite` for high-throughput, low-latency concurrent connections.
  - **PostgreSQL Row-Level Security (RLS)**: Enforced tenant isolation across all chat-related tables (`inboxes`, `conversations`, `messages`, `contacts`).
  - **Event-Driven Pub/Sub**: Utilize Redis/NATS for decoupling the core chat logic from AI agent processing and webhook dispatching.

  # Implementation Prompt
  **Goal:** Build the core backend (Rust) and frontend (Flutter/Next.js) infrastructure for OHC's native omnichannel chat system, replacing Chatwoot.
  **CUJ:** Maya (the baker) logs into her OHC app on her iPhone. She sees a unified inbox with an incoming Instagram DM and a Web Widget chat. She taps the Web Widget chat, reads the AI-generated context summary, taps "Generate Reply" to answer a question about vegan cakes, and sends it.
  **Acceptance Criteria:**
  - Create the PostgreSQL schema for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with strictly enforced RLS.
  - Implement Rust `axum` endpoints and WebSocket routes for sending/receiving real-time messages.
  - Implement a basic unified inbox UI in the frontend (mobile-first 375px layout) utilizing the premium OHC Translucent Glass token system.
  - Integrate a NATS/Redis event bus that broadcasts a `message.created` event, which the AI CS agent can listen to.
  - Verify full end-to-end flow using Playwright/E2E tests without any mocked API requests.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
