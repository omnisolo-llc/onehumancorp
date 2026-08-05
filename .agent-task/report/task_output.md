issue_title: "Architect Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Currently, owners and operators like Maya (baker) and Carlos (handyman) need to manage customer interactions across multiple channels (Instagram DMs, WhatsApp, SMS, web chat) seamlessly. However, relying on an external, third-party service like Chatwoot introduces multi-tenancy risks, latency, and disrupts the seamless, unified "assistant-first" experience OHC promises. To provide an immediate, secure, and fully integrated experience—where the AI assistant can proactively triage messages and draft replies—OHC needs a native, high-performance omnichannel inbox built directly into its Rust ecosystem.

  ## Research Report
  - **Market Context**: Platforms like Shopify (Shopify Inbox) and Wix have built native inboxes to keep owners on-platform and integrate closely with their core catalog and order data. Chatwoot provided a robust open-source model (Ruby on Rails/Vue) for multi-channel communication, but its monolithic architecture was not optimized for OHC's Rust/Bazel environment and AI-first workflows.
  - **Chatwoot Audit & Removal Plans**: Per `docs/superpowers/plans/2026-07-13-chatwoot-removal.md`, Chatwoot is 100% RETIRED from OHC. All references, Helm charts, docker containers, and integrations must be purged. We are replacing its functionality with a native inbox architecture (`docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`).
  - **Omnichannel Unified Inbox Vision**: Based on `docs/business/market_research/omnichannel_unified_inbox.md`, the AI Agent (The Ambassador) must proactively draft replies by querying the customer's omnichannel identity graph (purchase history, past bookings, previous DMs) before the owner even opens the app.
  - **Capabilities Required**:
    - **Data Models**: We need native Rust models for `Inboxes`, `Conversations`, `Messages`, and `Contacts`, tightly coupled to OHC tenants.
    - **WebSocket Real-time**: High-performance Rust WebSocket implementation for real-time messaging.
    - **AI Drafting**: Proactive drafting of responses using RAG against product catalog and customer history.
    - **Tenant Isolation**: Securely isolated by tenant ID (`ENABLE ROW LEVEL SECURITY`) via high-performance gRPC/Rust APIs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| AI_AGENT_DRAFT : triggers

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          string provider
          jsonb credentials
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Triage Feed (Home)**: Owner opens the app to see a unified "Work Triage" feed. Urgent messages (e.g., "Is my cake ready?") are pinned at the top.
  2. **Conversation View**: Tapping a message opens a clean chat thread. Translucent glass app bar with UniFi-style spacing.
  3. **AI Drafts**: At the bottom input area, an AI-generated draft reply is already waiting (e.g., "Yes, your cake is ready for pickup!").
  4. **Action**: The owner taps "Send" to approve the draft, or edits it via the native mobile keyboard.

  ### AI Agent Integration
  - **Work Triage Agent**: Listens to new `Message` inserts via Postgres trigger/CDC, categorizes intent, and updates the `Conversation` status.
  - **Customer Assistant Agent**: Generates draft replies (`AI_AGENT_DRAFT`) based on tenant context, past orders, and contact history.
  - **Observability**: AI interactions are logged. Lock key pattern `ohc:lock:{tenant_id}:conversation:{id}` ensures multiple agents don't process the same message simultaneously.

  ### Key Design Decisions
  - **Native Rust**: Replaces Rails monolith with a Rust-based gRPC microservice for maximum performance and lower memory footprint.
  - **Polymorphic Channels in Rust**: Use Rust Enums to handle different channel types (Web, Instagram, WhatsApp) safely.
  - **AI-First Abstraction**: Rather than just humans reading messages, all incoming messages hit an AI preprocessing queue before notifying the owner, aligning with the OHC Assistant promise.
  - **Zero Chatwoot Dependency**: Entire system operates natively without any external dependencies on Chatwoot APIs or databases.

  ## Implementation Prompt
  **Role**: Implementer Agent (Backend/Frontend)
  **Objective**: Implement the core data models, Rust gRPC service, and mobile-first Flutter UI for the native OHC Omnichannel Inbox, replacing the deprecated Chatwoot implementation.
  **CUJ**: Maya receives an Instagram DM. The system ingests it, the AI drafts a reply, and Maya opens the OHC mobile app (375px view), sees the draft in her unified inbox, and taps "Send".
  **Acceptance Criteria**:
  1. Create PostgreSQL schema migrations for `inboxes`, `conversations`, `messages`, and `contacts` with `tenant_id` RLS.
  2. Implement Rust gRPC endpoints for fetching conversations and sending messages.
  3. Build a Flutter UI screen for the Conversation view that supports 375px width, utilizing the OHC Translucent Glass design tokens.
  4. Ensure 100% unit test coverage for the new Rust services.
  5. Add a Playwright E2E test verifying the inbox creation and message sending flow (no mocked UI data).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []