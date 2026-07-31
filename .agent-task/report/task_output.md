issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: Native Rust Omnichannel Chat System

  ## Problem Statement
  OHC currently relies on external Chatwoot services for omnichannel customer support, which violates our core mandate of a single cohesive platform without external dependencies for core capabilities. Relying on Chatwoot limits our control over the multi-tenant database, forces non-native integration paths, and impacts performance due to language mismatch (Ruby vs. Rust) and the complexity of maintaining two separate systems. The user needs a seamlessly integrated messaging inbox natively within OHC.

  ## Research Report
  - We performed a codebase audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), specifically focusing on `app/models/` to understand their domain logic:
    - **Conversations**: Represent threads of communication, associated with accounts, contacts, and inboxes. Contains `additional_attributes` (JSONB).
    - **Messages**: Individual pieces of communication within a conversation. Types (incoming, outgoing, activity, template). `content_attributes` (JSON) and `additional_attributes`.
    - **Contacts**: Represents customers with custom attributes.
    - **Inboxes**: Define channels (web widget, API, email, etc.) and assignment policies.

  - The existing `src/proto/inbox.proto` has basic definitions (`OmniMessage`, `Conversation`) but lacks the depth to replicate Chatwoot's extensive feature set like assigning agents, snoozing, channel types, custom attributes, and SLA policies. We need to bridge this gap with robust Rust structures.

  ## Design Doc
  - **Architecture Diagram**:
    - `Client/Mobile UI (375px first)` -> `Rust API Server` -> `PostgreSQL (Multi-tenant schema with Row Level Security)`
    - `Rust API Server` <-(WebSockets)-> `Client/Mobile UI (Live updates)`
  - **Data Model (Rust/Postgres)**:
    - Use strict `tenant_id` for isolation.
    - `conversations` table: `id`, `tenant_id`, `contact_id`, `inbox_id`, `assignee_id`, `status` (open, resolved, snoozed), `snoozed_until`.
    - `messages` table: `id`, `tenant_id`, `conversation_id`, `content`, `message_type` (incoming, outgoing), `content_type` (text, image), `status` (sent, delivered, read).
    - `contacts` table: `id`, `tenant_id`, `name`, `email`, `phone_number`.
  - **Mobile UX Flow (375px first)**:
    - Unified Inbox view showing active conversations, sorted by last activity.
    - Conversation view for a specific chat with sticky input field and smart replies.
    - Smooth real-time update handling via WebSockets.
  - **AI Agent Integration**:
    - AI acts as the first responder or "Customer Assistant" drafting replies (`draft_reply` already in proto) based on context, reducing the manual burden on the owner.

  ## Implementation Prompt
  - Create the foundational Rust API endpoints and PostgreSQL database schemas to fully replace Chatwoot's core messaging flow.
  - Implement the `Conversation` and `Message` entities ensuring strict `tenant_id` based multi-tenancy.
  - Define gRPC/REST endpoints for creating and fetching conversations and messages.
  - *Acceptance Criteria*:
    - The endpoints handle multi-tenant data correctly.
    - Unit tests cover 100% of the new Rust backend logic.
    - E2E Playwright test verifies a user can load the inbox and see a populated list of conversations and messages (simulating the UI using the real backend).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
