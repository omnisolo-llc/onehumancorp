issue_title: "[Native Chat] System Design: Core Domain and Persistent Delivery Engine"
issue_description: |
  ## Problem Statement
  OneHumanCorp's existing chat footprint relies on an unused Chatwoot connection and a fragmented native persistence model (`inbox_messages`, `omni_inbox_messages`, `unified_*`). Non-technical owners (like Maya the baker and Carlos the handyman) need a seamless, unified inbox that flawlessly consolidates all their channels (Instagram DMs, SMS, Web Widget). They require absolute confidence that when they send a quote or draft a reply via the UI or their AI assistant, the message will be delivered with zero risk of silent failure. The current disjointed data model and lack of a robust delivery outbox expose the business to race conditions, dropped messages, and cross-channel confusion.

  ## Research Report
  - **Codebase Audit:** The native inbox foundation exists (Next.js `/inbox`, Rust APIs, multi-tenant DB structure), but it lacks a unified canonical conversation domain and a transactional delivery outbox.
  - **Chatwoot Audit:** Analyzed the `chatwoot` source codebase. It models `Conversation`, `Message`, `Inbox`, and `Contact`. It uses a robust status machine for message delivery and assignment policies.
  - **Competitive Analysis:** Leading omnichannel platforms (like Zendesk, Intercom, Shopify Inbox) use an immutable event ledger and a strict transactional outbox pattern to guarantee at-least-once delivery, decoupled from immediate API ingress/egress constraints.

  ## Design Doc
  ### Architectural Overview
  We will introduce a canonical, multi-tenant `Conversation` and `Message` domain in the Rust backend, fortified by a robust Transactional Outbox for delivery.

  1. **Canonical Domain Model:**
     - `Inbox`: Represents a specific channel connection (e.g., "Maya's Instagram", "Carlos' SMS").
     - `Conversation`: The central aggregation of a thread between the owner and a contact within an inbox.
     - `Message`: Immutable records representing inbound and outbound communications.

  2. **Transactional Outbox Engine:**
     - All outbound messages and system events (e.g., assignment changes) are written to a `DeliveryOutbox` table *within the same database transaction* as the state change.
     - An asynchronous Rust background worker (using the existing job queue or a dedicated polling loop with `SKIP LOCKED`) processes the outbox.
     - The worker handles retries (exponential backoff), idempotency (using unique `idempotency_key` per message), and dead-lettering for failed deliveries.

  3. **Multi-Tenancy & Security:**
     - Strict Row-Level Security (RLS) on all new PostgreSQL tables based on `tenant_id`.
     - Strong isolation guarantees; a tenant can never query or mutate another tenant's conversation data.

  ### Data Model (Mermaid Diagram)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ Conversation : "contains"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"
      Message ||--o| DeliveryOutbox : "triggers delivery"

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string channel_type
          jsonb credentials
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          timestamp last_activity_at
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string message_type "incoming, outgoing, system"
          text content
          string status "pending, sent, delivered, failed"
          uuid idempotency_key
      }
      DeliveryOutbox {
          uuid id PK
          uuid tenant_id FK
          uuid message_id FK
          string target_channel
          jsonb payload
          string status "pending, processing, completed, failed"
          int retry_count
          timestamp next_retry_at
      }
  ```

  ### Mobile-First UX Flow (375px)
  While this is a backend-heavy architectural change, the UI contract must be pristine:
  - **Unified Feed:** The primary mobile screen aggregates active `Conversations`.
  - **Truthful Delivery Status:** When the owner sends a message, it immediately renders optimistically in the thread with a translucent "sending" indicator (driven by the `pending` state). If the Outbox exhausts retries and dead-letters, the UI reflects a clear, actionable "Failed to send - Tap to retry" state (driven by the `failed` status).
  - **No Phantom Messages:** A message is never marked "Sent" unless the Outbox has successfully handed it off to the provider.

  ### AI Integration Points
  - **AI Drafter:** AI agents will propose draft `Messages`. These drafts are saved with a `draft` status and do not enter the `DeliveryOutbox` until explicitly approved (or auto-approved by policy) by the owner.
  - **Triage Agent:** An inbound AI triage agent will analyze new `Conversation` updates to automatically set priority, tags, or assign them to the owner.

  ## Implementation Prompt
  **Goal:** Implement the foundational schema and repository layers for the canonical Omnichannel Chat domain and the Transactional Delivery Outbox.

  **Acceptance Criteria:**
  1. Define Rust structs and diesel/sqlx models for `Inbox`, `Conversation`, `Message`, and `DeliveryOutbox`.
  2. Implement database migrations creating these tables with strict `tenant_id` foreign keys and PostgreSQL RLS policies enabled.
  3. Create repository functions (e.g., `create_message`) that accept a database transaction to ensure the `Message` and its corresponding `DeliveryOutbox` record are created atomically.
  4. Implement a background worker (Rust `tokio` task) that safely claims pending `DeliveryOutbox` records (using `SKIP LOCKED`), simulates a delivery attempt, and updates the status to completed or increments retry counts with backoff.
  5. 100% unit test coverage for the repository layer and the outbox worker logic.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
