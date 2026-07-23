issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. We are retiring Chatwoot as an external third-party service/dependency to ensure tight integration with OHC's AI agents, seamless data modeling across our tenant architecture, and improved performance/reliability without external dependencies. This shift is critical for owners like Maya, Carlos, and Priya who rely on unified customer interactions (Instagram DMs, SMS, Web Chat) being directly managed and actionable by their AI assistant.

  ## Research Report
  - **Chatwoot Source Code Audit**: Investigated the core data models of Chatwoot (`Conversation`, `Message`, `Contact`, `Inbox`).
  - Chatwoot handles omnichannel natively via different `channel_type` associated with an `Inbox`.
  - Conversations track `status`, `snoozed_until`, `priority`, and timestamps (`contact_last_seen_at`, `agent_last_seen_at`).
  - Messages handle `content_type`, `message_type`, and rich `content_attributes`.
  - Contacts are unique per `account_id` (tenant) and `identifier`/`email`/`phone_number`.
  - **OHC Architecture Gap**: Currently OHC's `inbox.proto` has basic definitions (`OmniMessage`, `Conversation`) but lacks the comprehensive channel routing, real-time WebSocket messaging, AI agent handoff protocols, and SLA tracking that a full omnichannel system requires.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : configures
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Channel : has
      Contact ||--o{ Conversation : initiates
      Inbox ||--o{ Conversation : contains
      Conversation ||--o{ Message : has
      Conversation ||--o{ AgentBot : assigned_to
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox View**: The owner sees a consolidated list of conversations across all channels (Instagram, SMS, Web) in a list view with clear channel icons and unread status.
  2. **Conversation Thread**: Tapping a conversation opens a chat thread. The AI assistant's draft replies are highlighted in a translucent glass container for 1-tap approval.
  3. **Context Pane**: A swipe-left or toggle button reveals the Contact context (past orders, lifetime value, preferences) natively fetched from OHC's core systems.
  4. **Action Menu**: Bottom sheet actions allow creating a booking, sending a payment link, or escalating to human natively within the chat flow.

  ### AI Agent Integration Points
  - **Work Triage**: Triggers on new `Message` creation. Analyzes intent, associates with `Contact`, and determines urgency.
  - **Customer & Relationship Assistant**: Drafts replies by analyzing `Conversation` history and `Contact` context. Submits draft `Message` for owner approval.
  - **Operations Assistant**: Extracts structured data (e.g., booking dates, custom order details) from `Messages` to generate `Tasks` or `Bookings`.

  ### Key Design Decisions
  - **Native Rust Implementation**: The entire backend (WebSockets, REST API, job processing) will be written in Rust within the `onehumancorp/mono` repo to guarantee performance and strict multi-tenant data isolation (`tenant_id` on every row).
  - **Unified Protobuf Definitions**: Expand `src/proto/inbox.proto` to support detailed channel types, message types, and AI agent handoff states.
  - **Zero Trust Security**: All endpoints and WebSocket connections will rely strictly on SPIFFE/SPIRE for identity and auth, ensuring no cross-tenant data leakage.

  ## Implementation Prompt
  Implement the native Rust omnichannel chat system as a replacement for Chatwoot. Start by expanding the `inbox.proto` and database schemas to model Inboxes, Channels, Contacts, Conversations, and Messages with strict row-level multi-tenant isolation. Implement the gRPC/REST services for conversation fetching, message creation, and real-time WebSocket event broadcasting. Ensure the UI components (in Flutter/Web) support a 375px mobile-first unified inbox view with translucent glass styling and AI draft approval interactions. Write full unit and Playwright E2E tests verifying the end-to-end journey of a customer message arriving, the AI drafting a reply, and the owner approving it.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
