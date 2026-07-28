issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Problem Statement
  OHC requires a native Rust omnichannel customer support and chat engine, fully retiring Chatwoot as a third-party dependency. Small business owners like Maya (baker) and Carlos (handyman) need unified communication channels (WhatsApp, Facebook, Instagram, Web Widget) directly integrated into the OHC command center. Without a native chat engine, maintaining Chatwoot creates deployment complexity, performance overhead, and disjointed multi-tenant user experiences, breaking the single-tenant OHC promise.

  ## Research Report
  - **Market Benchmark**: Chatwoot’s core system architecture relies on an Inbox > Channel > Conversation > Message hierarchy.
  - **Code Audit**: Auditing `/tmp/chatwoot/app/models/` revealed core tables: `conversations`, `messages`, `inboxes`, `contacts`, `channel_*`.
  - **OHC Implementation Gap**: OHC currently lacks robust Rust models and a scalable database schema mimicking the Chatwoot feature set for true omnichannel support. The implementation must ensure multi-tenancy (`tenant_id` at every level).

  ## Design Doc
  ### Data Model (Core Entities)
  1.  **Inbox**: Represents a unified destination for messages.
      - Fields: `id`, `tenant_id`, `name`, `channel_type`, `channel_id`, `greeting_enabled`, `working_hours_enabled`.
  2.  **Channel**: Adapters for different sources (WebWidget, WhatsApp, Facebook, Email).
      - Fields: `id`, `tenant_id`, `type`, `credentials_json`.
  3.  **Contact**: Represents the external user/customer.
      - Fields: `id`, `tenant_id`, `name`, `email`, `phone_number`, `identifier`, `avatar_url`.
  4.  **Conversation**: Represents a thread between a Contact and the Inbox.
      - Fields: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, snoozed, resolved), `assignee_id`.
  5.  **Message**: Individual chat entries.
      - Fields: `id`, `tenant_id`, `conversation_id`, `content`, `content_type` (text, attachment), `message_type` (incoming, outgoing), `sender_type`, `sender_id`.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL : configures
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          string content
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
      }
  ```

  ### Multi-Tenancy & Zero Trust
  - **RLS**: Row-Level Security policies on Postgres using `tenant_id` for every table.
  - **Isolation**: Every API call filters implicitly by `tenant_id` extracted from SPIFFE/SPIRE JWT tokens.

  ### Mobile UX Flow (375px)
  - **Unified Inbox View**: A simple list view showing all active conversations, badged with the channel icon (e.g., WhatsApp, Email).
  - **Chat Interface**: Standard message bubbles, persistent sticky input bar, quick-action agent commands (draft reply via AI).
  - **No clutter**: Settings for channel integration hidden under "Advanced Paths".

  ### AI Agent Integration
  - **Customer Assistant (Operations)**: Automatically listens to incoming messages, queries internal OHC memory (e.g., inventory, policies), and drafts replies for the owner to approve with one tap.

  ## Implementation Prompt
  **Goal**: Implement the core Rust data models and PostgreSQL schemas for the native OHC Omnichannel Chat System based on the Chatwoot audit.
  **Acceptance Criteria**:
  1. Create SeaORM (or equivalent SQLx) entities for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` under `src/server/services/chat/models.rs`.
  2. Implement Postgres database migrations defining these tables with strict `tenant_id` references and RLS constraints.
  3. Implement basic CRUD service methods in `src/server/services/chat/service.rs`.
  4. Ensure 100% unit test coverage for the new data models and multi-tenant boundary enforcement.
  5. Use `bazel test //...` to ensure no regressions in `main`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
