issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System

  **Problem Statement**:
  Currently, we rely on Chatwoot as an external third-party service for omnichannel customer support (email, SMS, web chat, DMs). This introduces latency, breaks our Zero-Trust multi-tenant isolation guarantees, and forces owners (like Maya the baker and Carlos the handyman) into a disjointed experience when jumping between their operational dashboard and customer communication. To provide a true "one assistant" experience, we must bring the omnichannel chat capability directly into OneHumanCorp as a native, highly performant Rust module integrated with our PostgreSQL/Redis stack and AI work assistant feed.

  **Research Report**:
  Audited the core Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`). The omnichannel routing architecture pivots around a few key entities:
  1. `Contact`: The unified customer profile (`account_id`, `email`, `phone_number`, `identifier`).
  2. `Inbox`: A specific channel endpoint (e.g., a specific email address, web widget, or WhatsApp number) attached to an `account_id` and a `channel_type`/`channel_id`.
  3. `Conversation`: The threaded context between a `Contact` and an `Inbox`. Features SLA policies, assignment (`assignee_id`), priority, and snooze capabilities.
  4. `Message`: The individual payload (text, media, attachments) belonging to a `Conversation`. Differentiates between incoming, outgoing, and internal private notes (`private: boolean`, `message_type: integer`).
  By replicating these entities strictly within our `tenant_id` boundaries (Row Level Security in PostgreSQL), we can achieve full feature parity while eliminating the external Chatwoot dependency.

  **Design Doc**:
  *Architecture Diagram (Mermaid.js)*
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Tenant ||--o{ Conversation : owns
      Inbox ||--o{ Conversation : receives
      Contact ||--o{ Conversation : initiates
      Conversation ||--o{ Message : contains
      Message }o--o| AI_Agent : processed_by
  ```
  *Mobile UX Flow (375px)*
  1. The owner opens the app to the unified feed.
  2. A new unread item appears: "Maya, 3 new Instagram DMs about custom cakes."
  3. Tapping opens a translucent-glass chat view. The AI Agent has already drafted suggested replies.
  4. The owner taps "Approve" or types their own message. The chat interface is native Flutter, resilient to offline state.
  *AI Agent Integration*
  The new Rust chat engine will emit PostgreSQL `SKIP LOCKED` job queue events when a new `Message` arrives. The `Customer & Relationship Assistant` agent will dequeue these events, analyze context, and create `draft` messages on the `Conversation` without blocking the main event loop.

  **Implementation Prompt**:
  Implement the core database schema and Rust service layer for the native omnichannel chat system.
  1. Create the PostgreSQL migration establishing `inboxes`, `contacts`, `conversations`, and `messages` tables. You MUST include `tenant_id` on every table and enable Row Level Security (RLS) matching our multi-tenant SaaS standards.
  2. Implement the Rust data structures and Diesel/SQLx queries in `src/server/ohc/chat.rs` (or equivalent domain folder).
  3. Implement the internal API layer to create/read messages for a specific conversation.
  4. Ensure 100% unit test coverage for the new service methods.
  *Acceptance Criteria*: A test can programmatically create a Contact, an Inbox, start a Conversation, and add Messages to it while respecting Tenant boundaries.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
