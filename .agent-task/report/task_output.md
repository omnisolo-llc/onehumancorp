issue_title: "Native Rust Omnichannel Chat System Architecture (cw Replacement)"
issue_description: |
  ### Problem Statement
  OHC currently relies on fragmented, rudimentary chat implementations or external dependencies (like cw) which breaks our multi-tenant Zero Trust model, creates disconnected data silos, and hinders the "One Assistant" vision. Our core owner personas (e.g., Maya taking Instagram orders, Carlos receiving SMS quotes, Fatima handling WhatsApp pre-orders) need a unified, high-performance Inbox that natively ingests all channels. The current implementation in `src/server/domain/repository/omnichannel_repo.rs` and `src/server/services/chat` is too simplistic to handle cw-level omnichannel routing, macros, SLA policies, and real-time WebSocket syncing for a 375px mobile-first work command center.

  ### Research Report
  - **cw Codebase Audit**: Exhaustive review of `/app/models` in cw (Conversation, Message, Contact, Channel, Inbox). cw relies on heavily polymorphic channels (`Channel::FacebookPage`, `Channel::Whatsapp`, `Channel::WebWidget`) feeding into a unified `Conversation` and `Message` model.
  - **OHC Existing Architecture**: `src/server/services/chat/models.rs` has basic struct definitions (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`), but lacks proper polymorphic channel adapters, webhook routing, AI draft injection, and SLA tracking schemas.
  - **Competitor Insights**: Shopify Ping (Sidekick) and WeCom use unified inboxes where the AI acts as a co-pilot, drafting responses before the human sees them. This perfectly aligns with our AI capabilities (e.g., `OmniChannelService::ingest_signal` injecting drafts).

  ### Design Doc
  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CHANNEL : contains
      CHANNEL ||--o{ CONVERSATION : routes_to
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| AI_DRAFT : has_suggestion

      CHANNEL {
          uuid id
          uuid tenant_id
          string provider_type "Whatsapp, Instagram, SMS, WebWidget"
          jsonb provider_config "Credentials & Webhook secrets"
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid contact_id
          uuid inbox_id
          string status "OPEN, SNOOZED, RESOLVED"
          uuid assignee_id
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string sender_type "CONTACT, AGENT, BOT"
          string content_type "TEXT, IMAGE, TEMPLATE"
          text content
      }
  ```

  #### Mobile UX Flow (375px First)
  1. **Unified Inbox List (Home)**: Owner opens app. Sees a unified list of active conversations across all channels, sorted by SLA/Urgency. Each row shows a channel icon (Instagram, SMS), contact name, and a snippet of the latest message or AI draft.
  2. **Conversation Thread**: Tapping a row opens the thread. The AI's suggested reply is pre-filled in the composer area with a "Sparkle" icon.
  3. **Action Modals**: Swipe left on a message to instantly generate a quote/order or invoke a specific AI Agent (e.g., "Operations Assistant" to check inventory).
  4. **Low-Data Tolerance**: Uses local-first sync via `unified_ws.rs` (which already supports `Subscribe` and `Replay`).

  #### AI Agent Integration Points
  - **Work Triage Agent**: Hooks into the `Message` creation webhook. Evaluates priority and tags the conversation.
  - **Customer Assistant Agent**: Generates an `AiDraft` for incoming customer messages. The draft is pushed via WebSocket and displayed in the UI composer.

  #### Key Design Decisions
  - **Native Rust**: Replace all external cw dependencies with a native Rust implementation in `ohc-mono` (`src/server/services/chat` + `omnichannel_repo.rs`).
  - **Unified WebSocket**: Leverage the existing `src/server/api/unified_ws.rs` for real-time delivery to the Flutter/PWA client.
  - **Row-Level Security**: Every table requires `tenant_id` and strict PostgreSQL RLS.
  - **Polymorphic Channels**: Use JSONB `provider_config` on `ChatChannel` rather than separate tables for each channel type, simplifying the schema while maintaining flexibility.

  ### Estimated Scope
  Large

  ### Implementation Prompt
  **Goal:** Implement the backend domain models, repository methods, and gRPC/REST API for the Native Rust Omnichannel Chat System.
  **CUJ:** Maya (the baker) receives an Instagram DM. The system ingests it via a webhook, creates a Contact and Conversation, and triggers the Customer Assistant AI to draft a response. Maya opens her 375px mobile app, sees the unified inbox via WebSocket, taps the conversation, and hits "Send" on the AI's drafted response.
  **Acceptance Criteria:**
  1. Update `ChatChannel`, `ChatConversation`, and `ChatMessage` SQLX models to support polymorphic channel types and AI draft associations.
  2. Implement webhook ingest endpoints for at least one mock channel (e.g., SMS/Twilio) that maps into the unified models.
  3. Wire the new models into the `unified_ws.rs` system to emit real-time events (e.g., `chat:conversation:tenant-123`).
  4. 100% Rust unit test coverage for the repository layer.
  5. 100% Main branch tests passing (`bazel test //...`). No external dependencies or mocks.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
