issue_title: "Native Omnichannel Platform Architecture"
issue_description: |
  # Native Omnichannel Replacement Design

  ## Problem Statement
  OneHumanCorp (OHC) is currently retiring the external omnichannel dependency. To serve our owner/operator personas (Maya the baker, Carlos the handyman, Fatima the food cart owner), OHC must provide a native, resilient, and multi-tenant omnichannel inbox that aggregates customer conversations across Instagram DMs, WhatsApp, SMS, Web Chat, and Email into a single actionable feed. Our owners cannot navigate complex CRM software; they need an assistant-first inbox where AI agents draft replies, update orders, and take deposits seamlessly alongside manual operator responses.

  ## Research Report
  - **Codebase Audit:** The external dependency has been fully unintegrated, and its artifacts have been cleansed from our Rust backend and Helm deployments. We have foundational elements in `src/server/ohc/inbox`, `src/ui/next/src/lib/auth`, and generic omnichannel data models to build upon.
  - **Source Benchmarking:** By analyzing the external dependency's source logic (particularly models like `Account`, `Inbox`, `Conversation`, `Message`, `Channel::*`, `Contact`, `Webhook`), we observe that a robust native implementation requires:
      - Strong multi-tenant data boundaries (row-level security via `tenant_id`).
      - A unified `Conversation` model that links back to a polymorphic `Channel`.
      - High-performance WebSocket event broadcasting to keep operators (desktop/web) in sync.
      - Reliable delivery state outbox patterns.
  - **Competitor Insights (Shopify Inbox, Zendesk, Apple Business Chat):** The best platforms unify the thread timeline, combining systemic events (order status, payment links) natively with customer messages. We must build this directly into the Rust backend so AI agents can execute actions (e.g. `Maya: "Can I get a vegan cake?" -> AI drafts quote + payment link`) as part of the normal conversation flow.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
    A[Customer Channels: Web Widget, IG, WhatsApp, Email] --> |Webhooks/REST| B[API Gateway & Auth]
    B --> C[Rust Omnichannel Service]
    C --> |Commands & Events| D[(PostgreSQL - Cloud Canonical)]
    C --> |Sync| E[(PowerSync/SQLite - Local Replicas)]
    C --> F[AI Agents Job Queue]
    D -.-> |CDC/WebSockets| G[Next.js Web / Tauri Desktop UI]
    F -.-> |Drafts/Actions| C
  ```

  ### Data Model & Invariants
  The domain is centered in a bounded `omnichannel` context:
  - `Tenant`: The owner workspace. Every table enforces `tenant_id` RLS.
  - `Inbox`: Routing boundary for channels and team members.
  - `ChannelConnection`: Encrypted provider credentials (e.g., WhatsApp API key).
  - `Contact`: Customer profile linked to unified identities.
  - `Conversation`: Thread timeline bridging messages, actions, and SLAs.
  - `Message`: Immutable timeline entry (inbound, outbound, private note, agent draft, system event) with delivery status (`Receipt`).

  ### Mobile UX Flow (375px First)
  - **Bottom Tab:** 'Inbox' showing unread badge count.
  - **List View:** Unified chronological feed of active conversations. Unread states use OHC Premium Token colors. Swipe to resolve/archive.
  - **Detail View:** Chat bubble timeline. Bottom sticky input field with AI draft chips (e.g., "Tap to send AI quote"). Translucent glass app bar showing customer name and channel icon.

  ### AI Agent Integration Points
  - **Work Triage:** AI agents subscribe to inbound message events via the Job Queue, interpret intent, and inject 'Private Notes' or 'Agent Drafts' into the timeline.
  - **Customer Relationship:** AI retrieves previous `Contact` context to personalize drafts.

  ## Implementation Prompt
  Implement the native Rust omnichannel data models and repository interfaces matching the feature parity audit.
  1. Define Rust structs/entities for `Inbox`, `ChannelConnection`, `Contact`, `Conversation`, and `Message` in the `omnichannel` module.
  2. Implement strict multi-tenant constraints enforcing `tenant_id` on every repository operation.
  3. Ensure compatibility with both PostgreSQL (cloud) and SQLite (desktop via PowerSync) implementations.
  4. Create the foundational unit tests demonstrating CRUD operations with tenant isolation.

  **Acceptance Criteria:**
  - `bazel test //...` passes.
  - Unit tests achieve 100% coverage on new models.
  - No external dependencies are introduced.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
