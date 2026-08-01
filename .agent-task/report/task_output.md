issue_title: "Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  **Problem Statement**
  OneHumanCorp relies on external chatwoot dependencies or lacks a unified natively built omnichannel messaging inbox in Rust. Maya (the baker) selling cakes via Instagram DMs and WhatsApp needs a single, unified inbox to triage messages, turn inquiries into orders, and rely on AI drafts. Using a 3rd-party SaaS like Chatwoot breaks the multi-tenant Zero Trust architecture, creates operational complexity, and prevents deep AI integration with our orchestration mesh.

  **Research Report**
  As mandated, Chatwoot has been fully audited. Chatwoot's core entities include:
  - `Account` (maps to OHC `Tenant`)
  - `Inbox` (Channel configuration: Email, Facebook, Instagram, WhatsApp, Web Widget, API, etc.)
  - `Contact` (Customer profiles)
  - `ContactInbox` (Junction of Contact + Inbox + Source ID)
  - `Conversation` (Thread of messages)
  - `Message` (Individual text/multimedia payload)
  - `Channel` (Polymorphic associations for different channel types)

  OHC needs this exact architectural capability natively built in Rust within the `onehumancorp/mono` repo using PostgreSQL row-level security (`tenant_id`) and Redlock for concurrency.

  **Design Doc**
  - **Architecture Diagram (Mermaid)**:
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : configures
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Channel : has_adapter
      Contact ||--o{ ContactInbox : uses
      Inbox ||--o{ ContactInbox : contains
      ContactInbox ||--o{ Conversation : has
      Conversation ||--o{ Message : contains
      Message ||--o| AiDraft : optionally_has
  ```
  - **Mobile UX Flow (375px)**:
    - **Inbox Tab**: A clean Apple Mail-like list of active conversations, with translucent glass headers. Each row shows the contact name, platform icon (Instagram/WhatsApp/Email), and latest message snippet.
    - **Conversation View**: Similar to iMessage. Tapping a row opens the thread. The AI's suggested draft is prominently displayed at the bottom with a primary "Send" button and secondary "Edit" button.
  - **AI Agent Integration Points**:
    - `Message` creation via webhooks/API triggers an async event to the AI Job Queue.
    - `Operations Agent` or `Sales Agent` consumes the new conversation context.
    - AI writes an `AiDraft` attached to the last `Message`. The mobile UI polls or receives a WebSocket push to display this draft.
  - **Key Design Decisions**:
    - **Data Model**: Adopt Chatwoot's normalized structure (`Inbox`, `Contact`, `ContactInbox`, `Conversation`, `Message`) but enforce strict `tenant_id` on every table for PostgreSQL Row-Level Security.
    - **Extensible Channels**: Use a trait/adapter pattern in Rust for channels (`EmailChannel`, `InstagramChannel`, `WhatsAppChannel`, `WebWidgetChannel`) so new integrations can be cleanly added.
    - **Realtime**: Use standard WebSockets (or our existing sync mechanisms) to stream new `Message` and `AiDraft` updates to the frontend.

  **Implementation Prompt**
  Implement the core database schema, SeaORM entities, and Rust service layer for the native OHC omnichannel inbox. The goal is 100% feature parity with Chatwoot's core messaging model.
  1. Define SeaORM entities with strict `tenant_id` isolation for: `Inbox`, `ChannelAdapter`, `Contact`, `ContactInbox`, `Conversation`, `Message`, and `AiDraft`.
  2. Implement a `ConversationService` in Rust that handles incoming messages (upserting contacts, creating conversations if they don't exist, and appending messages).
  3. Emit an internal event (e.g., using PostgreSQL NOTIFY or Redlock-coordinated queue) when a new message arrives so AI departments can process it and generate an `AiDraft`.
  4. Ensure 100% unit test coverage for the service logic. E2E Playwright test: Simulating an incoming webhook from "Instagram", verifying it appears in the 375px mobile inbox view, and confirming the AI draft is generated and visible.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
