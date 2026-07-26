issue_title: "Native Rust Omnichannel Chat Architecture Design"
issue_description: |
  # Native Rust Omnichannel Chat Architecture Design

  ## Problem Statement
  We are fully retiring Chatwoot as an external dependency. We need a native Rust implementation of an omnichannel inbox, messaging, and chat system built into `onehumancorp/mono`. Non-technical operators (like Maya the baker, Carlos the handyman) need a unified place to triage customer inquiries (Instagram DMs, email, website chat widget) without needing to configure a complex third-party SaaS setup. They just need a clean, single-screen "Inbox" to handle all inbound demand and customer conversations efficiently, seamlessly integrating with our AI agents and backend systems.

  ## Research Report
  Based on the Chatwoot source code repository audit (`/tmp/chatwoot/app/models/`), a fully functional omnichannel system needs a core set of models and state machines:

  - **Inbox**: A channel configuration (e.g., an Instagram account, an email address, or a website widget) for a specific `tenant_id`.
  - **Conversation**: An ongoing thread of interaction between a contact (customer) and an inbox. Contains state (`open`, `snoozed`, `resolved`) and an assignee.
  - **Message**: Individual payloads within a conversation (text, images, system events, agent bot actions).
  - **Contact**: The customer communicating with the business (has a unified profile across channels).

  By implementing these natively in Rust inside `ohc-mono`, we get:
  - Strong tenant isolation natively enforced by our data layers.
  - No external sync lag.
  - Seamless AI department integration directly hooked into message creation events.
  - Shared identity/auth context.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
          jsonb channel_config
          boolean is_active
      }

      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }

      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          uuid assignee_id
          timestamp last_activity_at
      }

      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string message_type
          string content
          uuid sender_id
          string sender_type
      }
  ```

  ### Data Model & Invariants (Multi-Tenant Rust Implementation)
  - Create a new module `src/server/ohc/domain/inbox` and `src/server/ohc/domain/chat` in the Rust backend.
  - The persistence layer must guarantee `tenant_id` isolation. All `SELECT`, `UPDATE`, and `DELETE` operations MUST scope to the active `tenant_id`.
  - All entities use UUID v4 for primary keys.
  - `channel_type` inside `Inbox` defines whether it's `Email`, `WebsiteWidget`, `InstagramDM`, etc.
  - `status` inside `Conversation` is an enum: `Open`, `Snoozed`, `Resolved`.
  - `message_type` inside `Message` is an enum: `Incoming`, `Outgoing`, `SystemAction`.

  ### AI Agent Integration Points
  - **Triage Agent Hook**: When a new `Message` is created where `message_type = Incoming`, an event is emitted (via Tokio async channels or database trigger) to the AI Triage Agent. The Triage Agent analyzes the message and optionally drafts a reply or categorizes the conversation.
  - **Auto-responder**: The Operations Assistant can automatically transition `Conversation` status or reply if a known pattern is matched (e.g., "Are you open today?").

  ### Mobile UX Flow (375px First)
  - **Inbox List Screen**: Clean UniFi-style list of `Conversation` cards. Each card shows the Contact's avatar, name, an excerpt of the last message, and a timestamp. Badges indicate unread status.
  - **Conversation Detail Screen**: A standard chat interface. A sticky header showing Contact name and channel icon. Scrollable message history in the center. A text input field at the bottom with a primary "Send" button.
  - **Agent Action Panel**: A small, dismissible panel above the text input showing "AI Draft Available" if the Triage Agent has proposed a reply. The user can tap "Approve" to insert it into the text box.

  ## Implementation Prompt
  **Goal:** Implement the core Rust data models, database schemas (PostgreSQL migrations), and basic gRPC/REST service endpoints for the native Omnichannel Chat architecture in `src/server/ohc/domain/inbox` and `src/server/ohc/domain/chat`.

  **Acceptance Criteria:**
  1. Define Rust structs for `Inbox`, `Contact`, `Conversation`, and `Message` in `src/server/ohc/domain`.
  2. Write SQLx database migrations to create the corresponding tables, ensuring `tenant_id` is a strictly enforced foreign key and indexed appropriately.
  3. Implement repository functions (e.g., `create_inbox`, `list_conversations_for_inbox`, `add_message_to_conversation`) with strict `tenant_id` validation.
  4. 100% unit test coverage for the repository methods using the real database connection (test suite setup).
  5. The implementation must strictly follow the architectural constraints defined above, prioritizing multi-tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
