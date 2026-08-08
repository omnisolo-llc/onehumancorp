issue_title: "Implement native Rust Omnichannel Chat to replace Chatwoot"
issue_description: |
  ## Problem Statement
  Chatwoot has been identified as an external dependency that needs to be completely removed from the OneHumanCorp stack, as dictated by the mandatory architectural standard `Chatwoot Retirement & Custom Rust Omnichannel Chat System Standard (MANDATORY)`. Our SMB personas (Maya, Carlos, Priya, Leo, Fatima) need a unified, high-performance, and multi-tenant safe inbox to handle customer inquiries seamlessly without relying on third-party integrations, which adds latency, complexity, and breaks offline/local capabilities. Currently, Chatwoot deployment footprint is being cleaned up, but we lack the complete native Rust-based omnichannel messaging system that matches Chatwoot's features to take over.

  ## Research Report
  - **Codebase context**: The repository already has `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`, a detailed spec outlining the removal of Chatwoot and replacement with a Rust native solution.
  - **Market benchmark (Chatwoot vs SMB needs)**:
      - Chatwoot provides an omnichannel inbox, widget, webhooks, SLA, and macros.
      - However, Chatwoot architecture (Rails/Sidekiq/Postgres/Redis) is resource-intensive and violates OHC's zero-trust SPIFFE/SPIRE goals and multi-tenant row-level-security paradigms.
      - Competing products like Shopify Inbox and Wix Inbox use highly integrated first-party systems that provide a cohesive experience.
  - **Observed Gap**: The system needs a Rust-native real-time conversation engine capable of handling WebSocket events, tenant isolation, outbox delivery receipts, and connecting with the Next.js/Flutter frontends. The Next.js frontend has basic `/inbox` paths, but they lack the full backend integration mirroring Chatwoot's capabilities.

  ## Design Doc
  ### Architecture
  - **Core Component**: A new Rust crate (e.g., `src/server/omnichannel`) defining the canonical conversation domain.
  - **Data Model**:
    - `Conversation` (tenant_id, id, status, channel, created_at, updated_at)
    - `Message` (tenant_id, id, conversation_id, sender_type, content, status)
    - `Contact` (tenant_id, id, name, email, phone)
    - `ChannelAdapter` configuration for email/SMS/WhatsApp.
    - All models must implement strict multi-tenant isolation via `tenant_id` predicates (SQLite/PostgreSQL RLS).
  - **Real-time Sync**: Authenticated WebSockets delivering events to the Next.js widget and PowerSync for local offline availability (Tauri desktop/Flutter mobile).
  - **Transaction Outbox**: Deliver messages via a transactional outbox pattern. This ensures no message is lost. If an external API fails, the outbox retries.
  - **AI Coordination**: The operations and customer service AI agents subscribe to inbox events to auto-draft replies or triage tickets without exposing credentials directly.

  ### Mobile UX Flow
  - Maya opens the app (375px viewport) and sees the `Unified Inbox` icon with a badge.
  - Tapping opens a list of conversations (WhatsApp, IG, Web Widget).
  - Tapping a conversation opens a familiar chat interface (macOS Translucent Glass style).
  - AI drafts are shown inline as suggested replies; Maya can tap to send or edit.

  ### AI Agent Integration
  - Incoming messages trigger an AI triage background job.
  - The AI worker reads the tenant context and the message, identifies intent (e.g., "quote request"), and posts a drafted reply in the `Message` table with status `draft`.

  ## Implementation Prompt
  Implement the core canonical conversation domain in Rust (`src/server/omnichannel` or equivalent as defined by the directory structure).
  1. Define the Rust structs and PostgreSQL/SQLite repository traits for `Conversation`, `Message`, and `Contact`.
  2. Ensure strict `tenant_id` based isolation in all queries.
  3. Implement the transactional outbox pattern for sending messages reliably.
  4. Create the core gRPC/REST APIs required by the frontend to fetch conversations and send messages.
  5. Achieve 100% unit test coverage for the new domain logic, ensuring no mock data is hardcoded in the frontend APIs.
  6. Ensure all `bazel test //...` run green.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
