issue_title: "Native Rust Omnichannel Chat System - Core Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System - Core Architecture

  ## Problem Statement
  We are retiring Chatwoot as an external dependency to build our own high-performance native omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. We need a core database schema and Rust architectural foundation capable of multi-tenant omnichannel conversations, unified inboxes, messages, and contacts that perfectly matches or exceeds Chatwoot's capabilities.

  ## Research Report
  - Audited Chatwoot source code (`https://github.com/chatwoot/chatwoot`) focusing on core domain entities: Inbox, Conversation, Message, Contact.
  - Chatwoot architecture features heavily on `account_id` (our `tenant_id`) and polymorphic associations for different channel types (Email, SMS, WebWidget, etc).
  - To implement in Rust + PostgreSQL, we will use our standard SaaS pattern: strict Row-Level Security (RLS) on all tables tied to `tenant_id`.
  - Audited `1009_native_omnichannel_chat.sql` to identify how migration structures have been laid out, ensuring robust coverage of all required schemas.

  ## Design Doc
  - **Data Model:**
    - `omni_inboxes` (tenant_id, id, name, channel_type, auto_assignment_config)
    - `omni_contacts` (tenant_id, id, name, email, phone_number, identifier)
    - `omni_conversations` (tenant_id, id, inbox_id, contact_id, status, assignee_id, unread_count)
    - `omni_messages` (tenant_id, id, conversation_id, inbox_id, sender_type, sender_id, content, content_type)
  - **Multi-Tenancy:**
    - Every table must have `tenant_id` and strict RLS policies.
  - **Architecture Diagram:**
    ```mermaid
    erDiagram
        TENANT ||--o{ OMNI_INBOX : has
        TENANT ||--o{ OMNI_CONTACT : has
        TENANT ||--o{ OMNI_CONVERSATION : has
        TENANT ||--o{ OMNI_MESSAGE : has
        OMNI_INBOX ||--o{ OMNI_CONVERSATION : receives
        OMNI_CONTACT ||--o{ OMNI_CONVERSATION : initiates
        OMNI_CONVERSATION ||--o{ OMNI_MESSAGE : contains
    ```
  - **Mobile UX Flow:**
    - Inbox layout must adapt to 375px: A master-detail view where the conversation list takes full width, and clicking a conversation slides in the chat view.
  - **AI Agent Integration Points:**
    - Customer Support Department: Agents hook into `omni_messages` creation via database triggers or pub/sub to automatically draft responses, categorizing tickets based on incoming message context.
    - Sales & Operations Departments: Agents monitor `omni_conversations` state changes (e.g., status updates) to generate follow-up tasks and sync context across other operations.

  ## Implementation Prompt
  Implement the core Rust domain models, controllers, and repository traits for the native omnichannel chat system.
  1. Define Rust structs for `Inbox`, `Contact`, `Conversation`, `Message` in `src/server/ohc/domain/omnichannel.rs`.
  2. Implement strict multi-tenant Row Level Security in Postgres migrations.
  3. Write comprehensive E2E tests validating that messages sent are properly isolated per tenant.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
