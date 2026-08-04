issue_title: "Native Rust Omnichannel Chat: Inbox, Conversation, Message, and Channel Models"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is currently lacking a native omnichannel chat system. The original plan was to integrate with Chatwoot, but as per the new Engineering Standards, Chatwoot as an external service is 100% RETIRED. OHC must build its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust.

  Small business owners need to manage inquiries across multiple channels (Instagram, WhatsApp, Email, Web Widget) without manual aggregation. A unified inbox with AI "Ambassador" agent integration is critical for drafting contextual responses based on full customer history.

  # Research Report
  - **Codebase & Docs Audit:** The OHC repository needs a complete omnichannel chat backend in Rust. We need to implement the core domain models.
  - **Chatwoot Source Code Audit:** I have cloned and analyzed the Chatwoot source code (`app/models`). The core architecture revolves around `Account` (Tenant), `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact`.
    - **Inboxes** belong to Accounts and have a `Channel` (polymorphic or one-to-one mapping).
    - **Conversations** belong to Inboxes, Contacts, and Accounts.
    - **Messages** belong to Conversations and Accounts.
    - **Contacts** belong to Accounts and represent the customer's identity.
    - **Channels** (e.g., `channel_whatsapp`, `channel_web_widget`) hold provider-specific configuration.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      TENANT ||--o{ MESSAGE : has
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|| CHANNEL : configured_via
  ```

  ## Mobile UX Flow (375px First)
  - **Inbox List:** Users see a unified list of conversations categorized by `Contact` and `Channel`. The layout uses UniFi modular dashboard cards.
  - **Conversation View:** Tapping a conversation opens a standard chat interface optimized for 375px without horizontal scrolling. Key action buttons (Approve AI Draft, Send, Edit) are easily accessible with a thumb.

  ## AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the incoming message event stream. When a new `Message` is created in a `Conversation`, The Ambassador queries the `Contact`'s history and drafts a suggested reply. The draft is stored with an `is_draft` flag and presented to the owner for approval.

  ## Key Design Decisions
  - **Native Rust Implementation:** We will implement this in `src/server/integrations/chat` (or appropriate core service directory) using Rust (or Go if Rust is not the primary backend language, but instructions specify Rust for Chatwoot replacement).
  - **Row-Level Security:** All tables must include `tenant_id` for PostgreSQL RLS.
  - **Omnichannel Gateway:** The system will eventually accept webhooks from Instagram/WhatsApp, route them to an Inbox, resolve the Contact, and append Messages to Conversations.

  # Implementation Prompt
  **User-Facing Outcome:** Lay the foundation for a native omnichannel chat system so that OHC can eventually ingest messages from various sources into a unified inbox without relying on external Chatwoot.

  **CUJ & Acceptance Criteria:**
  1. Create the database schema (SQL migrations) for `inboxes`, `channels` (e.g., generic `channels` table with JSONB config or specific tables), `contacts`, `conversations`, and `messages`. Ensure `tenant_id` is present on all and RLS is enabled.
  2. Implement the core models and repository layer (CRUD operations) for these entities in the OHC backend (Go or Rust, following repo conventions).
  3. Write unit tests for the repository layer ensuring tenant isolation (a query for tenant A should not return tenant B's messages).
  4. (Optional) Provide a simple internal API to create an inbox and send a message to a conversation.
  5. Ensure `bazel test //...` passes 100%.

  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
