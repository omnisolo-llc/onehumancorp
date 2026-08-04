issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot, an external service, for omnichannel customer support and chat features. The core business personas (Maya, Carlos, Priya, Leo, Fatima) need a reliable, integrated, and high-performance unified inbox that handles interactions across multiple channels (Instagram DMs, WhatsApp, SMS, Web Widget) seamlessly. Using an external dependency introduces latency, complexity, and breaks our multi-tenant isolation and data consistency models. We need a native Rust implementation of Chatwoot's core features (unified inbox, conversations, messages, channels) within `onehumancorp/mono`.

  ## Research Report
  - **Market Dynamics:** Platforms like Shopify and Wix provide robust native inboxes. A tightly integrated, low-latency omnichannel system is expected by small business operators.
  - **Chatwoot Source Code Audit:**
    - Models: `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*` (WebWidget, WhatsApp, SMS, etc.).
    - Architecture: Event-driven webhooks, WebSocket real-time messaging, omnichannel payload routing.
  - **Gap Analysis:** We have webhook endpoints and agent routing (e.g. `omnichannel_webhook`), but we lack the foundational native persistence and real-time state synchronization that Chatwoot offers.

  ## Design Doc
  **Architecture (Mermaid)**
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "has many"
      Inbox ||--o{ ChannelAdapter : "configured with"
      Inbox ||--o{ Conversation : "has many"
      Conversation ||--o{ Message : "contains"
      Conversation }|--|| Contact : "with"
      Message ||--|| Attachment : "optional"
  ```
  - **Data Model:**
    - `Inbox`: Configuration for a unified entry point, multi-tenant scoped (`tenant_id`).
    - `Conversation`: Represents a thread between a `Contact` and the business.
    - `Message`: Individual messages with types (`incoming`, `outgoing`, `template`), linked to `Conversation`.
    - `ChannelAdapter`: Stores credentials and settings for specific channels (WhatsApp, Web, Instagram).
  - **Mobile UX Flow (375px):**
    - A centralized `Omnichannel Feed` screen showing a list of `Conversations`.
    - Tapping a conversation opens a chat view with native mobile keyboard support.
    - The chat view clearly indicates the source channel via an icon.
    - Read/unread indicators and agent assignment state are visible.
  - **AI Agent Integration Points:**
    - Incoming messages trigger the `Omnichannel Dispatcher` which routes to the `Customer Success Agent` or `Operations Agent` based on context (e.g. quoting vs general inquiry).
    - Agents draft replies into the `Message` table as `outgoing` with a `draft` status, pending owner approval.
  - **Key Design Decisions:**
    - **Native Rust & Postgres:** Replicate Chatwoot's core models using native Rust structs and PostgreSQL (with row-level security per tenant).
    - **WebSockets:** Implement real-time updates for the UI via WebSocket endpoints on the API server.
    - **Multi-Tenancy:** Ensure strict isolation by embedding `tenant_id` on all tables and verifying it at the API boundary.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner (e.g., Maya) receives Instagram DMs and WhatsApp messages directly in the OHC mobile app. The AI drafts replies. Maya can view the conversation history, approve the draft, and send it back to the customer, all within OHC's native UI, without ever realizing Chatwoot was removed.

  **Acceptance Criteria:**
  1. Create Rust database models and migrations for `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`. Ensure strict multi-tenant RLS rules.
  2. Implement CRUD REST APIs for these models to support the mobile/desktop UI.
  3. Implement a WebSocket hub in the Rust API server to broadcast new messages and conversation state changes to connected clients.
  4. Refactor existing `omnichannel_webhook` routes to persist incoming data into the new native `Message` and `Conversation` models instead of routing to an external service.
  5. Ensure AI agents (Customer Success, Operations) read from and draft replies to the native `Message` model.
  6. E2E tests using Playwright must simulate incoming messages via the webhook, verify they appear in the unified inbox UI, and confirm the owner can send a manual or AI-drafted reply.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
