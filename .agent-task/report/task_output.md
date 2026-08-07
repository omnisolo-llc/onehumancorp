issue_title: "Implement Chatwoot-Compatible Core Omnichannel Models in Rust"
issue_description: |
  # Problem Statement
  OneHumanCorp currently relies on third-party integrations or disjointed services for messaging, which fragments the owner's operational context. As a core mandate, Chatwoot must be retired and replaced by a native Rust omnichannel backend inside OHC that achieves feature parity for unified inboxes, omnichannel routing, and customer engagement. Owners need a unified view of Instagram DMs, WhatsApp, Emails, and SMS to efficiently capture demand and serve customers without switching contexts or tabs.

  # Research Report
  Based on our competitive analysis (`docs/business/market_research/ohc_owner_work_assistant_competitive_research.md` and `docs/business/market_research/omnichannel_unified_inbox.md`), legacy systems like Shopify Inbox and Wix aggregate messages but fail to provide true autonomous context. Chatwoot provides an excellent schema foundation (Contacts, Inboxes, Conversations, Messages, Channels) but is too heavy as an external Ruby/Postgres dependency.

  Benchmarking against Chatwoot's core data models (found in `/tmp/chatwoot/app/models/`):
  * **Inbox**: Represents a channel endpoint (e.g., a specific WhatsApp number or Instagram page) assigned to an account.
  * **Contact**: Represents the end-user (customer, visitor) with cross-channel identity potential.
  * **Conversation**: Links a Contact and an Inbox, acting as the threaded container for messages.
  * **Message**: The granular unit of communication, supporting text, attachments, and rich payloads, linked to a conversation and sender (Contact or Agent/Bot).
  * **Channel Adapters**: Specific channel configurations (Email, Facebook Page, Web Widget, API, etc.) bound to an Inbox.

  # Design Doc
  ## Architecture & Data Model (Native Rust + Postgres)
  We will implement a native Rust set of domain models mapped to a multi-tenant PostgreSQL schema (via SeaORM, sqlx, or the active OHC ORM pattern).

  **Entity-Relationship Invariants:**
  * Strict Row-Level Security / Multi-Tenant Isolation using `tenant_id` (or `account_id`) on EVERY table.
  * **`ohc_inboxes`**: `id`, `tenant_id`, `name`, `channel_type`, `channel_id` (polymorphic or JSON config).
  * **`ohc_contacts`**: `id`, `tenant_id`, `name`, `email`, `phone_number`, `identifier` (for channel-specific IDs), `custom_attributes` (JSONB).
  * **`ohc_conversations`**: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `agent_id` (optional, for assigned human/bot).
  * **`ohc_messages`**: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, agent, bot), `sender_id`, `content` (text), `content_type` (enum), `status` (sent, delivered, read).

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      TENANT ||--o{ MESSAGE : has

      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ## AI Agent Integration Points
  * **The Ambassador Agent** will listen to the event bus for `MessageCreated` events on `ohc_messages` where `sender_type == contact`.
  * It will query the `ohc_conversations` and `ohc_contacts` tables to pull historical context and draft a reply.

  ## Mobile UX Flow (375px)
  * The frontend will consume these models via a gRPC/REST API.
  * The **Unified Inbox Feed** will render `ohc_conversations` ordered by `last_activity_at`, showing the `Contact` name, `Inbox` source icon (e.g., IG, WhatsApp), and a snippet of the latest `ohc_messages`.

  # Implementation Prompt
  Implement the core database schema (migrations) and Rust domain entities/DAOs for the native OHC Omnichannel Chat system, achieving parity with Chatwoot's core structural models.

  **Acceptance Criteria:**
  1. Define up/down database migrations for `ohc_inboxes`, `ohc_contacts`, `ohc_conversations`, and `ohc_messages` ensuring strict `tenant_id` multi-tenancy.
  2. Implement the Rust structs and repository interfaces to perform basic CRUD operations for these entities.
  3. Ensure all new models have 100% unit test coverage validating tenant isolation (e.g., fetching a conversation for Tenant A cannot return a conversation for Tenant B).
  4. Integrate these repositories into the existing backend service layer, exposing foundational internal APIs that the Event Mesh and API gateway can build upon.

  *Note: Do not build the channel-specific webhook listeners (IG, WhatsApp) in this task. Focus strictly on the core multi-tenant data structures, repositories, and domain models.*

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
