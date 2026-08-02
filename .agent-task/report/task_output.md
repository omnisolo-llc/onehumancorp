issue_title: "[Research] Architect Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp previously relied on Chatwoot as an external dependency for omnichannel messaging. We are retiring this dependency to bring all messaging logic in-house as a native Rust multi-tenant service. Our small business personas (like Maya the baker and Carlos the handyman) need a seamless, highly reliable inbox that unifies Instagram DMs, WhatsApp, SMS, Web Widget, and Email into a single feed. The external Chatwoot dependency broke our security models, introduced high latency, and made offline-first mobile usage impossible. We need a performant, multi-tenant Rust architecture built on PostgreSQL and Valkey (Redis) that mirrors and improves upon Chatwoot's data models and capabilities.

  ## Research Report
  - **Chatwoot Audit**: The cloned Chatwoot repository (`app/models`) reveals the core entities required: `Account` (Tenant), `Inbox`, `Channel::*` (WebWidget, API, Email, FacebookPage, Instagram, Whatsapp, Sms, Telegram, TwilioSms, Line, Tiktok, TwitterProfile), `Conversation`, `Message`, `Contact`, `ContactInbox`, `AgentBot`, and `AutomationRule`.
  - **Architecture Learnings**: Chatwoot uses polymorphic associations (`channel` belongs to an `Inbox`), a unified `messages` table linked to `conversations`, and event-driven webhook dispatches.
  - **OHC Implementation Need**: We need a strictly isolated Rust `ohc-chat` service (or a module within `ohc-core`) leveraging `SQLx` with row-level security (`tenant_id`), `axum` for API/Webhooks, and `tokio-tungstenite` for WebSocket real-time updates.
  - **Mobile-First UX**: The UI must function offline with local queueing. Agents shouldn't have to wait for a network roundtrip to see a drafted reply.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        TENANT ||--o{ INBOX : has
        TENANT ||--o{ CONTACT : has
        INBOX ||--o{ CHANNEL : configures
        INBOX ||--o{ CONVERSATION : contains
        CONTACT ||--o{ CONVERSATION : participates
        CONVERSATION ||--o{ MESSAGE : contains
        MESSAGE }|--|| TENANT : belongs_to

        CHANNEL {
            string type "WebWidget, WhatsApp, Instagram"
            jsonb credentials
        }
    ```
  - **Data Model**:
    - `conversations` (id, tenant_id, inbox_id, contact_id, status, created_at, updated_at)
    - `messages` (id, tenant_id, conversation_id, sender_type, sender_id, content, status, message_type)
    - `inboxes` (id, tenant_id, name, channel_type, channel_id)
  - **Mobile UX Flow**:
    1. 375px Viewport: Unified "Inbox" tab with unread indicators.
    2. Tap conversation: Opens full-screen chat view with sticky bottom input.
    3. Channel icon (WhatsApp, IG) clearly visible on the conversation card.
  - **AI Agent Integration**:
    - AI Agents hook into the `message.created` pub/sub event via Valkey.
    - The `AgentBot` can draft replies (status `draft`) that the human owner can approve or auto-send based on tenant `AutomationRule` configs.

  ## Implementation Prompt
  **Title**: Implement Native Rust Omnichannel Inbox Foundation
  **Goal**: Build the core Rust database schema, Axum API endpoints, and initial WebSocket hub to replace Chatwoot.
  **CUJ**: Maya receives an Instagram DM. The webhook hits OHC, creates a `contact`, `conversation`, and `message` in the DB. The WebSocket pushes the new message to Maya's mobile app. Maya types a reply, hits send, and the API queues a job to send the message back to Instagram.
  **Acceptance Criteria**:
  1. Define `up.sql` and `down.sql` migrations for `inboxes`, `conversations`, `messages`, and `contacts` with `tenant_id` RLS.
  2. Implement CRUD APIs in `src/server/services/chat/` using Axum and SQLx.
  3. Implement a basic WebSocket hub for real-time delivery to authenticated clients.
  4. Write comprehensive E2E tests verifying the full flow from incoming webhook to WebSocket dispatch.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
