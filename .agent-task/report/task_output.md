issue_title: "[Platform] Implement Native Rust Omnichannel Chat Inbox Foundation"
issue_description: |
  ## Problem Statement
  We have retired the third-party Chatwoot service dependency to bring our core communication capabilities native. For our owner personas (Maya the Baker, Carlos the Handyman), they rely on multiple channels—Instagram DMs, SMS, Email, and Web Widget—to speak with customers. The platform currently lacks a native, multi-tenant Rust core for managing an omnichannel unified inbox, including Conversations, Messages, and Channels. This creates a critical architectural gap where we cannot process incoming leads or route agent replies without relying on the now-retired Ruby system.

  ## Research Report
  - **Source benchmark:** Audited Chatwoot's `Inbox`, `Conversation`, and `Message` models.
  - **Findings:** The models depend on polymorphic `channel` associations, threaded conversational models with `waiting_since` and `snoozed_until` states, and rich JSONB attributes for multi-channel metadata.
  - **Gap:** OHC lacks corresponding Rust models, Diesel schemas, and `ohc-server` endpoints to accept webhook data and route it to an internal unified database.

  ## Design Doc

  ### Architecture
  We will introduce three foundational domains in the native Rust backend:
  1. **Inboxes**: The entry points (e.g. Maya's Instagram DM inbox).
  2. **Conversations**: The ongoing thread between a Contact and the Business.
  3. **Messages**: The individual events within a conversation.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : has
      CONTACT ||--o{ CONVERSATION : participates_in
  ```

  ### Mobile UX Flow
  - Users will open the OHC app, tap "Inbox", and see a unified list of `Conversations`.
  - Tapping a Conversation reveals the thread of `Messages`.
  - Native performance enables instant transitions between the list and detail views without network jank.

  ### AI Agent Integration
  - These tables must support an `assignee_agent_id` or similar construct so background AI workers can pick up "pending" conversations, generate a reply, and post it to the `Messages` table using the generic API.

  ### Key Decisions
  - Enforce strict row-level security / multi-tenant isolation by requiring `tenant_id` on all tables.
  - Use Postgres JSONB columns for flexible `channel_metadata` rather than polymorphic polymorphic table explosions initially.

  ## Implementation Prompt
  **Outcome:** Implement the core database schema migrations and Rust data models (Diesel structs) for `inboxes`, `conversations`, and `messages` within the OHC server.
  **CUJ:** As an engineer, I can create an Inbox, start a Conversation, and add Messages to it via Rust tests, ensuring all records are properly isolated by tenant.
  **Acceptance Criteria:**
  - Database migrations for `inboxes`, `conversations`, and `messages` exist with `tenant_id` and RLS enabled.
  - Rust models (Diesel/SQLx or whatever the repo uses) are implemented.
  - Unit tests demonstrate basic CRUD operations per tenant.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
