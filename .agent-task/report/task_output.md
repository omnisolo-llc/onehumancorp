issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external chat systems or decoupled messaging architectures to handle customer support, omnichannel conversations, and inbox management. As OHC scales to handle more demanding owner/operator use cases (e.g., Maya's Instagram DMs, Carlos's SMS quotes), relying on third-party dependencies like Chatwoot introduces multi-tenancy risks, data fragmentation, performance overhead, and disjointed agent coordination. We need a native, high-performance, multi-tenant omnichannel customer support and chat engine built entirely in Rust, operating inside `onehumancorp/mono`.

  ## Research Report
  ### Market & Architecture Analysis
  An audit of Chatwoot's source code (v3.x) reveals the foundational models of a modern omnichannel inbox:
  - **Tenancy**: `Accounts` map to our OHC `Tenants` (Row-Level Security required).
  - **Identities**: `Users` (Agents) and `Contacts` (Customers).
  - **Core Entities**: `Inboxes`, `Conversations`, and `Messages`.
  - **Channel Adapters**: Independent models linking external channels (`Channel::WebWidget`, `Channel::Email`, `Channel::Whatsapp`, `Channel::Instagram`) to `Inboxes`.
  - **Real-time**: WebSocket connections pub/sub via Redis.

  **Limitations of adopting a Ruby-based 3rd-party tool (Chatwoot)**:
  - High memory footprint.
  - No direct integration with OHC's internal `Tenant` context, necessitating messy syncs.
  - Lacks native integration with OHC AI agents for automated replies, triage, and draft generation.

  ### Proposed Architecture & Solution
  We will implement a native Rust omnichannel chat system within `onehumancorp/mono`, integrating seamlessly with OHC's existing PostgreSQL schema, Redis lock patterns, and AI Agent job queues.

  ## Design Doc
  ### Data Model & Invariants
  1. **Strict Multi-Tenancy**: All tables must include `tenant_id` and enforce PostgreSQL Row-Level Security (`ENABLE ROW LEVEL SECURITY`).
  2. **Core Entities**:
     - `chat_inboxes`: Represents a channel endpoint (e.g., "Maya's IG DM", "Carlos Web Widget").
     - `chat_channels`: Configuration and credentials for specific adapters (WhatsApp, IG, SMS).
     - `chat_contacts`: The customer profile interacting across channels.
     - `chat_conversations`: Groups messages for a specific contact and inbox.
     - `chat_messages`: Immutable message events (Text, Image, System, AI-Draft).
     - `chat_participants`: Links agents or AI bots to conversations.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--|| Channel : configures
      Tenant ||--o{ Contact : manages
      Contact ||--o{ Conversation : initiates
      Inbox ||--o{ Conversation : contains
      Conversation ||--o{ Message : holds
      Tenant ||--o{ User : employs
      User ||--o{ Conversation : assigned_to

      Tenant {
          uuid id
          string name
      }
      Inbox {
          uuid id
          uuid tenant_id
          string name
          uuid channel_id
      }
      Channel {
          uuid id
          string type
          json config
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
          uuid conversation_id
          string content
          string message_type
          uuid sender_id
      }
      Contact {
          uuid id
          uuid tenant_id
          string name
          string phone
      }
  ```

  ### Mobile UX Flow
  1. **Work Triage**: On a 375px mobile device, an owner sees a unified "Inbox" tab.
  2. **Unified Feed**: A single list shows Instagram DMs, SMS, and Web Widget messages ordered by priority and recent activity.
  3. **AI Draft Assistance**: When the owner opens a conversation, the OHC Customer Assistant AI agent has already drafted a context-aware reply (e.g., pulling pricing info for Carlos).
  4. **Action Sheets**: Owners can mark as "resolved", "snooze", or convert the conversation into a "quote" or "booking" directly from the chat UI without leaving the screen.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to the `message_created` event (via PG `SKIP LOCKED` job queue). Analyzes intent and contextually drafts replies for human approval.
  - **Work Triage**: Automatically tags and prioritizes inbound `chat_conversations` based on urgency (e.g., "complaint", "new lead").

  ### Technical Integrity & Zero-Trust Security
  - **Identity**: API and WebSocket access governed by SPIFFE/SPIRE for internal microservices, and standard JWTs scoped strictly to `tenant_id` for client apps.
  - **Real-Time**: Axum (Rust) WebSocket server routing messages via Redis Pub/Sub channels formatted as `ohc:tenant:{tenant_id}:inbox:{inbox_id}`.
  - **Performance**: P99 read latency < 50ms. Messages pushed to offline-first Flutter local SQLite (Drift) for seamless low-network usage (Fatima persona).

  ## Implementation Prompt
  **Goal**: Implement the native Rust core domain logic and PostgreSQL repository layer for the OHC Omnichannel Chat System.
  **CUJ**: A non-technical owner (Maya) opens her OHC app and sees a unified list of conversations spanning her Instagram and website widget, with AI drafts ready for approval.
  **Tasks for Implementer**:
  1. Define the SQL migrations for the core models (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with `tenant_id` and RLS.
  2. Implement the Rust Axum REST endpoints for fetching inboxes and conversations.
  3. Implement the Axum WebSocket handler for real-time message streaming using Redis Pub/Sub.
  4. Ensure 100% unit test coverage for the repository and service layers.
  5. Implement E2E Playwright tests simulating a new inbound message hitting the API and appearing in the unified inbox.

  ## Priority
  P0
  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
