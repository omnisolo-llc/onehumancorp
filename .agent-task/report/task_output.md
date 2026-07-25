issue_title: "Architecture & Implementation Plan: Native Omnichannel Chat Core & Inbox"
issue_description: |
  # Problem Statement
  Small business owners (Maya, Carlos, Priya, Leo, Fatima) struggle to manage customer communications across fragmented channels (Instagram DMs, WhatsApp, SMS, Email). The previous third-party solution (Chatwoot) was overly complex, lacked deep native integration with OHC's multi-tenant architecture, and has been completely removed. We need a native, Rust-based, highly performant omnichannel unified inbox that aggregates communications, enforces strict row-level security per tenant, and deeply integrates with OHC's AI agents (The Ambassador, The Manager) to proactively draft context-aware responses.

  # Research Report
  **Findings & Chatwoot Source Code Audit:**
  - Chatwoot's architecture relies on Ruby on Rails with a complex set of models: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*` (Adapters).
  - WebSockets (ActionCable) were used for real-time updates.
  - OHC requires a Rust-based, highly concurrent alternative leveraging gRPC/REST APIs and a robust distributed event bus (e.g., Redis/NATS or our existing PostgreSQL SKIP LOCKED queue) for real-time message processing and background AI tasks.
  - Native integration with our PostgreSQL Row-Level Security (RLS) is paramount to guarantee zero cross-tenant data leakage.

  **Competitive Analysis:**
  - **Shopify/Wix:** Basic aggregation, manual replies, limited AI context.
  - **OHC Advantage:** "Assistant-First." The inbox isn't just a list of messages; it's a prioritized feed where the AI has already drafted responses based on the customer's full history and current business context (e.g., catalog, inventory, schedule).

  # Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[External Channels: IG, WhatsApp, Email] -->|Webhooks| B(Omnichannel Gateway / Ingress)
      B --> C{Connector Verification & Resolution}
      C -->|Valid| D[Event Bus / Queue]
      D --> E(Core Message Processor)
      E --> F[(PostgreSQL: Unified Conversation DB with RLS)]
      E --> G[Agent Trigger: The Ambassador]
      G -->|Lookup Context| F
      G -->|Draft Reply| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J(Omnichannel Egress / Outbox)
      J --> A
      F -.->|Real-time Sync| K(PowerSync / WebSockets)
      K -.-> I
  ```

  ### Data Model & Invariants (Core Entities)
  - `Tenant` (`tenant_id` - enforced via RLS on all child tables)
  - `Contact`: Represents the end customer. Links to identity resolution.
  - `Inbox`: A channel configuration for a tenant (e.g., "Maya's Instagram").
  - `Conversation`: A thread between a `Contact` and an `Inbox`.
  - `Message`: Individual messages within a `Conversation`. Supports attachments, rich text, and status (draft, pending, sent, failed).
  - `ChannelAdapter`: Configuration and credentials for specific external services.

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed:** Owner opens OHC. Top priority card: "New Message from [Contact] via [Channel]".
  2. **Unified View:** Tapping the card reveals the message history.
  3. **AI Assistance:** A prominent "Drafted Reply" section shows what *The Ambassador* suggests, alongside the reasoning ("Based on their last order...").
  4. **Action:** Big, thumb-friendly buttons: "Approve & Send", "Edit Draft", "Escalate".
  5. **Network Resilience:** Sending a message immediately updates the UI optimistically, queueing it in the robust Outbox pattern for guaranteed delivery.

  ### Key Design Decisions
  - **Rust Native:** High performance, low memory footprint, strong type safety compared to Rails.
  - **Transactional Outbox:** Guaranteed delivery to external channels even if the application crashes immediately after saving to the database.
  - **Protected by Default:** All routes require authentication. Webhooks undergo strict signature validation.
  - **Strict Multi-Tenancy:** PostgreSQL RLS is the ultimate enforcement layer.

  # Implementation Prompt
  **User-Facing Outcome:** The owner receives messages from any channel in a single, unified, mobile-first feed, complete with context-aware, AI-drafted responses ready for one-tap approval.
  **CUJ & Acceptance Criteria:**
  1. Implement the core database schemas for `Contact`, `Inbox`, `Conversation`, and `Message` in PostgreSQL, strictly enforcing `tenant_id` via Row Level Security (RLS).
  2. Create the Rust data models and repository traits/implementations for these entities in the `src/server/integrations/core` or equivalent module.
  3. Implement a canonical REST/gRPC API for querying and mutating conversations and messages, ensuring tenant isolation.
  4. Implement the Transactional Outbox pattern for reliable message egress.
  5. Create comprehensive unit tests with 100% coverage for the new Rust code.
  6. Write a Playwright E2E test verifying a user can view a mock conversation and message in the unified inbox UI.

  # Priority & Scope
  **Priority:** P0 (Critical foundational architecture for the product)
  **Scope:** Large (Requires database migrations, Rust backend implementation, and fundamental API design)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, core]
assignees: []
