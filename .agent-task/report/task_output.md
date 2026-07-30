issue_title: "Native Rust Omnichannel Chat: Data Model & Architecture Design"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) has removed the legacy Chatwoot integration and requires a native, high-performance, multi-tenant Rust omnichannel chat system. Our personas (Maya, Carlos, Priya, Leo, Fatima) need a unified, mobile-first inbox for Instagram DMs, WhatsApp, web chat, and SMS, seamlessly coordinated by AI assistants. We must design a strictly isolated, multi-tenant data model and architecture in Rust that fulfills the `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` specification while meeting OHC's high-scale and Zero Trust requirements.

  ## Research Report & Architectural Audit
  - **Chatwoot Source Benchmarking:** Based on historical Chatwoot architecture (now superseded), the core model features `Account` (Tenant), `User`, `Inbox`, `Channel::*`, `Contact`, `Conversation`, and `Message` entities. The system relies heavily on polymorphic associations and PostgreSQL for transactional integrity.
  - **OHC Gaps:** OHC needs native Rust models for this deep, omnichannel messaging hierarchy. We need a Rust implementation using SQLx that enforces tenant isolation (RLS via `tenant_id`) at every level.
  - **Competitor Insights:** Leading platforms use edge-caching and WebSocket-driven real-time updates. Our design must support similar real-time capabilities natively within the OHC stack.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ USER : contains
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CHANNEL_ADAPTER : configures

      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : participates_in
      CHANNEL_ADAPTER ||--o| INBOX : routes_to

      CONVERSATION ||--o{ MESSAGE : contains
      USER ||--o{ MESSAGE : sends

      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          uuid sender_id
          string content
          string message_type
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid tenant_id
          string provider_type
          jsonb credentials
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean list of active conversations, clearly badged by channel. Touch targets > 44px.
  - **Conversation Thread:** Translucent glass header, scrollable message history. Quick reply suggestions powered by the local AI agent.
  - **Offline Resilience:** Messages queue locally and sync when connection restores.
  - **No Admin Clutter:** Channel configuration is hidden in an "Advanced Settings" drawer.

  ### AI Agent Integration
  - **Work Triage Agent:** Intercepts incoming messages, categorizes intent (e.g., "quote request", "support"), and routes to the correct inbox.
  - **Customer Assistant Agent:** Drafts replies based on past context and tenant memory, presenting them for owner approval or sending autonomously if confident.

  ### Key Design Decisions
  - **Strict Row-Level Security (RLS):** Every table MUST include a `tenant_id` column, enforced via Postgres RLS.
  - **Native Rust Services:** Built using Axum for APIs and SQLx for database access.
  - **Real-time Engine:** Native WebSocket integration within the Rust server.

  ## Implementation Prompt
  **Goal:** Implement the foundational database schema and Rust entity models for the native OHC Omnichannel Chat system.

  **Critical User Journey (CUJ):**
  1. A new tenant (e.g., Maya's Bakery) is created.
  2. The system automatically provisions a default `Inbox` and a web `ChannelAdapter`.
  3. A customer initiates a chat, creating a `Contact`, a `Conversation`, and an initial `Message`.
  4. The owner views the message in the unified inbox API.

  **Acceptance Criteria:**
  - Database migrations for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages` are created.
  - Every table includes a `tenant_id` and enforces RLS.
  - Rust models and SQLx repository interfaces are implemented.
  - 100% unit test coverage for the repository layer.
  - **MANDATORY:** Run `bazel test //...` to ensure no regressions and verify test coverage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
