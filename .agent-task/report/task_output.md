issue_title: "Architecture Design: Native Rust Omnichannel Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat Engine

  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a high-performance, native omnichannel chat and inbox system. Relying on an external or legacy dependency like Chatwoot introduces cross-system latency, fractures our multi-tenant Zero Trust security model, and creates UI friction for non-technical owner/operators. Small-business owners (like Maya the Baker or Carlos the Handyman) need unified messaging (WhatsApp, Web Widget, Email, Instagram DMs) fully integrated with their main work assistant, without having to manage a separate "support tool."

  ## Research Report & Feature Benchmarking
  Following a deep source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`), the following core capabilities and architectural patterns were benchmarked for replication:
  - **Data Models:** `Account` (mapped to OHC `tenant`), `Inbox`, `Channel` (WebWidget, Email, API, WhatsApp, etc.), `Conversation`, `Message`, `Contact`, `ContactInbox`, `User` (mapped to OHC owners/staff).
  - **Channel Adapters:** Webhooks processing to handle incoming messages uniformly.
  - **Real-Time Communication:** WebSockets over ActionCable in Chatwoot must be replaced with native Rust async WebSockets (e.g., using `tokio` and `axum` or `tungstenite`) directly integrated with our Redis pub/sub mechanism.
  - **Automation & Routing:** SLA policies, macros, canned responses, and assignment policies (round-robin, manual).
  - **Agentic Integration:** OHC's LLM agents (Triage, Customer Assistant) need direct programmatic hooks into the new Rust chat service to draft replies and trigger operations.

  Unlike Chatwoot (Ruby on Rails), our implementation will be built in native Rust, ensuring sub-millisecond response times, minimal memory footprint, strict compile-time safety, and built-in row-level multi-tenancy.

  ## Design Doc
  ### High-Level Architecture (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer Web Widget / WhatsApp / IG] -->|Webhooks / WebSockets| B[OHC API Gateway]
      B --> C[Rust Chat Microservice / Axum]
      C --> D[PostgreSQL with RLS]
      C --> E[Redis / Valkey PubSub]
      C --> F[OHC AI Agent Queue]
      F --> G[LLM Processing - Customer Assistant]
      G --> C
      E -->|WebSocket Broadcast| H[Owner Mobile/Web App]
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox View:** The owner opens the OHC mobile app. The main feed consolidates messages from all channels.
  2. **Thread View:** Tapping a message opens a clean, macOS-style translucent chat interface.
  3. **Agent Drafts:** AI-generated replies appear instantly as "Suggested Drafts" beneath the customer's last message, requiring just one tap to approve and send.
  4. **Context Panel:** A swipeable right drawer reveals customer history, active orders, and notes (eliminating the need to switch tabs).

  ### AI Agent Integration
  - **Operations Dept:** Listens to the Rust Chat event bus. Upon a new `ConversationCreated` or `MessageCreated` event, it analyzes intent.
  - **Drafting:** If the intent is actionable (e.g., "Do you do vegan cakes?"), the agent pushes a draft message via a gRPC or internal REST call back to the Rust chat service, tagged as `agent_draft`.

  ### Key Design Decisions
  - **Language:** Rust (Axum/Tokio) for the core service.
  - **Data Isolation:** Strict row-level security (`tenant_id`) enforced at the DB query level in Rust.
  - **Event Bus:** Redis Redlock and Pub/Sub for distributed state and AI handoffs.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the foundational Rust models, database migrations, and Axum handlers for the core `Inbox`, `Conversation`, and `Message` entities, replicating Chatwoot's data architecture but localized to OHC's multi-tenant PostgreSQL schema.
  - **CUJ:** An owner logs in, navigates to the Inbox, sees a combined list of conversations from different channels, taps one, and can send a reply.
  - **Acceptance Criteria:**
    - Migration files created for `inboxes`, `conversations`, and `messages` with `tenant_id` RLS policies.
    - Rust HTTP endpoints (CRUD) for these entities.
    - Rust WebSocket endpoint established for real-time `MessageCreated` broadcasts.
    - Playwright E2E tests validating the creation and viewing of messages via the UI.
    - 100% unit test coverage for the new Rust modules.
    - The UI must use translucent glass styling and remain fully functional at 375px.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
