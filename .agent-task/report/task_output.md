issue_title: "[Platform] Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OHC requires a unified, high-performance omnichannel inbox that handles Instagram DMs, webchat, SMS, and WhatsApp for business owners like Maya and Carlos. Currently, we lack a native Rust chat architecture, which blocks the AI from acting autonomously across communication channels. Chatwoot has been 100% RETIRED as a dependency, so we must build its core features natively.

  ## Research & Benchmarking (Chatwoot)
  Analysis of Chatwoot (`db/schema.rb`) highlights core entity isolation:
  - **Conversations**: Tied to `account_id` (tenant) and `inbox_id`. Tracks `status`, `assignee_id`, `contact_id`, and `sla_policy_id`.
  - **Messages**: Belongs to `conversation_id`, polymorphic `sender_type` (Contact vs User/AgentBot), tracks `message_type` (incoming/outgoing/template), and JSON `content_attributes`.
  - **Inboxes**: Organizes channels (`channel_type`: Email, WebWidget, API, FBPage) for a tenant.
  - **Contacts**: Omnichannel identity, storing `identifier` and `custom_attributes`.

  ## Architecture & Design Doc

  ### 1. Data Model (PostgreSQL + Row-Level Security)
  Entities mapped for OHC:
  - `ohc_chat_inboxes`: `tenant_id`, `id`, `name`, `channel_type`, `config` (JSONB)
  - `ohc_chat_contacts`: `tenant_id`, `id`, `name`, `email`, `phone`, `avatar_url`
  - `ohc_chat_conversations`: `tenant_id`, `id`, `inbox_id`, `contact_id`, `status` (open, snoozed, resolved), `assignee_id` (null for AI pool)
  - `ohc_chat_messages`: `tenant_id`, `id`, `conversation_id`, `sender_type` (owner, customer, ai_agent), `sender_id`, `content`, `attachments` (JSONB)

  *All tables enforce multi-tenancy via RLS.*

  ### 2. Rust API & Real-time Layer
  - **HTTP Endpoints**: Axum REST APIs for `GET /api/v1/inboxes`, `POST /api/v1/conversations/:id/messages`
  - **Real-time (WebSockets)**: `tokio-tungstenite` integrated with Valkey/Redis PubSub. Topics: `tenant:{tenant_id}:inbox:{inbox_id}`.
  - **AI Hook**: The `Operations Assistant` agent sub-system subscribes to new `ohc_chat_messages`. If no human assignee exists, it generates drafts or direct replies via background workers.

  ### 3. Mobile-First UX Flow (375px)
  - **Work Triage Hub**: Replaces a generic "Inbox" tab. Shows a unified feed of "Urgent Messages" and "Needs Reply".
  - **Conversation View**: Clean chat bubbles. Persistent bottom input bar with "AI Draft" vs "Type Manually" toggle.
  - **Customer Card**: Swiping left on a chat reveals the Contact's history, active orders, and notes.

  ## Implementation Prompt
  Implement the core database schema and Axum backend for the native Rust omnichannel inbox. Create `ohc_chat_inboxes`, `ohc_chat_contacts`, `ohc_chat_conversations`, and `ohc_chat_messages` tables with strict RLS on `tenant_id`. Implement standard CRUD routes under `src/server/api/chat/`. Build the basic WebSockets pub/sub using Valkey. No UI implementation yet.

  Acceptance Criteria:
  - DB schema deployed and migration scripts added.
  - CRUD API routes for inboxes and messages exist and require auth.
  - 100% test coverage for the API layer.
  - Playwright E2E test verifying API endpoints (can be headless API calls in E2E).

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
