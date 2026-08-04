issue_title: "Architecture Design: Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot as an external dependency with a native, high-performance omnichannel chat system in Rust. The current external dependency adds latency, complicates multi-tenant isolation, and breaks the Zero Trust / SPIFFE/SPIRE architecture by forcing data out of the core system. Non-technical owners like Maya and Carlos need their DMs, emails, and SMS messages seamlessly integrated into their single OHC work feed without managing a separate inbox product or configuring API keys in a third-party portal.

  ## Research Report
  Based on an audit of the `chatwoot/app/models` codebase, Chatwoot's core architecture revolves around the following entities:
  - `Account` (maps to OHC Tenant)
  - `Inbox` (Container for Channels)
  - `Channel::*` (Adapters for Facebook, Twitter, Email, API, SMS, Line, WhatsApp, Telegram, etc.)
  - `Conversation` (Thread within an Inbox for a Contact)
  - `Message` (Individual items in a Conversation)
  - `Contact` (The end customer)

  In OHC, we have started creating equivalent Rust domain models (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages` in `1009_native_omnichannel_chat.sql`). Our system must replicate the multi-channel adapters, SLA policies, and real-time WebSocket capabilities, but with stricter `tenant_id` Row Level Security (RLS) enforcement and integration with OHC's internal AI agent framework instead of third-party NLP pipelines.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ Channel : "has"
      Inbox ||--o{ Conversation : "contains"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "has"
      Message ||--o| AiDraft : "triggers"
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed**: Instead of a traditional Chatwoot inbox, messages appear as tasks in the primary owner feed, minimizing context switching.
  - **Translucent Glass Cards**: Each message card uses macOS-style translucent materials, showing the channel icon (e.g., Instagram, Email) and a snippet.
  - **Action Sheet**: Tapping a conversation opens a bottom sheet (native feel) with the chat thread, AI suggested replies (AiDrafts), and quick actions (Send Quote, Book Appointment).
  - **Offline Resilience**: Reads are cached via PWA/local storage. Pending messages queue in the background.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to new `Message` inserts via Postgres trigger / logical replication or Redis PubSub. Generates `AiDraft` records automatically based on business context.
  - **Work Triage**: Evaluates conversation priority and SLA, moving high-priority items to the top of the owner's feed.

  ## Implementation Prompt
  **User Facing Outcome:** An owner can connect their Instagram and Email in OHC settings. When a customer messages them, it appears instantly in their OHC Work Feed, complete with an AI-drafted reply. The owner can tap "Approve" to send the reply natively.

  **CUJ / Acceptance Criteria:**
  1. Complete the Rust repository layer for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages` using `sqlx`.
  2. Implement an API route to ingest webhooks from an external channel (e.g., an abstract API channel adapter) and create a `Message`.
  3. Ensure 100% `tenant_id` isolation in all DB queries using the newly defined `1009_native_omnichannel_chat.sql` schema and RLS policies.
  4. Ensure a UI screen can list conversations and messages with ZERO mock data.
  5. 100% unit and Playwright E2E test coverage for the chat inbox journey.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
