issue_title: "Architect & Implement Native Rust Multi-Tenant Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  **Problem Statement:** OHC currently relies on Chatwoot as an external dependency for omnichannel messaging. This violates the new engineering standard which mandates a 100% native Rust implementation. We need a high-performance, multi-tenant conversational engine integrated directly into `onehumancorp/mono` that supports Web, Instagram, WhatsApp, and SMS, eliminating the operational overhead of a separate Ruby-on-Rails service.

  **Research Report:**
  Benchmarking against the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals the following core requirements for parity:
  - **Data Models:** Deeply linked `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, and `Agent` entities.
  - **Channel Adapters:** Abstracted modules handling vendor-specific webhooks (e.g., Twilio for SMS, Meta Graph API for IG/WhatsApp).
  - **Real-time Delivery:** An event-driven WebSocket server (`ApplicationCable` / `RoomChannel` equivalent) to broadcast presence, typing indicators, and message events to the frontend.
  - **Automation:** SLA policies, macros, and AI auto-responders that trigger on message creation.

  **Design Doc:**
  - **Architecture:**
    - A new Rust crate `ohc_chat_engine` containing the core logic.
    - **Database:** PostgreSQL schema utilizing Row-Level Security (RLS) on `tenant_id` for strict isolation.
      - Tables: `chat_inboxes`, `chat_conversations`, `chat_messages` (with JSONB `content_attributes` and `external_source_ids`), `chat_contacts`.
    - **WebSocket Server:** Axum + Tokio WebSockets to handle real-time pub/sub, replacing Chatwoot's ActionCable. Redis Pub/Sub for cross-node broadcasting.
    - **Agent Integration:** AI agents hook into the `message.created` event stream via the AI Job Queue (PostgreSQL `SKIP LOCKED`) to draft responses automatically.
  - **UI Wireframes/Flow (Mobile 375px first):**
    1. Unified Inbox view natively rendered in the OHC Flutter app.
    2. Conversations feed displaying messages from all channels with clear source icons.
    3. AI drafting interface integrated directly above the compose bar.

  **Implementation Prompt:** Implement the core data models (`Inbox`, `Conversation`, `Message`, `Contact`) and the database migration for the native Rust chat engine. Include strict multi-tenant RLS policies. Implement the initial REST API endpoints for creating Inboxes and sending/receiving basic text messages, ensuring 100% unit test coverage. This is the foundation upon which WebSocket channels and external adapters will be built.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
