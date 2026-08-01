issue_title: "Implement Custom Rust Omnichannel Chat System to Replace External chat systems"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems or incomplete integrations for omnichannel inbox and conversational commerce. The market research and platform constraints explicitly mandate that external dependencies for this are 100% RETIRED. The core owner/operator personas (Maya the baker, Carlos the handyman) require a unified inbox (Omnichannel) directly embedded in their 375px mobile experience where they can see inquiries from Instagram DMs, WhatsApp, SMS, and Web Chat in one place, alongside automatic AI drafted replies from the Ambassador Agent. Without a native, high-performance, embedded Rust-based omnichannel chat system in OHC, we cannot provide the seamless "Zero-Touch Fallback" and context-aware business operations required.

  ## Research Report & Gap Analysis
  - **The Gap**: We lack a central, multi-tenant-safe Rust backend architecture for managing omnichannel communications natively. The existing `inbox.rs` is extremely basic and focuses heavily on webhook relay rather than managing conversations, contacts, and state (open/snoozed/resolved) the way a true unified inbox must.
  - **Competitor Audit (External)**: Analyzing external chat platforms models, their core primitive is the `Conversation` (linked to `Account`, `Inbox`, `Contact`, `ContactInbox`). A `Conversation` tracks `status`, `assignee`, and metrics. An `Inbox` ties to a `Channel`.
  - **The Solution**: We need to replicate this core omnichannel data model natively in Rust for OHC. We must implement `Conversation`, `Message`, `Inbox`, and `ContactInbox` structs and database schemas that support our `tenant_id` based multi-tenant isolation.
  - **Agent Integration**: The Ambassador agent will listen to new `Message` inserts in this new native system, draft replies, and place them in the `ActionRequiredQueue` (as described in the omnichannel market research).

  ## Design Doc
  ### Architecture Diagram (Mental Model)
  ```mermaid
  graph TD
    Client[Incoming Webhook/API] --> API_Gateway
    API_Gateway --> InboxService[Rust Native Inbox Service]
    InboxService --> DB[(PostgreSQL)]
    DB --> ConversationTable
    DB --> MessageTable
    DB --> InboxTable
    InboxService --> AmbassadorAgent[AI Ambassador Agent]
    AmbassadorAgent --> ActionRequiredQueue[Action Required Queue]
  ```

  ### Data Model (PostgreSQL + Rust Structs)
  - `omni_inboxes`: `id`, `tenant_id`, `name`, `channel_type` (e.g., 'instagram', 'whatsapp', 'sms', 'web').
  - `omni_conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `snoozed_until`, `last_activity_at`.
  - `omni_messages`: `id`, `tenant_id`, `conversation_id`, `content`, `message_type` (incoming, outgoing), `status` (sent, delivered, read, failed).

  ### Mobile UX Flow (375px)
  1. Owner opens app. The Unified Agent Feed shows a card: "3 new DMs waiting for reply."
  2. Tapping the card opens the Unified Inbox view.
  3. The Inbox view lists active `omni_conversations` across all channels, tagged with the channel icon (IG, WhatsApp).
  4. Selecting a conversation shows the chat history and a pre-drafted Ambassador response in a prominent "Approve & Send" bubble.

  ### Key Design Decisions
  - **Strict Row-Level Security / Multi-Tenancy**: Every table must have a `tenant_id` and strict `WHERE tenant_id = $1` checks in the repository layer.
  - **Native Rust**: No Ruby, no external services. We build the core inbox engine in `src/server/domain/repository/omnichannel_repo.rs` and `src/server/domain/inbox.rs`.

  ## Implementation Prompt
  **User-Facing Outcome:** The user has a fully functional, lightning-fast native unified inbox that tracks conversations from multiple channels.
  **CUJ & Acceptance Criteria:**
  1. Implement the database migrations for `omni_inboxes`, `omni_conversations`, and `omni_messages`.
  2. Implement the Rust data models and repository methods in `omni_inbox_repo.rs` or update `inbox.rs` with `create_inbox`, `create_conversation`, `create_message`, and `get_conversations_for_tenant`.
  3. Write comprehensive unit tests for the repository methods ensuring `tenant_id` isolation works perfectly.
  4. Integrate this new data model with the existing `handle_inbox_action` workflow, ensuring that new messages trigger the Ambassador draft logic properly (e.g., inserting into the Action Required Queue).
  5. Provide Playwright E2E tests validating that a mock webhook creates a conversation and message in the DB, and it surfaces in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
