issue_title: "Native Rust Omnichannel Chat: Data Models & Inbox Architecture"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Chatwoot has been 100% RETIRED as an external dependency. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. Currently, the system lacks the core data models, multi-tenant isolation, and inbox architecture required to replace Chatwoot's functionality natively. This prevents owners from managing communications efficiently from a single, unified view, leading to missed opportunities.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** We have audited the `chatwoot/chatwoot` source code. Chatwoot heavily utilizes a relational model grouping `Conversations`, `Messages`, `Contacts`, and `Inboxes`. Inboxes tie to specific `Channel` adapters (e.g., `Channel::Email`, `Channel::Whatsapp`).
  - **Shopify Inbox & Wix Inbox:** Provide basic aggregation but lack deep, native AI-driven contextual awareness that OHC's "Ambassador" and "Manager" agents will provide.
  - **OHC Native Requirement:** OHC needs native Rust microservices and data models (PostgreSQL) that replicate and enhance Chatwoot's core entities. This allows for deep integration with OHC's AI event mesh, customer identity graph, and tenant isolation (RLS).
  - **Multi-Tenancy:** Chatwoot uses `account_id` for scoping. OHC uses `tenant_id` with strict PostgreSQL Row Level Security (RLS).

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|| CHANNEL_ADAPTER : configured_with
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Inbox List:** A clean, Unifi-style translucent list of active conversations. Each row shows the customer name, latest message snippet, time, and a channel icon (e.g., Instagram, WhatsApp).
  - **Conversation View:** Standard chat interface. Messages grouped by date. Clear indication of AI-drafted messages pending approval.
  - **UX Flow:** Maya opens the app -> sees 3 unread messages in the unified Inbox -> taps one -> reads the context -> taps 'Approve AI Draft' -> message is dispatched natively.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the event mesh for new `Message` creation events. Uses the `Conversation` and `Contact` context to generate drafts.
  - **Zero Trust & Security:** All queries to the Inbox/Conversation tables enforce `tenant_id` isolation at the database level via RLS.

  ### Key Design Decisions
  - **Native Rust Implementation:** Eliminates the operational overhead of managing a separate Chatwoot deployment (Ruby/Rails).
  - **Unified Event Mesh:** Natively integrating the chat system allows immediate triggers for AI agents without external webhooks.
  - **Strict RLS:** Ensures complete data isolation between OHC tenants.

  # Implementation Prompt
  **User-Facing Outcome:** The backend foundation is ready for the new native Unified Inbox. The necessary data models (Inbox, Conversation, Message, Contact) exist with strict multi-tenant isolation, preparing the system for channel integrations and UI development.

  **CUJ & Acceptance Criteria:**
  1. Define Rust structs and PostgreSQL schemas (with RLS policies) for `Inbox`, `Conversation`, `Message`, and `Contact`.
  2. Implement a Rust service layer (e.g., `InboxService`, `ConversationService`) to handle CRUD operations, ensuring `tenant_id` is always enforced.
  3. Provide comprehensive unit tests verifying that a user in `Tenant A` cannot access `Conversations` or `Messages` belonging to `Tenant B`.
  4. Integrate these new models into the Bazel build system and ensure all tests pass.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
