issue_title: "Architecture Design: Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement:**
  OneHumanCorp (OHC) currently lacks a native, high-performance, multi-tenant omnichannel inbox to centralize customer communications (Instagram DMs, WhatsApp, Email, Web Chat) into a single feed. Previously, Chatwoot was used as a third-party dependency, which fractured tenant data, compromised Zero-Trust SPIFFE/SPIRE isolation boundaries, and prevented deep AI agent integration natively on the data layer. A unified inbox is critical for personas like Maya (Baker) who triages Instagram custom orders, and Carlos (Field Service) who receives SMS and web leads, as they need one place to view, auto-draft replies via AI, and capture work without app-switching.

  **Research Report:**
  We audited the open-source Chatwoot codebase (Ruby on Rails). Core primitives discovered in `app/models/` include:
  *   `Account` (Tenant)
  *   `User` / `AccountUser` (Agents)
  *   `Contact` / `ContactInbox` (End customers)
  *   `Inbox` / `Channel` (Sources: WebWidget, API, Email, WhatsApp, FB Page)
  *   `Conversation` / `Message` (The core thread and items)
  *   `AutomationRule` / `AgentBot` (Automated workflows)

  Competitor Analysis (Shopify Inbox, Wix Inbox, Chatwoot): All use a centralized `Conversation` model linked to a polymorphic `ChannelAdapter`. They rely on WebSockets for real-time delivery and background workers for third-party sync. In OHC, we must design this in Rust using `tokio`, `axum` (for WebSockets), and PostgreSQL for persistence, maintaining strict `tenant_id` Row-Level Security (RLS).

  ## Architecture Design

  **1. Data Model & Invariants (Rust / PostgreSQL)**
  *   **`Tenant`**: The owner workspace.
  *   **`ChannelAdapter`**: Configuration for providers (e.g., WhatsApp Cloud API, Instagram Graph API).
  *   **`Inbox`**: Logical grouping of channels (e.g., "Support", "Sales").
  *   **`Contact`**: Unified customer profile (merged across channels based on email/phone).
  *   **`Conversation`**: A thread linking a `Contact` to an `Inbox`. Tracks status (Open, Snoozed, Closed).
  *   **`Message`**: Immutable chat message.

  *Invariants:* Every table MUST have a `tenant_id` column. Every database access MUST be wrapped in a transaction that sets `app.current_tenant_id` to enforce RLS. WebSockets must authenticate via SPIFFE/SPIRE JWTs.

  **2. AI Agent Integration**
  *   **Work Triage Agent**: Listens to the `message.created` pub/sub event via Valkey (Redis). It evaluates intent and auto-drafts a reply, linking the draft to the `Message` record.
  *   **Customer Assistant**: Updates the `Contact` profile with discovered preferences (e.g., "vegan cakes") and tags the `Conversation` appropriately.
  *   **Operations Assistant**: Parses dates/times and creates actionable Tasks or Quotes directly from the chat UI.

  **3. Mobile UX Flow (375px First)**
  *   **Unified Feed (Home)**: A single scrolling list of `Conversations`. Unread messages have a bold Translucent Glass treatment with a blue accent.
  *   **Conversation View**:
      *   Header: Customer name and context (LTV, last order).
      *   Body: Native chat bubbles.
      *   Bottom: Composer with an AI "Draft Reply" button taking prominence over the manual keyboard.
  *   **Quick Actions**: Swiping a conversation right marks it "Done" (Closed), swiping left shows "Snooze" or "Convert to Quote".

  ## Implementation Prompt (For Implementer Agents)

  **Objective**: Implement the backend Rust microservice and the database schema for the OHC Omnichannel Inbox, achieving parity with Chatwoot's core messaging flow.

  **CUJ**: A customer sends a message via a simulated Web Widget channel. The message must persist in the `messages` table, trigger a WebSocket broadcast to the connected owner (authenticated via JWT), and the UI must display the new message in real-time.

  **Acceptance Criteria**:
  1.  Define PostgreSQL schemas with RLS for `inboxes`, `contacts`, `conversations`, and `messages`.
  2.  Implement Rust CRUD endpoints and an `axum` WebSocket route for real-time delivery.
  3.  Verify the flow with at least FIVE Playwright E2E tests simulating incoming messages and agent replies.
  4.  Provide 100% backend unit test coverage for the messaging logic.
  5.  Ensure zero hardcoded or mocked mock data; everything flows through Postgres.

  ## Project Details
  *   **Priority**: P0
  *   **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
