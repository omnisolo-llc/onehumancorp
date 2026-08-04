issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat Engine (Chatwoot Replacement)

  ## Problem Statement
  OHC currently relies on external systems (or lacked native capability) for omnichannel customer support, creating fragmentation in the multi-tenant architecture. Chatwoot has been 100% RETIRED as a third-party dependency. OHC owners (Maya, Carlos, Priya) need a unified inbox that brings together Instagram DMs, WhatsApp, SMS, Email, and Web Chat into a single, cohesive interface. We must build a high-performance, multi-tenant omnichannel chat engine natively in Rust, directly integrated into OHC's backend, fully supporting SPIFFE/SPIRE identity and Postgres RLS.

  ## Research Report
  ### Competitive Analysis: Chatwoot Architecture Audit
  An extensive audit of the `chatwoot/chatwoot` repository (Ruby on Rails) revealed its core strengths and architectural models:
  1. **Omnichannel Inboxes**: Associates physical channels (e.g., WhatsApp, Web Widget) with logical Inboxes.
  2. **Conversations & Messages**: Centralized conversational threads linked to Contacts.
  3. **Real-time WebSockets**: ActionCable-based pub/sub for instant message delivery and typing indicators.
  4. **Agent Routing & SLAs**: Round-robin/manual routing and SLA-based SLA breach monitoring.

  ### Proposed Rust Architecture
  We will translate these concepts into a modern, high-performance Rust stack (Axum/Tokio) tailored for OHC:
  - **WebSockets**: Axum WebSockets integrated with Valkey (Redis) Pub/Sub for horizontal scaling.
  - **Data Models**: Implement `Contact`, `Conversation`, `Message`, `Inbox`, and `ChannelAdapter` with strict `tenant_id` boundaries for Postgres Row-Level Security (RLS).
  - **AI Agent Integration**: The Customer Service AI agent will listen to the Valkey Pub/Sub feed to automatically draft replies, triage messages, and execute macros.
  - **Zero-Trust**: All internal microservice communication will use mTLS via SPIFFE/SPIRE.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      TENANT ||--o{ CONTACT : owns
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| CHANNEL_ADAPTER : delivered_via

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          text content
          string sender_type
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          string channel_type
          json credentials
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: Bottom navigation tab leading to a list of active conversations, sorted by urgency/SLA.
  2. **Conversation Thread**: Tapping a conversation opens a chat view. Displays customer context (tags, past orders) in a collapsible top drawer to save vertical space.
  3. **AI Drafts**: AI-suggested replies appear above the native mobile keyboard as translucent, interactive chips.
  4. **Quick Actions**: Swipe left on a conversation to resolve, swipe right to assign to a human agent.

  ### AI Agent Integration Points
  - **Message Triage**: A background AI task (via PostgreSQL SKIP LOCKED queue) analyzes incoming messages, assigns priority, and applies tags.
  - **Auto-Drafting**: Generates draft responses based on tenant context (e.g., Maya's cake pricing) and places them in a `draft` state in the database, broadcasted to the UI via WebSocket.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core Native Rust Omnichannel Chat API and database schemas.
  1. Define the PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, `messages`, and `channel_adapters`, ensuring `tenant_id` is present on all tables for RLS.
  2. Build the Axum REST endpoints for CRUD operations on these entities.
  3. Implement the Axum WebSocket handler that subscribes to a Valkey (Redis) channel for real-time message broadcasting to the frontend.
  4. Ensure 100% unit test coverage for the Rust controllers and models.
  5. Add Playwright E2E tests verifying that a user can create an inbox, start a conversation, and see a message appear without reloading the page.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
