issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external Chatwoot services for omnichannel customer support and chat functionality. Chatwoot as an external third-party service is being 100% RETIRED from OHC. We need a native, high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono` to ensure 100% feature parity with Chatwoot while adhering to our zero-trust, multi-tenant, and mobile-first architectural mandates.

  ## Research Report
  - **Chatwoot Architecture**: An audit of the `chatwoot/chatwoot` source code reveals core concepts such as: Accounts, Users, Inboxes (Channels like Web Widget, Email, API, WhatsApp, etc.), Conversations, Messages, Contacts, Labels, Campaigns, Macros, Canned Responses, Agent Routing, and SLA Policies.
  - **Data Models**: Chatwoot extensively uses relational models (e.g. `conversations`, `messages`, `inboxes`, `contacts`, `channel_web_widgets`, `channel_api`).
  - **Real-time Messaging**: It uses WebSockets via ActionCable for real-time dispatching.
  - **Agentic Workflow Integration**: OHC's version needs deep integration with AI agents (Operations, CS, Marketing) for automatic responses, triage, and SLA enforcement, moving beyond standard human-only routing.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--|{ INBOX : maps_to
      MESSAGE }o--|| AGENT : created_by
  ```

  - **System Components**:
    - **API Layer**: Rust (Axum/Tonic based) for REST and gRPC endpoints handling inbox management, conversation retrieval, and message sending.
    - **WebSocket Gateway**: High-performance Rust WebSocket server for real-time push events to client devices and web dashboards.
    - **Channel Adapters**: Pluggable interface for different channels (Web Widget, Email, API, etc.).
    - **Routing & SLA Engine**: Background worker (Rust) matching Conversations to Agents/AI based on load and priority.
  - **Multi-tenant Isolation**: All data models MUST include `tenant_id` with Row Level Security (RLS) in PostgreSQL. Cross-tenant queries are forbidden.
  - **Mobile UX Flow (375px first)**:
    - **Inbox View**: A clean, unified list of conversations. Unread indicators and tags (e.g., "AI Draft Ready") prominent. Bottom sheet for filtering.
    - **Conversation View**: Familiar chat interface (like iMessage/WhatsApp). Translucent glass navbar. Bottom input area with quick-actions (macros, AI assist, file attachment). Touch targets >= 44x44px.
  - **AI Agent Integration Points**:
    - **Work Triage**: AI listens to new inbound messages, categorizes them, and optionally drafts a reply.
    - **CS Assistant**: Capable of generating complete, context-aware replies based on tenant knowledge base.

  ## Implementation Prompt
  **Goal**: Implement the core Rust data models, channel adapters, API endpoints, and a mobile-first UI for the native Omnichannel Chat System that replaces Chatwoot.
  **CUJ**: An owner (e.g., Maya) opens the OHC app, navigates to the unified Inbox, views a new Instagram DM inquiry (routed through the unified system), sees an AI-drafted reply, and taps "Send".
  **Acceptance Criteria**:
  1. Define Rust data models for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` isolation.
  2. Implement backend REST/gRPC APIs for retrieving and sending messages.
  3. Build the Flutter/Web mobile-first UI for the Inbox and Conversation views, utilizing OHC Premium Tokens (Translucent Glass).
  4. Integrate a WebSocket gateway for real-time message delivery.
  5. 100% Unit and E2E (Playwright) test coverage for the CUJ.
  6. ZERO mock data in UI code.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
