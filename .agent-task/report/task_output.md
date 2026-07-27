issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine. Previously, we relied on Chatwoot, but as an external third-party service, it introduces latency, external dependency risks, and violates our single-platform, hybrid-architecture Zero-Trust principles. We must retire Chatwoot completely and implement a native Rust matching architecture within `onehumancorp/mono` that handles omnichannel data models, controllers, WebSocket real-time messaging, and inbox architecture, heavily tailored for non-technical owner/operators (e.g. Maya the baker, Carlos the handyman).

  # Research Report
  **Chatwoot Source Code Benchmarking Findings:**
  An exhaustive audit of the `chatwoot/chatwoot` source repository reveals the core data models necessary for parity:
  - **Account/Tenant:** Multi-tenancy root.
  - **Inbox & Channel Adapters:** Inboxes aggregate conversations. Channels (WhatsApp, Email, Instagram DM, Web Widget) require specialized adapters (`Channel::Whatsapp`, etc.).
  - **Contact & ContactInbox:** Unifies customer identity across different channels (omnichannel graph).
  - **Conversation & Message:** Core models for tracking thread state, assignments, and individual chat bubbles.
  - **AgentBots / AI Handoff:** Chatwoot uses `agent_bot.rb` for automated handling. In OHC, this maps to our AI agents.
  - **Real-time Engine:** Chatwoot relies on ActionCable/WebSockets. OHC will need an asynchronous, high-concurrency WebSocket server (via `tokio`/`axum`).

  **Competitive Analysis (OHC vs. Chatwoot vs. Shopify Inbox):**
  - OHC's implementation will not just be a unified inbox for human agents (like Chatwoot or Shopify). It will directly integrate with our AI departments (The Ambassador, The Manager), drafting replies contextually based on unified customer history, shifting the paradigm from "read & reply" to "read & approve" for the business owner.

  # Design Doc
  ## Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[External Webhooks: WhatsApp/Insta/Email] -->|HTTP POST| B(Gateway / API Layer - Axum)
      C[Mobile/Web Clients 375px] -->|WebSocket / WSS| B
      B --> D{Omnichannel Routing Service}
      D --> E[Native Rust Chat Engine]
      E --> F[(PostgreSQL - Unified Graph DB)]
      E --> G[Redis / PubSub - Event Mesh]
      G --> H[AI Ambassador Agent]
      H -->|Drafts Reply| E
      E -->|Broadcasts Action Required| C
  ```

  ## UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Feed (Mobile):** The owner's homepage shows pending interactions in translucent, glassmorphism cards.
  - **Interaction:** A card titled "Message from Carlos (WhatsApp)" shows context ("Carlos booked a repair 2 weeks ago").
  - **Action required:** Below the context, a drafted reply sits ready. The owner taps a prominent, 44x44px target "Approve & Send" button.
  - **Offline-Tolerance:** Reads are cached locally; drafted approvals are queued and dispatched reliably when the network returns.

  ## AI Agent Integration Points
  - **Event Mesh Trigger:** When a new `Message` is created and stored in PostgreSQL, an event is emitted to Redis PubSub.
  - **The Ambassador:** Subscribes to new message events, queries the `Contact` history via RAG against the tenant's context, and inserts a draft message.
  - **Locking:** Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) ensures multiple agents do not draft replies simultaneously.

  ## Key Design Decisions
  - **Zero Trust & Multi-Tenancy:** The database schema must enforce row-level security (RLS) on `tenant_id` for all models (Inbox, Conversation, Message, Contact). All APIs require strict SPIFFE/OIDC context.
  - **Native Rust Axum WebSockets:** Replaces Ruby ActionCable for massive concurrency and lower memory footprint, vital for real-time typing indicators and instant AI draft rendering.
  - **Data Modeling:** Adopt a polymorphic channel strategy where `Conversation` belongs to a `ContactInbox`, and `ContactInbox` belongs to a unified `Contact`.
  - **Estimated Scope**: Large.

  # Implementation Prompt
  **User-Facing Outcome:** A non-technical business owner receives customer queries from WhatsApp, Instagram, and web chat into a single mobile view (375px). The system proactively drafts intelligent replies based on past interactions, allowing 1-tap approval without leaving the main feed.
  **CUJ & Acceptance Criteria:**
  1. Implement Rust structs and DB entities for `Tenant`, `Contact`, `Inbox`, `Conversation`, and `Message` with `tenant_id` RLS constraints.
  2. Implement an Axum WebSocket handler that allows clients to subscribe to conversation updates.
  3. Implement a generic Channel Trait/Adapter pattern to ingest payloads from different external webhooks.
  4. Ensure a new incoming message successfully emits an event to the AI Job Queue (Redis PubSub / PG SKIP LOCKED).
  5. Provide minimum 5 Playwright E2E tests verifying the real-time UI flow: user logs in, receives a message through a repo-provided local adapter (or test-mode credentials, strictly zero UI mocks), sees the real-time update in the mobile-first UI, and clicks "Approve" to resolve the thread.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
