issue_title: "[Platform] Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine. Previously, we relied on the external Chatwoot service, but as per the architectural directive, we are fully retiring Chatwoot as a third-party dependency. OHC must implement its own native Rust chat system to support real-time messaging, multi-channel aggregation (WhatsApp, Instagram, Web Chat), and seamless AI agent coordination (Operations and CS departments). This system must be completely multi-tenant, observable, and fully usable on a 375px mobile viewport. The business personas (e.g., Maya the baker managing Instagram DMs, Carlos the handyman answering service inquiries) need a unified inbox that guarantees offline capabilities and real-time synchronization.

  ## Research Report
  Based on an in-depth audit of the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`), the core architecture revolves around a robust set of data models and event-driven abstractions. Chatwoot relies on the following key entities, which we must replicate natively in Rust:
  - **Account/Tenant**: The root boundary for isolation.
  - **Inbox**: A collection of channels and routing rules.
  - **Channel**: Specific integrations (e.g., WhatsApp, Instagram, Email, Web Widget).
  - **Conversation**: An ongoing thread between a Contact and Agents (Human or AI).
  - **Message**: Individual payloads (text, attachments, interactive elements) within a conversation.
  - **Contact**: The customer or lead on the other side of the interaction.

  To ensure performance and multi-tenant safety, OHC will use PostgreSQL with Row-Level Security (RLS) for persistence and Redis (Redlock) for distributed locks and fast pub/sub event broadcasting.

  ## Design Doc
  The Omnichannel Chat System will be a native Rust module located in `src/server/ohc/domain/omnichannel_chat/`.

  ### Data Invariants and Multi-Tenancy
  All tables and data entities must enforce `tenant_id` isolation, protected by PostgreSQL Row Level Security (RLS). All cross-channel operations (e.g., syncing a WhatsApp message) must carry the `tenant_id` context through the background job queue (utilizing `SKIP LOCKED`).

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Tenant ||--o{ Conversation : "owns"
      Inbox ||--o{ Channel : "aggregates"
      Channel ||--o{ Conversation : "originates"
      Contact ||--o{ Conversation : "participates"
      Conversation ||--o{ Message : "contains"

      Tenant {
          uuid id PK
          string name
          string settings
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          jsonb routing_rules
      }
      Channel {
          uuid id PK
          uuid inbox_id FK
          string channel_type
          jsonb credentials
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
          jsonb custom_attributes
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          timestamp last_activity_at
      }
      Message {
          uuid id PK
          uuid conversation_id FK
          uuid tenant_id FK
          string content_type
          text content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px)
  - **Unified Inbox View**: The mobile view displays a consolidated list of active conversations, clearly badged by channel origin (e.g., Instagram icon, Web icon).
  - **Conversation Thread**: Optimized for touch, with native keyboard support, fast scroll, and translucent glass material for the message input bar.
  - **Action Sheet**: Easy-access buttons for AI Agent delegation (e.g., "Draft Reply", "Generate Quote").

  ### AI Agent Integration
  - **CS Department**: Background workers monitor the Redis pub/sub queue for incoming messages. If a message is unassigned and matches auto-reply rules, the CS Agent triggers a draft response.
  - **Coordination**: Uses Redis locks (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to ensure human agents and AI agents do not simultaneously reply to the same message.

  ## Implementation Prompt
  **Goal:** Build the native Rust multi-tenant Omnichannel Chat module and corresponding unified inbox UI.
  **Target Personas:** Maya (Baker on Instagram), Carlos (Handyman using Web Chat).
  **Acceptance Criteria:**
  1. Define Rust data models for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` in `src/server/ohc/domain/omnichannel_chat`.
  2. Implement database migrations incorporating strict multi-tenant Row Level Security (`tenant_id`).
  3. Implement the `SKIP LOCKED` PostgreSQL job queue pattern to handle incoming async messages.
  4. Build the unified inbox Flutter/Dart UI optimized for a 375px mobile screen, applying macOS-style translucent glass styling.
  5. **MANDATORY**: Ensure all features are verified by real browser/Playwright E2E tests executing a complete user journey without mock network calls or mock UI data.
  6. Achieve 100% test coverage and ensure `bazel test //...` passes.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
