issue_title: "Native Rust Omnichannel Chat System Implementation"
issue_description: |
  ### Title
  Native Rust Omnichannel Chat System Implementation to Retire Chatwoot

  ### Problem Statement
  OHC currently relies on external third-party services like Chatwoot for omnichannel customer support and chat capabilities. This introduces latency, breaks strict tenant data isolation, creates disconnected agent experiences, and degrades mobile offline capabilities for our core owner/operator personas like Maya (baker) and Carlos (handyman) who need instant, unified triage of DMs and messages right in the OHC app. Chatwoot must be retired and replaced with a native Rust implementation.

  ### Research Report
  Based on an audit of the `chatwoot/chatwoot` source repository and OHC's current architecture:
  - **Chatwoot's Architecture:** Chatwoot uses a Rails monolith heavily dependent on PostgreSQL and Redis. Its data models center around `Accounts` (tenants), `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `ChannelAdapters` (e.g., Web Widget, API, WhatsApp). It uses ActionCable for WebSocket events and Sidekiq for background jobs.
  - **OHC Architecture Gap:** OHC requires a high-performance Rust-based API natively integrated into our multi-tenant PostgreSQL with strict `tenant_id` Row Level Security (RLS), orchestrated by our Kubernetes/Bazel stack.
  - **Competitor Systems:** Modern systems like Shopify Inbox and Stripe Customer interactions maintain extremely low latency via Edge-caching and native mobile websocket synchronization, which Chatwoot's generic architecture does not optimize for without heavy infrastructure.

  ### Design Doc
  **Architecture Diagram:**
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : owns
    TENANT ||--o{ CONTACT : owns
    INBOX ||--o{ CONVERSATION : tracks
    CONTACT ||--o{ CONVERSATION : initiates
    CONVERSATION ||--o{ MESSAGE : contains
    CHANNEL_ADAPTER ||--o{ INBOX : configures
  ```
  - **Data Model:** Core entities include `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`. All tables enforce multi-tenant isolation via `tenant_id` RLS.
  - **Mobile UX Flow (375px):**
    - A single unified "Work Triage" feed screen where WhatsApp, Web Widget, and Instagram DMs appear.
    - Tap a conversation -> opens a full-screen chat UI with translucent glass headers and a sticky native keyboard composer.
    - AI drafted replies appear as subtle suggestion chips above the composer.
  - **AI Agent Integration:** The Customer & Relationship Assistant monitors the `CONVERSATION` table via PostgreSQL SKIP LOCKED queue. It automatically drafts replies and updates customer context/tags invisibly before the owner even opens the app.
  - **Key Design Decisions:** Implement in Rust for maximum performance and low memory footprint in the OHC API layer. Use gRPC for internal services and REST/WebSockets for clients. Enforce `tenant_id` at the lowest repository layer.

  ### Implementation Prompt
  Implement the native Rust Omnichannel Chat API and corresponding Flutter UI components.
  - **CUJ:** As an owner (e.g., Maya), I want to receive a web widget message and a WhatsApp DM in a single unified mobile view, read the AI's drafted response, and hit "Send" instantly.
  - **Acceptance Criteria:**
    - Rust API endpoints for Inbox, Conversation, and Message CRUD operations.
    - PostgreSQL schema with strict `tenant_id` RLS.
    - WebSocket integration for real-time message delivery.
    - Flutter UI implementation matching the macOS translucent glass and UniFi modular dashboard design for 375px screens.
    - 100% Unit and Playwright E2E test coverage verifying the complete messaging loop.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
