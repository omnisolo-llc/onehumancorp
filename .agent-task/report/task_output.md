issue_title: "[Native Chat] Implement Rust-Native Omnichannel Chatwoot Replacements (Data Models & Core Service)"
issue_description: |
  # Native Rust Omnichannel Inbox (Chatwoot Replacement) - Phase 1: Core Architecture

  ## Problem Statement
  OHC requires a unified inbox capable of replacing Chatwoot completely. Non-technical owner/operators (Maya, Carlos, Priya) currently manage inquiries across multiple channels (Instagram DMs, email, website chat widget, SMS, WhatsApp). Relying on an external service (like Chatwoot) increases complexity, disrupts multi-tenant isolation guarantees, and breaks the "Zero Secrets" architecture.

  We need a native Rust implementation of the core omnichannel messaging engine within the `onehumancorp/mono` repository to support the OHC Assistant. This replaces Chatwoot and gives OHC absolute control over routing, real-time WebSockets, and AI agent interception.

  ## Research Report
  Based on auditing the `https://github.com/chatwoot/chatwoot` repository, Chatwoot relies on the following core entities for its omnichannel architecture:
  - **Inbox**: A channel configuration (e.g. "Support Email", "Website Widget", "Instagram DM"). Includes configurations for greeting messages, auto-assignment, and working hours.
  - **Contact**: The customer interacting with the business (has email, phone, name, custom attributes).
  - **Conversation**: An ongoing thread between a `Contact` and an `Inbox`. Has status (open, resolved, snoozed), priority, assignee, and SLA details.
  - **Message**: A single message within a conversation. Has `message_type` (incoming, outgoing, template), `content_type` (text, attachment, interactive), and `sender_type`.

  OHC will rebuild this architecture natively in Rust (using Axum, Tokio, and our PostgreSQL/Redis stack). The primary divergence is integrating our AI Assistant seamlessly: the assistant should act as the default `assignee` before escalating to human operators.

  ## Design Doc
  ### Architecture Summary
  We will introduce a new Rust microservice (or crate within the mono repo) responsible for Omnichannel messaging.

  **Core Data Entities (PostgreSQL via sqlx in Rust):**
  - `tenant_id` (UUID) applied to all tables for Row-Level Security.
  - `inboxes`: `id`, `tenant_id`, `name`, `channel_type`, `settings (JSONB)`.
  - `contacts`: `id`, `tenant_id`, `name`, `email`, `phone_number`, `identifier`.
  - `conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status`, `assignee_id`.
  - `messages`: `id`, `tenant_id`, `conversation_id`, `content`, `message_type`, `sender_type`, `sender_id`.

  **System Components:**
  - **API Layer**: Rust Axum routes for CRUD operations on Inboxes, Contacts, and Conversations.
  - **Real-time Layer**: Tokio Tungstenite WebSockets for real-time delivery to the Flutter frontend.
  - **AI Hook**: When an incoming message arrives, a background job is enqueued to PostgreSQL `SKIP LOCKED` job queue for the AI Assistant to generate a draft reply or perform triage (Work Triage capability).

  ### Mobile UX Flow (375px First)
  - The Inbox screen acts as a "Work Triage" feed.
  - Tapping a conversation opens a full-screen chat view.
  - AI-drafted replies are highlighted in a translucent glass container just above the keyboard input, allowing the owner to tap "Send Draft" or edit.

  ## Implementation Prompt
  **To the Implementer:**
  Your mission is to establish the foundation of the native OHC Omnichannel messaging system in Rust, completely retiring any external dependency on Chatwoot.
  1. Define the SQL schemas and migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring strict Row-Level Security with `tenant_id`.
  2. Implement the core Rust data models and DB access functions using `sqlx`.
  3. Create the basic CRUD gRPC or REST (Axum) API endpoints for these entities.
  4. Ensure 100% unit test coverage for the Rust models and handlers.
  5. The AI Assistant should be designed to plug into this message flow (e.g., via background worker hooks when a new message is created).

  Do not implement the channel integrations (e.g. WhatsApp, Instagram) yet. This task is strictly for the core messaging data models and internal API.

  **Acceptance Criteria:**
  - Database migrations for the 4 core entities are merged.
  - Rust models and CRUD services are implemented.
  - Tests prove multi-tenant data isolation.
  - Zero dependencies on Chatwoot exist.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, core-architecture, native-chat]
assignees: []
