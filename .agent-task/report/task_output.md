issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema Design"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is retiring external integrations with third-party inbox providers like Chatwoot to build a fully native, high-performance omnichannel inbox in Rust (`onehumancorp/mono`). Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels (Instagram DMs, WhatsApp, SMS, email). We need a unified backend architecture capable of mapping diverse channel identities to a single customer graph, handling multi-tenant realtime conversational events, and natively routing messages to our internal AI agent (The Ambassador).

  # Research Report
  **Findings & Chatwoot Source Code Audit:**
  - Audited `github.com/chatwoot/chatwoot` models (`conversation.rb`, `message.rb`, `inbox.rb`, `channel/*`, `contact.rb`).
  - Chatwoot utilizes polymorphic channels (`channel_type` + `channel_id`) mapped to a unified `Inbox`. Messages belong to `Conversations` which belong to `Inboxes` and `Contacts`.
  - Chatwoot relies heavily on background jobs (Sidekiq) for event dispatches (webhooks, Slack notifications) and ActionCable for real-time WebSocket updates to the frontend.
  - **OHC Implementation Need:** A port of this robust relational model into our PostgreSQL + Row-Level Security (RLS) backend via Rust, replacing Active Record with SQLx. Crucially, our schema must enforce `tenant_id` at every level and integrate tightly with our Event Mesh for AI drafting, rather than just acting as a passthrough for human replies.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : tracks
      INBOX }o--|| CHANNEL_ADAPTER : configured_by
      CONTACT ||--o{ CONVERSATION : initiates
      CONTACT }o--|| TENANT : belongs_to
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes

      INBOX {
          uuid id
          uuid tenant_id
          string name
          enum channel_type
          jsonb channel_credentials
      }

      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
          string avatar_url
      }

      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          enum status "open | resolved | pending | snoozed"
          uuid assignee_id "Optional human or AI agent ID"
      }

      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          uuid sender_id "Polymorphic: Contact or Agent/User"
          enum sender_type
          text content
          enum message_type "incoming | outgoing | internal_note"
          boolean is_ai_draft "True if awaiting owner approval"
      }
  ```

  ### Mobile UX Flow (375px First)
  *This task is backend schema focused, but serves the following UI:*
  - **Home Feed:** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view showing the `Conversation`.
  - **Action:** A prominent primary button "Send Draft" (updates `Message.is_ai_draft` to false and triggers dispatcher) and a secondary "Edit".

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Listens to `message_created` events where `message_type = incoming`. Synthesizes a response and inserts a `MESSAGE` with `is_ai_draft = true`.
  - **Data Isolation:** All operations MUST enforce `tenant_id` via Postgres RLS.

  ### Key Design Decisions
  - **Polymorphic Senders:** `MESSAGE` table needs to distinguish between messages sent by the `Contact`, an AI `Agent`, or a human `User`.
  - **AI First:** The `is_ai_draft` flag on the `MESSAGE` table is a native first-class citizen, differentiating OHC from Chatwoot which treats AI as an external integration.
  - **Tenant Strictness:** Every table must have a `tenant_id` with an RLS policy applied. No exceptions.

  # Implementation Prompt
  **User-Facing Outcome:** The foundational database schema and Rust SQLx entities are in place to support the unified inbox. When a webhook hits the API in the future, it can be routed into these structured tables securely.

  **CUJ & Acceptance Criteria:**
  1. Define the SQL migrations (up and down) for `inboxes`, `contacts`, `conversations`, and `messages`.
  2. Implement Row-Level Security (RLS) on all new tables ensuring `tenant_id` isolation.
  3. Generate the corresponding Rust structs and SQLx CRUD operations inside the `server/db` module.
  4. Implement automated unit tests in Rust verifying that CRUD operations succeed and that RLS prevents tenant cross-talk (e.g., trying to read a conversation belonging to `tenant_A` using `tenant_B`'s context fails).
  5. Ensure `bazel test //...` passes completely.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
