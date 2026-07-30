issue_title: "Native Rust Omnichannel Chat: Core Models & Multi-tenant Inbox Architecture"
issue_description: |
  # Native Rust Omnichannel Chat: Core Models & Multi-tenant Inbox Architecture

  ## Problem Statement
  OneHumanCorp currently relies on an external system (Chatwoot) for omnichannel messaging. We are completely retiring Chatwoot in favor of a high-performance, native Rust omnichannel chat system within `onehumancorp/mono`. We need a scalable, strict multi-tenant architecture designed to support small business owners (Maya the baker, Carlos the handyman) tracking customer inquiries across multiple channels (Instagram, SMS, Web Widget, Email) in a unified inbox without any complex technical overhead.

  ## Research Report
  - **Market:** Platforms like Shopify Inbox and Wix Chat consolidate messages but often lock you into their ecosystem. We are building a unified, fast, API-first inbox core.
  - **Source Code Audit (Chatwoot):** An audit of `chatwoot/app/models` shows a robust, though traditional Rails implementation:
    - `Inbox`: Aggregates conversations. Belongs to an `Account` (tenant).
    - `Conversation`: The core thread of messages. Links a `Contact`, an `Inbox`, and an `Assignee`.
    - `Message`: The individual chat item, linked to a `Conversation`.
    - `Channel::*` (e.g. `api`, `web_widget`, `facebook_page`, `sms`): Adapters that define how incoming webhooks/messages are parsed.
  - **Gaps:** Chatwoot uses global tables with a scoped `account_id`. We need to use row-level security (RLS) and strict `tenant_id` constraints in Postgres, paired with high-performance Rust to handle webhook ingestion efficiently and coordinate with AI assistants (Operations/Sales/Support agents).

  ## Design Doc
  - **Architecture Diagram (Mental Model / Mermaid):**
    ```mermaid
    erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ Channel : "has one"
      Inbox ||--o{ Conversation : "has many"
      Contact ||--o{ Conversation : "has many"
      Conversation ||--o{ Message : "has many"
      Tenant {
        uuid tenant_id
        string name
      }
      Inbox {
        uuid id
        uuid tenant_id
        string name
        uuid channel_id
      }
      Conversation {
        uuid id
        uuid tenant_id
        uuid inbox_id
        uuid contact_id
        string status
      }
      Message {
        uuid id
        uuid tenant_id
        uuid conversation_id
        string content
        string sender_type
      }
    ```
  - **Mobile UX Flow (375px First):**
    - The Owner opens the app and sees a unified "Triage Feed" (Inbox list).
    - Tapping a thread opens the Chat UI (translucent macOS style headers, UniFi layout for cards).
    - The user can tap a "Draft Reply" button which triggers the AI Agent to draft a response contextually.
    - No configuration forms are visible unless the user explicitly navigates to an "Advanced Settings" section.

  - **AI Agent Integration Points:**
    - AI workers listen for new `Message` events (via PostgreSQL `SKIP LOCKED` or Redis pub/sub queue).
    - Based on channel & message context, the agent evaluates the customer intent (e.g., cake inquiry, scheduling request).
    - The agent drafts a response (adding a `Message` with `status="draft"`) or automatically creates an action item/task for the owner.

  - **Key Design Decisions:**
    - **Rust for Core Inbox API:** Handle webhook processing from WhatsApp/Instagram and socket connections extremely fast.
    - **PostgreSQL RLS:** Strict `tenant_id` on every table. All DB access must pass the tenant context.
    - **Polymorphic Sender / Channels:** Design the system to handle multiple inbound channel types gracefully without hardcoding logic in the core inbox routing.
    - **API First:** The Flutter/PWA frontend will rely solely on these new Rust APIs and WebSockets.

  ## Implementation Prompt
  Implement the foundation for the Native Rust Omnichannel Chat System.
  1. Define the core data schemas for `Inbox`, `Conversation`, `Message`, and `ChannelAdapter` with strict `tenant_id` fields. Ensure Row-Level Security (RLS) is enabled and migrations are written.
  2. Implement the Rust data access layer (repositories) to fetch, create, and list these entities. Ensure tenant isolation.
  3. Create the foundational gRPC / REST API endpoints for the Frontend to query Inboxes and Conversations.
  4. Ensure all database interactions utilize the correct tenant context.
  5. Include full unit tests for the core logic and repository methods.

  **Acceptance Criteria:**
  - Migrations for the core chat entities exist and run cleanly.
  - Rust API endpoints to list and create Inboxes, Conversations, and Messages exist.
  - API enforces tenant isolation (fetching data for Tenant A returns zero records for Tenant B).
  - Code compiles, and all tests pass with 100% coverage on the new code.
  - Follow the OHC strict requirements: no mock data in UI/tests, real DB, complete E2E functionality in mind.

  ## Priority: P0 (Critical - Blocks all chat capabilities)
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
