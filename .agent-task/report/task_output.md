issue_title: "[Research] Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Title
  [Research] Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OHC currently relies on external systems or lacks a comprehensive native, high-performance, multi-tenant omnichannel chat inbox architecture similar to Chatwoot. As mandated by the platform standards, Chatwoot as an external service is 100% RETIRED, and OHC needs to build its own native Rust omnichannel customer support & chat engine. Small business owners like Maya (baker) and Carlos (handyman) need to triage Instagram DMs, SMS, WhatsApp, and Web Chat from a single, unified, mobile-friendly interface without dealing with technical complexity or multiple apps.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture requires several key entities to enable an omnichannel inbox:
  - **Account / Tenant**: The root isolation boundary.
  - **Inbox**: A container for conversations coming from a specific channel (e.g., "Main Website Widget", "Instagram DMs").
  - **Channel Adapters**: Abstracted handlers for different platforms (Web Widget, API, Email, Facebook Page, Instagram, SMS, Whatsapp, Line, Telegram, Twilio).
  - **Conversation**: Represents a thread between an agent/bot and a contact. Needs status (`open`, `resolved`, `snoozed`, `pending`), priority, and assignment tracking.
  - **Message**: Individual payloads within a conversation (text, attachments, template messages).
  - **Contact**: The end-user communicating with the business across potentially multiple channels (needs identity merging).

  Competitors like Shopify Inbox, WeCom, and Inbox by Zendesk all use similar unified data models but often struggle with mobile-first performance on low-end devices. OHC's implementation will use Rust for high-throughput, low-latency WebSocket connections and robust background job processing, backed by PostgreSQL with strict row-level security (RLS) for multi-tenancy.

  ## Design Doc
  ### Architecture Diagram (Conceptual)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL : configures
      CHANNEL {
          string type "web_widget, sms, whatsapp, instagram"
          jsonb config "credentials and settings"
      }
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      CONTACT {
          string identifier
          string name
          string phone_number
          string email
      }
      MESSAGE {
          string content
          string message_type "incoming, outgoing, template"
          jsonb attachments
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home)**: A single, unified list of active conversations across all channels. Unread messages have a clear visual indicator. Uses standard UniFi dashboard card layouts with translucent glass headers.
  2. **Conversation View**: Tap a conversation to view the message thread. Input area at the bottom uses the native mobile keyboard. Send button changes state based on network connectivity.
  3. **Contact Context Sheet**: Swiping left or tapping a contact avatar opens a bottom sheet showing the customer's order history, tags, and previous interactions.
  4. **Agent Handoff**: A clear "Assign to AI / Assign to Me" toggle at the top of the conversation.

  ### AI Agent Integration
  - **Work Triage Agent**: Monitors new incoming messages, applies tags, categorizes intent (e.g., "Support", "Sales", "Refund"), and decides whether to draft a reply or alert the owner.
  - **Customer Assistant Agent**: Drafts context-aware replies based on the tenant's knowledge base and past interactions. Can be set to auto-reply or save as draft for owner approval.
  - **Operations Agent**: Can extract structured data from conversations (e.g., delivery address, appointment time) and create actionable tasks or bookings.

  ### Key Design Decisions
  - **Native Rust Implementation**: High performance, memory safety, and seamless integration with the existing OHC backend architecture.
  - **Row-Level Security (RLS)**: Mandatory for all new tables (`inboxes`, `conversations`, `messages`, `contacts`) to ensure strict tenant isolation.
  - **Real-time WebSockets**: Push new messages and status updates to the mobile/web clients instantly.
  - **Offline-Tolerant Mobile Writes**: The Flutter app must queue outgoing messages locally and sync when the network is restored, showing truthful pending states.

  ## Implementation Prompt
  **Goal**: Implement the foundational data models and database migrations for the native Rust omnichannel inbox system in `src/server`.

  **Tasks**:
  1. Create SQLx migrations for the following entities, ensuring strict multi-tenant RLS (Row-Level Security) using `tenant_id`:
     - `inboxes` (id, tenant_id, name, channel_type, settings_jsonb)
     - `contacts` (id, tenant_id, name, email, phone, avatar_url, custom_attributes_jsonb)
     - `conversations` (id, tenant_id, inbox_id, contact_id, status, assignee_id, last_activity_at)
     - `messages` (id, tenant_id, conversation_id, sender_type, sender_id, content, attachments_jsonb)
  2. Implement the corresponding Rust struct models in `src/server/domain` or the appropriate models directory, deriving necessary traits for Serde and SQLx.
  3. Implement repository functions for basic CRUD operations on these entities.
  4. Ensure 100% unit test coverage for the repository layer.
  5. Provide a basic Playwright E2E test verifying that a simulated API call to create a message successfully persists the data (without full UI implementation yet).

  **Acceptance Criteria**:
  - Migrations run successfully and create tables with RLS policies.
  - Rust models compile and accurately represent the schema.
  - Repository tests pass with 100% coverage for the new code.
  - No external dependencies on Chatwoot are introduced.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
