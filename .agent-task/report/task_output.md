issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently lacks a native omnichannel messaging and customer support engine, and relying on third-party services like Chatwoot introduces multi-tenancy risks, scaling bottlenecks, and fragmentation of the owner's workspace. Maya, Carlos, and Priya need a unified inbox that integrates seamlessly with OHC's internal agentic workflows (e.g., auto-drafting replies to Instagram DMs, quoting service requests via WhatsApp, and providing localized support). Chatwoot as an external service is 100% retired, and OHC requires a high-performance, multi-tenant omnichannel chat system built natively in Rust.

  ## Research Report
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals the core entities required for a robust omnichannel engine:
  - **Conversations & Messages:** The core ledger of communication.
  - **Inboxes & Channels:** Adapters for various mediums (Web Widget, Email, API, WhatsApp, Instagram, FB Messenger).
  - **Contacts & Contact Inboxes:** Resolving identities across different channels.
  - **Agents, Teams & Routing:** Assignment policies (round-robin, manual) and agent bot routing.
  - **Automations, Macros & Canned Responses:** Rule-based triggers for message handling.

  Comparing this to OHC's requirements, we need strict row-level tenant isolation (via `tenant_id`), integration with our Gemini/OpenAI-powered AI agents, and a mobile-first Flutter UI that feels like an owner's command center rather than a disconnected helpdesk.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Flutter Web/Mobile App] -->|gRPC / REST| API[Rust API Gateway]
      API --> Auth[SPIFFE/SPIRE Auth & Tenant Middleware]
      Auth --> ChatEngine[Rust Omnichannel Chat Service]

      ChatEngine --> ChannelWeb[Web Widget Channel]
      ChatEngine --> ChannelSocial[Social Channels - IG/WA/FB]
      ChatEngine --> ChannelEmail[Email Channel]

      ChatEngine --> DB[(PostgreSQL with RLS)]
      ChatEngine --> Cache[(Redis / Redlock)]

      ChatEngine --> AIQueue[(AI Agent Job Queue - SKIP LOCKED)]
      AIQueue --> OperationsAgent[AI Operations / Triage Agent]
      AIQueue --> CSReplyAgent[AI Customer Reply Agent]
  ```

  ### Data Model & Invariants
  - `tenant_id` must be present on every table with RLS enabled.
  - **Inbox**: `id`, `tenant_id`, `name`, `channel_type`, `channel_id`.
  - **Conversation**: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, pending), `agent_id`, `agent_bot_id`.
  - **Message**: `id`, `tenant_id`, `conversation_id`, `content`, `message_type` (incoming, outgoing, template), `sender_type`, `sender_id`.
  - **Contact**: `id`, `tenant_id`, `name`, `email`, `phone`, `avatar_url`.

  ### Mobile UX Flow (375px First)
  - **Inbox View**: Clean, Apple-style list of active conversations. Translucent glass app bar. Unread indicators and AI draft badges.
  - **Conversation Thread**: Native feeling chat bubbles. AI suggested replies appear at the bottom above the native keyboard. One-tap "Approve & Send" or "Edit".
  - **Contact Sidebar (Drawer)**: Hidden by default, swipe to reveal customer context (past orders, lifetime value, upcoming bookings) pulled from OHC's operational data.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Listens to new `Conversation` creations, tags them, and routes to appropriate team members or AI departments.
  - **Customer Reply Agent**: Listens to incoming `Messages`, uses tenant-scoped RAG (knowledge base, past orders), and inserts an AI-drafted `Message` (marked as a draft) into the thread for owner approval.

  ## Implementation Prompt
  **For the Implementer Agent:**
  1. **Backend:** Implement the native Rust omnichannel chat system within `onehumancorp/mono`. Define the core data entities (Inboxes, Conversations, Messages, Contacts) with strict PostgreSQL Row-Level Security (`tenant_id`). Implement the gRPC/REST APIs to support the CRUD operations.
  2. **WebSocket / Real-time:** Implement real-time message broadcasting using Rust (e.g., using Tokio/Tonic or Axum WebSockets).
  3. **Frontend (Flutter):** Build the mobile-first (375px) Unified Inbox UI. Integrate the OHC Premium Token library (translucent materials, clean hierarchy). Ensure the UI correctly displays AI-drafted messages and allows the owner to approve/send them.
  4. **Tests:** Achieve 100% unit test coverage for the Rust crates and Flutter widgets. Write full-loop Playwright E2E tests for the "Receive Message -> AI Draft -> Owner Approve -> Send" Critical User Journey. ZERO mock data in the UI.

  **Estimated Scope:** Large

  ## Top 5 things that do not make sense in the repository to fix later
  1. No `tests` directory in `src/server/integrations/chat` to accompany `README.md`.
  2. Incomplete `.bazelrc` configuration for Rust toolchains as evidenced by `rules_rust+` fetch failures on some environments.
  3. `vitest.config.ts` exists but a lot of tests are unwritten or mock-based contrary to the ZERO mock data rule.
  4. Mobile-first (375px) is a strict requirement, but E2E tests for `playwright` don't explicitly force 375px viewport in all configs.
  5. The repo relies on an external `Chatwoot` instance in some older documentation (`docs/business/market_research/omnichannel_unified_inbox.md`), despite the directive to fully retire it.

  Acceptance Criteria: A user can open the OHC app, view a unified inbox of messages, see an AI-generated draft for an incoming message, and approve it to send.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
