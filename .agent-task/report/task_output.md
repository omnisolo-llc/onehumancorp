issue_title: "Architecture Design: Native Rust Omnichannel Chat Engine & Multi-Tenant Support System"
issue_description: |
  # OmniSolo (OHC) Native Rust Omnichannel Chat System

  ## 1. Problem Statement & Opportunity
  OmniSolo aims to be an AI work assistant for small business owners and operators (Maya, Carlos, Priya, Leo, Fatima, etc.). Communication is their primary demand intake and customer interaction surface. Relying on heavy, complex, and unintegrated external tools like Chatwoot introduces security risks, sync latency, third-party operational dependencies, and a high-barrier developer setup. By retiring Chatwoot completely and building a high-performance, OHC-native, multi-tenant-safe Omnichannel Chat Engine natively in Rust, OmniSolo can achieve perfect local-first/offline integration, Zero-Trust compliance, and seamless multi-agent intelligence.

  ## 2. Research Report
  ### Competitive Analysis (Shopify Inbox, Wix Chat, Chatwoot)
  - **Shopify Inbox**: Deeply integrated into the Shopify Core. It is not an admin portal; it is an operator-focused chat app with mobile native push notifications, automated discount offers, and product variant suggestions.
  - **Wix Chat**: Simple widget with instant triggers, localized bookings, and lead captures.
  - **Chatwoot**: Powerful open-source omnichannel platform, but written in Ruby on Rails, requiring Postgres, Redis, Sidekiq, and ActionCable. This is prohibitively complex for standalone desktop mode or resource-constrained clusters.
  - **The Native OHC Solution**: By replicating Chatwoot's omnichannel concepts (Inboxes, Channels, Contacts, Contact-Identities, Conversations, Messages, and Receipts) in high-performance Rust, we reduce the footprint, eliminate external Rails/Sidekiq dependencies, and allow the exact same codebase to run in multi-tenant cloud-native PostgreSQL (secured by Row-Level Security) and desktop SQLite (isolated via localized predicates).

  ## 3. High-Level Architectural Design (Design Doc)

  ### 3.1 Architecture Overview & Data Flow
  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      actor Operator
      participant Widget as Web Widget / Connector
      participant Ingress as Rust HTTP Webhook Ingress
      participant DB as Database (Postgres RLS / SQLite)
      participant Realtime as Realtime Gateway (WS / PowerSync)
      participant AI as AI Department (CS, Ops, Revenue)

      Customer->>Widget: Sends message
      Widget->>Ingress: Normalizes & signs payload
      Ingress->>DB: Resolves tenant/contact, appends Message (Transaction)
      DB-->>Ingress: Commits transaction & appends Outbox Event
      Ingress-->>Realtime: Publishes state update
      Realtime-->>Operator: Real-time broadcast (WebSocket/PowerSync)
      Ingress-->>AI: Triggers AI analysis background task
      AI->>DB: CS Agent drafts reply + Ops Agent checks availability
      DB-->>Realtime: Publishes Draft event
      Realtime-->>Operator: Renders translucent glass draft card
      Operator->>Realtime: Approves draft (clicks "Send")
      Realtime->>DB: Transitions draft to committed
      DB->>Widget: Delivers message outbound
  ```

  ### 3.2 Data Models & Schema Invariants
  - **Multi-Tenant Row-Level Security**: Every table must have a `tenant_id UUID NOT NULL` column. PostgreSQL applies:
    `CREATE POLICY tenant_isolation_policy ON <table_name> FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);`
  - **Deduplication and Idempotency**: Unique constraints on `(tenant_id, channel_type, provider_message_id)` prevent duplicate webhooks.

  ### 3.3 Mobile UX Flow (375px Viewport)
  - **Adaptive Shell**: The 375px screen uses a focus-first single-pane view.
    - **Pane 1 (Conversation List)**: Showing active, snooze, and open threads. Users swipe left to snooze/resolve, and swipe right to delegate to AI.
    - **Pane 2 (Active Thread)**: A beautiful translucent glass conversation timeline. Under the timeline, a context-aware bottom sheet displays:
      1. **Active AI Draft**: Pressing a prominent "Approve & Send" button instantly sends the AI draft.
      2. **Customer CRM Context**: Displays customer attributes, tags, and previous history.
      3. **Action Drawer**: One-click shortcuts to schedule visits, draft quotes, or request payments.
  - **Touch Targets**: All interactive elements (e.g., Quick replies, layout toggles, close buttons) adhere to a minimum size of 44x44px.

  ### 3.4 AI Department Coordination
  - **Triage & Classifier**: Classifies intent, sentiment, language, and maps metadata.
  - **CS Agent ("The Ambassador")**: Formulates personalized answers from localized RAG knowledge bases, offering multi-language translations.
  - **Operations Agent**: Coordinates with bookings, delivery times, and inventory levels.
  - **Revenue Agent**: Compiles proposals and Stripe payment link drafts.

  ### 3.5 Zero-Trust Security & Real-Time Sync
  - **Real-Time Ticket Gateway**: To ensure absolute security, WebSocket authentication does not supply JWTs in query strings. Clients use a short-lived (60s) single-use cryptographically signed ticket obtained via `POST /api/v1/auth/realtime-ticket`.
  - **PowerSync Isolation**: Real-time persistent state convergence filters tables by tenant ID and team membership claims.

  ## 4. Implementation Prompt for the Engineering Swarm

  ### 4.1 Objective & Context
  Implement the OHC-native, multi-tenant-safe Omnichannel Chat Engine in Rust inside `src/server/services/chat/`. Consolidate the schemas, provide high-performance Axum handlers for webhooks/API operations, and build a secure, token-authorized WebSocket/ActionCable-style real-time delivery protocol.

  ### 4.2 Critical User Journey (CUJ)
  1. **Channel Connection**: Admin creates an Inbox (`chat_inboxes`) and adds a Web Widget Channel (`chat_channels`).
  2. **Inbound Ingestion**: A customer sends a message. The API normalizes the payload, resolves/creates the Contact (`chat_contacts`) and ContactIdentity, creates a Conversation (`chat_conversations`), commits the Transaction, and emits an Outbox Event.
  3. **AI Automated Draft**: CS AI Agent classifies the message, drafts a translation/reply, and registers the draft in `chat_messages`.
  4. **Real-Time Delivery**: The draft is pushed instantly over the WebSocket channel.
  5. **Human Approval**: The operator views the thread on a 375px viewport, reviews the AI draft, approves it, and the message is delivered outbound.

  ### 4.3 Technical Requirements & Acceptance Criteria
  - **Clean Architecture**: Follow the established Rust layout. Expose endpoints via Axum.
  - **Multi-Tenant Integrity**: Every query must enforce the verified `tenant_id`. All PostgreSQL migration scripts must enable RLS.
  - **Test Suite**: Provide a shared contract test suite that runs successfully on both PostgreSQL and SQLite, asserting isolation, delivery states, and error handling. Achieve 100% test coverage.
  - **Zero Secrets & Security**: Authenticate WebSocket upgrades using secure, single-use `realtime-tickets`.

  ## 5. Metadata
  - **Priority**: P0 (Core capability)
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
