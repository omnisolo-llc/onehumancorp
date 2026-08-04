> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.
issue_title: "Native Rust Omnichannel Chat: Feature Replication from Chatwoot"
issue_description: |
  ## Problem Statement
  OHC recently removed its dependency on the external Chatwoot service in favor of a native omnichannel chat implementation (`docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`). While the foundation (PostgreSQL, some REST paths, Next.js UI) is present, it lacks the fully mature multi-channel ingestion, robust WebSocket real-time updates, AI-first automated triage, and SLA enforcement that Chatwoot provided.
  For Maya (baker managing IG DMs), Carlos (handyman handling SMS quotes), and Priya (boutique owner managing FB/Email inquiries), the unified inbox must provide seamless, real-time aggregated communication without losing the reliability they expect. OHC needs a native Rust architecture that replicates Chatwoot's proven domain models and channel adapters, but enhanced with our Zero-Trust multi-tenant guarantees and KAIROS AI orchestration.

  ## Research Report
  ### Context & Findings
  1.  **Current State:** Chatwoot has been fully removed from the deployment graph. We have a native `/inbox` foundation, but the `src/server/integrations/chatwoot/` has been purged, leaving a gap in mature channel adapters (WhatsApp, IG, FB, SMS, Email).
  2.  **Chatwoot Source Audit (`/tmp/chatwoot/app/models/`):**
      - **Core Entities:** `Conversation`, `Message`, `Inbox`, `Contact`, `Channel::*` (Api, Email, FacebookPage, Instagram, Line, Sms, Telegram, WebWidget, Whatsapp).
      - **Relationships:** `Account` (tenant) -> `Inbox` (queue) -> `Conversation` (thread) -> `Message` (payload).
      - **Real-time:** ActionCable/WebSockets for pushing `conversation.created`, `message.created` events to the UI.
  3.  **OHC Goal:** We must design a matching native Rust microservice architecture within `onehumancorp/mono` (e.g., under `src/server/ohc/domain/inbox` and `src/server/ohc/domain/channels`) that implements this omnichannel flow, strictly isolated per `tenant_id` and integrated with our existing AI background workers for auto-drafting and triage.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Customer Channels: IG, WA, SMS, Web] -->|Webhook / API| B(Omnichannel Gateway - Rust)
      B --> C{Channel Adapters}
      C --> D[Inbox Service]
      D --> E[(PostgreSQL: omni_inbox_messages)]
      D --> F[Redis Pub/Sub]
      F --> G(WebSocket Server)
      G --> H[Flutter/PWA Operator UI]

      D --> I(AI Triage Worker - KAIROS)
      I --> J[Draft / Auto-Reply]
  ```

  ### Data Model (Rust / PostgreSQL)
  Strict row-level security (RLS) enforcement on all tables using `tenant_id`.
  - `omni_inboxes`: `id`, `tenant_id`, `name`, `channel_type`, `config (JSONB)`
  - `omni_conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open/resolved), `snoozed_until`
  - `omni_messages`: `id`, `tenant_id`, `conversation_id`, `content`, `message_type` (incoming/outgoing), `status` (sent/delivered/read)
  - `omni_contacts`: `id`, `tenant_id`, `identifier`, `custom_attributes (JSONB)`

  ### Mobile UX Flow (375px)
  - **Screen 1 (Triage Feed):** A unified list of open conversations. Badges indicate AI-drafted replies vs. needing manual attention.
  - **Screen 2 (Thread View):** Standard chat interface. AI suggested response pinned above the keyboard. One-tap to "Approve & Send".
  - **Screen 3 (Customer Context):** Swipe left in the thread to reveal contact info, past orders, and AI-generated summary of the relationship.

  ### AI Agent Integration
  - **Triage Agent:** Subscribes to `message.created` events for new incoming messages. Categorizes urgency and intent.
  - **Drafting Agent:** Reads conversation history and `omni_contacts` context to generate a suggested reply, stored as a draft message pending operator approval.

  ## Implementation Prompt
  Implement the native Rust backend and Next.js/Flutter frontend for the Omnichannel Unified Inbox, replacing the deprecated Chatwoot dependency.
  1.  **Backend (Rust):** Implement the core data models (`Inbox`, `Conversation`, `Message`, `Contact`) under `src/server/ohc/domain/inbox/`. Create REST API endpoints for fetching conversations and sending messages. Implement strict multi-tenant isolation using the `tenant_id` context.
  2.  **Channel Adapters:** Build the first native channel adapter for the **Web Widget** (matching Chatwoot's `Channel::WebWidget` concept) to allow initial E2E testing without external vendor keys.
  3.  **Real-time:** Implement WebSocket broadcasting for new messages to ensure the UI updates instantly.
  4.  **Frontend (Next.js/Flutter):** Update the `/inbox` UI to consume the new Rust APIs. Ensure the 375px mobile view is prioritized, featuring a unified conversation list and a thread view with AI draft suggestions.
  5.  **Verification:** Write exhaustive Playwright E2E tests covering: incoming message creation via API -> UI real-time update -> operator reply -> outgoing message API call. Mock external API calls where necessary using repository-approved local adapters, but do NOT mock the internal Rust APIs or PostgreSQL. Ensure 100% Rust unit test coverage. All Bazel tests (`bazelisk test //...`) MUST pass.

  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
