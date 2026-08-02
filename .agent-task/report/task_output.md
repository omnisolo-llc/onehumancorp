issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**:
  OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust. This system must replace the legacy Chatwoot integration and provide seamless support for various communication channels (web chat, email, SMS) with strict multi-tenant isolation.

  **Research Report**:
  Following the OHC Engineering Standards, the Chatwoot integration is being retired. We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture:
  - `Inbox`: The central hub for a specific communication channel within an account.
  - `Conversation`: Represents a thread of messages between a customer and agents within an Inbox.
  - `Message`: Individual communication items within a Conversation.
  - `Channel`: The specific medium (e.g., Email, SMS, Web Widget).

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
      INBOXES ||--o{ CONVERSATIONS : contains
      CONVERSATIONS ||--o{ MESSAGES : has
      TENANTS ||--o{ INBOXES : owns
      TENANTS ||--o{ CONVERSATIONS : owns
      TENANTS ||--o{ MESSAGES : owns
    ```
  - **Architecture**: A set of Rust microservices within `onehumancorp/mono` handling:
    - **API Layer**: gRPC/REST endpoints for managing inboxes, conversations, and messages.
    - **WebSocket Server**: Real-time message delivery for the web chat widget and agent dashboard.
    - **Worker Queue**: Background processing for external channel integrations (sending emails/SMS).
  - **Data Model (PostgreSQL)**:
    - `inboxes`: `tenant_id`, `id`, `name`, `channel_type`, `config (JSONB)`
    - `conversations`: `tenant_id`, `id`, `inbox_id`, `status`, `customer_id`
    - `messages`: `tenant_id`, `id`, `conversation_id`, `sender_type`, `content`
    *Note: All tables must include `tenant_id` for row-level security.*
  - **Mobile UX Flow (375px first)**:
    - A consolidated message center screen listing all `conversations` sorted by recent activity.
    - Tapping a conversation opens a chat interface where the user can reply, add internal notes, or assign to an AI agent.
    - Real-time updates push new messages into the UI using WebSocket without horizontal scrolling.
  - **AI Agent Integration**: Agents will monitor `conversations` for specific triggers (e.g., new message in a "sales" inbox) and can inject `messages` representing AI replies or internal notes.

  **Implementation Prompt**:
  Implement the foundational Rust data models and database migrations for the Omnichannel Chat System. Create the PostgreSQL schema for `inboxes`, `conversations`, and `messages`, ensuring `tenant_id` is present on all tables for RLS. Implement the corresponding Rust structs in the shared models crate and basic CRUD operations in the database layer.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
