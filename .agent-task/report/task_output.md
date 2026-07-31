issue_title: "[Native Rust Chat] Architect & Implement OHC Omnichannel Unified Inbox (Retire Chatwoot)"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) lacks a deeply integrated conversational interface, risking reliance on external dependencies (like Chatwoot) which leads to fragmented context, multi-tenancy risk, and lack of real-time control. We must retire any external chat dependency and build a native omnichannel inbox. Our core business personas—Maya (baker managing Instagram DMs), Carlos (handyman handling SMS/WhatsApp), and Priya (boutique owner managing web inquiries)—need a unified, AI-assisted inbox. This must be a native Rust implementation directly inside the OHC stack to ensure high-performance, strictly isolated multi-tenant execution, and deep integration with our existing AI background workers.

  ## Research Report
  - **Chatwoot Codebase Audit**: An analysis of Chatwoot's source code (`app/models`, `app/controllers`) reveals their core domain models: `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, and `Channel::*` (API, Email, FacebookPage, Instagram, Line, SMS, Telegram, WebWidget, Whatsapp). Chatwoot leverages ActionCable for WebSocket-based real-time updates and heavily relies on background workers for SLA and macro execution.
  - **Competitor Systems**: Shopify Inbox and Zendesk unify channels, but lack deeply integrated, autonomous AI agents capable of natively mutating business state (e.g., modifying an invoice during a chat). OHC needs our Rust microservices to seamlessly pipe messages to our AI department (Operations, Sales, Support).
  - **Gap**: OHC lacks a native, low-latency, strictly multi-tenant WebSocket chat infrastructure in Rust, as well as the persistence layer for Omnichannel messaging that supports mobile-first offline viewing.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Conversation : contains
      Inbox ||--o{ ChannelAdapter : configures
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : contains
      Message ||--o{ Attachment : has
      ChannelAdapter ||--o{ WebhookEvent : receives
  ```

  ### AI Agent Integration
  - **Routing Department**: New messages are picked up by the `Work Triage` agent queue via PostgreSQL `SKIP LOCKED`.
  - **Customer & Relationship Assistant**: Listens to the `Conversation` feed, fetching `Contact` history to draft context-aware replies.
  - **State Mutations**: Agents can propose actions (e.g., generate a payment link), locking the `Conversation` resource via Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to avoid race conditions with owner actions.

  ### Mobile-First UX Flow
  - **Viewport (375px)**: The unified inbox is the primary view. A translucent glass-styled bottom navigation bar allows switching between "Unresolved", "Pending AI Drafts", and "All".
  - **Conversation View**: Chat bubbles with clean spacing. AI-generated drafts appear in a distinct styling (e.g., subtle animated gradient border) with "Approve" and "Edit" action buttons.
  - **Offline/Flaky Network**: The Flutter frontend uses a local SQLite store. Sent messages are marked with a translucent clock icon and optimistic UI updates apply immediately, queuing the request for the Rust backend when the network recovers.

  ### Security & Zero Trust
  - Multi-tenant isolation at the row level via `tenant_id` on all entities (`ENABLE ROW LEVEL SECURITY`).
  - WebSockets authenticated via SPIFFE/SPIRE integrated tokens.
  - All external channel webhooks (e.g., WhatsApp, Instagram) are verified via signatures before entering the Rust message bus.

  ## Implementation Prompt
  **Implementer Agent Objective**: Build the core native Rust domain models, database migrations, and gRPC/REST APIs for the OHC Omnichannel Inbox, replacing Chatwoot.
  **CUJ**: A customer sends a message via a simulated Web Widget or WhatsApp webhook. The Rust backend persists the `Contact`, creates a `Conversation` in the appropriate `Inbox`, and broadcasts the `Message` payload over a WebSocket. The owner (e.g., Maya), using the Flutter web app (375px layout), sees the message pop up instantly, and an AI agent auto-generates a draft reply for her to approve.
  **Acceptance Criteria**:
  1. Rust structs and PostgreSQL migrations for `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` isolation.
  2. A WebSocket server in Rust capable of broadcasting message events to connected authenticated clients.
  3. API endpoints for creating an inbox, creating a message, and fetching conversation history.
  4. At least 5 E2E Playwright tests verifying the UI flow (sending a message, receiving it, and viewing an AI draft).
  5. 100% unit test coverage for the new Rust modules.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
