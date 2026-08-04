issue_title: "[research] Architect Custom Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is replacing the external chat provider dependency with a custom, native Rust omnichannel chat system inside `onehumancorp/mono`. Currently, the system lacks the robust core structures for real-time messaging, inbox assignments, and multi-tenant isolation that owners (Maya, Carlos, Priya, Leo, Fatima) need to aggregate and triage all their communications (Instagram DMs, Web Chat, Email, SMS) seamlessly into a single prioritized work feed. They need this built native so it’s offline-tolerant, blazingly fast on 375px screens, and completely embedded in OHC.

  ## Research Report
  - **Market Benchmark**: Benchmarked against leading customer service tools like Zendesk and Intercom, as well as the former external dependency. The legacy tool provides an excellent reference for omnichannel data modeling (`app/models/conversation.rb`, `inbox.rb`, `contact.rb`).
  - **Codebase Audit**: Our Rust backend uses `axum` and `tokio-tungstenite` for WebSockets, and `sea-orm` for Postgres data persistence. We need robust SeaORM schemas and Axum handlers to replicate the core functionalities of the retired omnichannel platform.
  - **Findings**: The missing link in OHC is a unified `Inbox` and `Conversation` data model that ties together multiple channel providers (Twilio SMS, Resend Email, Meta Instagram, Custom Web Widget) and allows AI agents to intervene, draft, and triage automatically.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Channels[External Channels: SMS, Web, Insta] --> WebhookAPI(Axum Webhooks & Endpoints)
      WebhookAPI --> ChannelAdapters(Rust Channel Adapters)
      ChannelAdapters --> CoreInboxLogic(Inbox & Routing Engine)
      CoreInboxLogic --> Database[(PostgreSQL/SeaORM)]
      Database --> Models[Models: Inbox, Conversation, Message, Contact]
      CoreInboxLogic --> RealTime[WebSocket PubSub / NATS]
      RealTime --> FrontendApp[Tauri/Flutter Client 375px Mobile First]
      CoreInboxLogic --> AIAgents[AI Agent Queue - Drafts & Triage]
  ```

  ### Data Model Invariants (Multi-Tenant First)
  - **`inboxes`**: `id`, `tenant_id`, `name`, `channel_type`
  - **`contacts`**: `id`, `tenant_id`, `name`, `email`, `phone`
  - **`conversations`**: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, snoozed, resolved)
  - **`messages`**: `id`, `tenant_id`, `conversation_id`, `content`, `message_type` (incoming, outgoing, internal_note), `sender_type` (user, contact, ai_agent)
  - **Strict Multi-Tenancy**: EVERY query to these tables MUST include `tenant_id` and utilize PostgreSQL Row Level Security (RLS) where applicable.

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home Screen)**: Unified list of active `conversations` sorted by urgency. Clean "Ubiquiti UniFi" card layout.
  2. **Conversation View**: Tap a conversation -> open a chat UI. Native mobile keyboard support. Messages stream in real-time via WebSockets.
  3. **AI Draft Action**: Floating action button allows the owner to "Draft Reply" via the AI Customer Assistant, which uses the context of the `conversation` to prepare a response.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the core backend data schemas (SeaORM entities) and basic service layer for the native Rust Omnichannel Chat System, completely retiring the external chat dependency.
  1. Create the `Inbox`, `Contact`, `Conversation`, and `Message` SeaORM entities within `src/server/ohc/entity/` or a new dedicated chat module.
  2. Ensure every entity requires a `tenant_id` for strict multi-tenant isolation.
  3. Implement the basic service functions to create an inbox, start a conversation, and add a message.
  4. Build comprehensive unit tests mocking the DB to verify multi-tenant isolation.
  5. The UI must render a unified triage feed on a 375px mobile viewport using standard macOS Translucent Glass styling. (Create the foundational data layer first, UI integrations will follow).

  ## Priority
  `P0` (Critical - Required for core work triage)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
