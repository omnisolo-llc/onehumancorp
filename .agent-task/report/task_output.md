issue_title: "Native Rust Omnichannel Chat System: WhatsApp & Web Widget Migration"
issue_description: |
  # Native Rust Omnichannel Chat System: WhatsApp & Web Widget Migration

  ## Problem Statement
  OneHumanCorp (OHC) is replacing its external reliance on Chatwoot with a 100% native Rust omnichannel chat system. The current legacy architecture relies on an external third-party chat service which violates our goal of complete control, privacy, performance, and Zero-Trust tenant isolation.
  Our owner personas (Maya the baker, Carlos the handyman, Fatima the food cart owner) need a unified inbox that supports Meta Webhooks (WhatsApp) and real-time website chat (Web Widget), all natively integrated directly into the OHC application so AI Agent Triage can monitor, respond, and categorize inbound interactions in real time.

  ## Research Report
  - **Chatwoot Source Code Audit**: Benchmarked the `inboxes`, `conversations`, `messages`, and `channel_whatsapp` schemas from `github.com/chatwoot/chatwoot`. Chatwoot uses polymorphic associations (`sender_type`, `sender_id`) and separate channel tables (e.g. `channel_whatsapp`, `channel_web_widget`) mapped via an `inboxes` table which acts as the unified sink.
  - **Data Isolation Requirement**: OHC must enforce strict tenant isolation using `tenant_id` at the database level with RLS (Row Level Security).
  - **Performance Requirement**: OHC's new system needs to use native Rust microservices for handling webhooks, persisting to Postgres efficiently, and routing WebSocket events.

  ## Design Doc
  ### Architecture
  1. **Data Model**:
     - `inboxes` (`id`, `tenant_id`, `name`, `channel_type`, `channel_id`, etc.)
     - `conversations` (`id`, `tenant_id`, `inbox_id`, `contact_id`, `status`)
     - `messages` (`id`, `tenant_id`, `conversation_id`, `sender_type`, `sender_id`, `content_type`, `content`)
     - `channel_whatsapp` (`id`, `tenant_id`, `phone_number`, `provider_config`)
     - `channel_web_widget` (`id`, `tenant_id`, `website_url`, `widget_color`)
  2. **Microservices (Rust)**:
     - **WhatsApp Webhook Handler**: Validates Meta webhook signatures and ingests messages into the `messages` table.
     - **Web Widget WebSocket Server**: Handles real-time bi-directional messaging from the storefront.
     - **AI Agent Router**: Hooks into Postgres `INSERT`s on `messages` to triage unassigned conversations.
  3. **Mobile UX Flow (375px)**:
     - Unified "Inbox" tab.
     - Swipe-to-resolve and tap-to-reply functionality.
     - "AI Triage" badge showing agent-drafted replies before manual send.

  ## Implementation Prompt
  Implement the backend core of the Native Rust Omnichannel Chat System for OneHumanCorp.
  1. Define the PostgreSQL schemas for `inboxes`, `conversations`, `messages`, and channel adapters (`channel_whatsapp`, `channel_web_widget`) ensuring strictly enforced multi-tenant RLS via `tenant_id`.
  2. Implement the Rust API endpoints to accept incoming Meta (WhatsApp) Webhooks and validate signatures.
  3. Build the core Rust domain logic for routing an incoming message to a `conversation` and `inbox`.
  4. Integrate a local WebSocket server foundation for the `channel_web_widget`.
  5. Include full unit test coverage and E2E Playwright tests that emulate an incoming WhatsApp message and verify it appears in the tenant's unified inbox API response.
  Follow the `src/server/integrations/chat/README.md` guidelines. Use `sqlx` or the repo's designated ORM and enforce standard OHC security practices.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
