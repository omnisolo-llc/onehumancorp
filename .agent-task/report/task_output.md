issue_title: "Native Chat: Implement Rust-based Omnichannel Conversation Engine (Phase 1: Core Models & Migrations)"
issue_description: |
  ## Mission Queue Protocol: Chatwoot Retirement & Native Rust Omnichannel Chat System

  ### Problem Statement
  OHC is retiring the external third-party Chatwoot dependency. A core promise of OHC is to consolidate all customer communication (DMs, emails, webchat) into one unified triage inbox for the owner/operator, without relying on external monolithic services. To do this securely and performantly, we must build a native, high-performance, multi-tenant omnichannel chat engine inside `onehumancorp/mono` using Rust.

  ### Research Report
  Based on an exhaustive audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core data model revolves around Accounts (Tenants), Inboxes (Channels), Conversations (Threads), and Messages.

  **Key Learnings from Chatwoot:**
  - Conversations belong to an Inbox and a Contact, but an Inbox belongs to an Account. This implies tenant isolation (`tenant_id`) is essential at every table to support OHC's Row Level Security (RLS) PostgreSQL architecture.
  - Messages have rich types (incoming, outgoing, template) and statuses (sent, delivered, read).
  - Contacts are shared across an Account, allowing a single customer profile to aggregate conversations from multiple Inboxes (e.g., WhatsApp, Email, Web Widget).

  **Why Native Rust?**
  - **Performance:** Rust microservices will handle thousands of concurrent WebSocket connections and webhook events with significantly lower memory footprint than Chatwoot's Ruby on Rails architecture.
  - **Tenant Isolation:** We can natively enforce OHC's `tenant_id` RLS directly in the database schemas and ORM (SeaORM/Diesel).
  - **Integration:** Deep integration with OHC's existing AI Triage, Customer Relationship, and Operations assistants without cross-service API overhead.

  ### Design Doc
  This issue represents Phase 1: Core Data Models & Migrations for the native Rust chat engine.

  **Data Model Outline (to be implemented with RLS `tenant_id`):**
  1.  `chat_inboxes`: Represents a channel (e.g., "Main Website Chat", "Support Email").
  2.  `chat_contacts`: Represents the customer interacting via the inbox.
  3.  `chat_conversations`: Represents a thread between a Contact and the Inbox.
  4.  `chat_messages`: Individual messages within a Conversation.

  **Integration Points:**
  - PostgreSQL with Row Level Security (`tenant_id`).
  - Rust backend service (using the designated OHC web framework and ORM).
  - This phase focuses purely on database migrations and Rust entity generation/setup. No frontend or API endpoints in this phase.

  ### Implementation Prompt
  1.  **Database Migrations:** Create PostgreSQL migrations for the core chat entities (`chat_inboxes`, `chat_contacts`, `chat_conversations`, `chat_messages`). Ensure every table includes a `tenant_id` UUID column and is configured for Row Level Security (RLS) according to OHC standards. Define appropriate indexes (e.g., on `tenant_id`, `conversation_id`, `contact_id`).
  2.  **Rust Entities:** Generate or implement the corresponding Rust ORM entities (models) for these tables. Ensure relationships are correctly defined (e.g., a Message belongs to a Conversation).
  3.  **Repository/DAO Layer:** Create a basic repository or data access layer in Rust with methods for creating and querying these entities, explicitly enforcing tenant context (e.g., `find_conversations_by_tenant(tenant_id)`).
  4.  **Testing:** Write comprehensive unit and integration tests in Rust for the repository layer. Tests must verify that RLS/tenant isolation works correctly (e.g., querying for tenant A does not return records for tenant B). Use the standard OHC testing framework.

  **Acceptance Criteria:**
  - Migrations run cleanly on an empty database.
  - Rust models accurately reflect the schema.
  - Repository methods perform basic CRUD operations.
  - 100% unit test coverage for the new Rust repository code.
  - `bazel test //...` passes.

  ### Priority
  P0

  ### Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
