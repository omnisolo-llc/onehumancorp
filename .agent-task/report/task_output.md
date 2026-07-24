issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  We are fully retiring Chatwoot as our external service dependency and need a native, multi-tenant Rust omnichannel support & chat engine built directly into the OHC platform. Currently, without Chatwoot, owners lose unified inbox capabilities, and we lack a central hub for SMS, WhatsApp, Web, and Email customer interactions with native AI agentic orchestration. OHC owners need an integrated inbox where all communications funnel seamlessly so AI triage, follow-ups, and background agent tasks can run naturally without syncing latency.

  ## Research Report
  - **Source Analysed:** The Chatwoot Ruby on Rails repository (`https://github.com/chatwoot/chatwoot`).
  - **Findings:**
    - Chatwoot utilizes a strong model of `Conversations`, `Messages`, `Contacts`, `Inboxes`, and `ChannelAdapters`.
    - The core relies on `additional_attributes`, `custom_attributes` (JSONB) for extensibility and schema-less flexibility.
    - WebSockets drive real-time web-widget rendering.
    - AI integration in Chatwoot is mostly via webhooks/API, which isn't tight enough for our autonomous agent orchestration (Work Triage, Operations Agent, etc.).
    - Competitor systems like Front, Intercom, or Zendesk require massive context-switching. OHC’s native unified inbox will weave commerce, CRM, and AI directly into the message feed.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Inbox ||--o{ Channel : "has"
      Tenant ||--o{ Contact : "owns"
      Contact ||--o{ Conversation : "starts"
      Inbox ||--o{ Conversation : "routes"
      Conversation ||--o{ Message : "contains"
      Message ||--o{ Attachment : "has"
      Conversation ||--o{ AgentAssignment : "has"
  ```

  ### Data Model & Invariants
  We need to establish these core entities in PostgreSQL/Rust:
  1.  **Tenant/Account:** Row-level tenant isolation via `tenant_id` on all tables.
  2.  **Inbox:** A logical grouping for incoming messages (e.g., "Support", "Sales", "General").
  3.  **Channel (Trait/Enum):** Adapters for `WebWidget`, `Email`, `TwilioSMS`, `WhatsApp`, etc.
  4.  **Contact:** The customer entity across channels.
  5.  **Conversation:** The stateful session tying a Contact, Inbox, and Channel.
  6.  **Message:** The core payload (text, rich media, templates).

  ### AI Agent Integration
  - **Work Triage:** Every incoming message triggers a PostgreSQL background job (`SKIP LOCKED`). The triage agent determines intent, updates the `Conversation` status, and drafts an initial reply.
  - **Customer Assistant:** Observes the `Contact` history to summarize and prep suggestions.

  ### Mobile UX Flow (375px First)
  - **Screen 1 (Inbox List):** Unified list of active conversations. Unread badges, snippet previews. Uses UniFi/macOS translucent glass header.
  - **Screen 2 (Chat Thread):** WhatsApp-style thread. AI draft suggestions float above the keyboard. Quick-action chips (e.g., "Send Invoice", "Book Appt") inline. Native bottom input with attachment sheet.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  **User Persona:** Carlos (Handyman) receives texts (SMS) and Web Widget inquiries on his Android phone. He needs one inbox to reply, with AI drafting the quotes based on his past work.

  **Acceptance Criteria:**
  1. Build the Rust `ohc_chat` microservice/crate implementing `Inbox`, `Conversation`, `Message`, and `Contact` data models with strict tenant isolation.
  2. Implement an Actix-web / Axum WebSocket handler for real-time `Message` delivery to the Flutter frontend.
  3. Create the Channel adapter trait structure to support mock `WebWidget` and `SMS` injection.
  4. Build the Flutter UI screens (Inbox List & Chat Thread) following 375px responsive design and translucent premium tokens.
  5. Wire up the AI Work Triage agent to auto-draft a reply on the first message of a new `Conversation`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-retirement]
assignees: []
