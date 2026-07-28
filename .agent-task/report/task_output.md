issue_title: "Implement Native Omnichannel Chat Architecture"
issue_description: |
  **Problem Statement**
  Chatwoot, which was previously intended for omnichannel social inbox functionality, has been completely removed from the OHC architecture due to its heavyweight nature, external dependencies, and mismatch with OHC's native multi-tenant edge-first architecture (as specified in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`). OHC needs a high-performance, native Rust-based omnichannel chat engine that integrates natively with its unified inbox, PowerSync realtime sync, and multi-tenant SQLite/PostgreSQL architecture without relying on external services. The previous Chatwoot integration approach is obsolete and a new native architecture must be implemented to support Maya, Carlos, Priya, Leo, and Fatima in managing all customer communications (Instagram, WhatsApp, Email, Web Widget) seamlessly.

  **Research Report**
  - **Legacy Approach**: OHC previously used an external Chatwoot deployment, which added significant operational overhead (Redis, Sidekiq, Postgres for Chatwoot), duplicated data models, and complicated multi-tenant isolation.
  - **Current State**: Chatwoot has been fully retired (verified by `deploy/tests/no_chatwoot_residue_test.sh` and lack of Chatwoot references).
  - **Target Architecture**: As mandated by `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`, OHC requires a canonical conversation domain (Project 3) and a native delivery outbox (Project 4) built entirely in Rust (`onehumancorp/mono`), communicating via gRPC/REST.
  - **Competitor Analysis**:
    - **Shopify Inbox**: Deeply integrated, native to the platform, handles Apple Business Chat, Instagram, and web chat.
    - **Tencent Workbuddy / WeCom**: Natively bridges consumer WeChat with enterprise tools without third-party middleboxes.
    - OHC must emulate this deep native integration. A native Rust chat core ensures strict row-level security (RLS), Zero Trust compliance via SPIFFE/SPIRE, and immediate offline-first availability via PowerSync.
  - **Source Code Audit**: Audited `https://github.com/chatwoot/chatwoot` to understand channel adapters, webhooks, SLA policies, and canned responses. OHC will reimplement these features natively in Rust.

  **Design Doc**
  - **Architecture**:
    - **Core Domain**: Native Rust microservices (`server_integrations_native_chat`) implementing the `Conversation`, `Message`, `Participant`, and `ChannelAdapter` entities.
    - **Data Model**: Multi-tenant isolated SQLite (edge) and PostgreSQL (cloud).
      - `conversations` (id, tenant_id, status, channel_type)
      - `messages` (id, conversation_id, tenant_id, sender_type, content, status)
      - `channel_adapters` (id, tenant_id, provider, credentials_encrypted)
    - **Realtime**: WebSocket ingress mapped to PowerSync synchronization rules for instant mobile delivery.
    - **AI Integration**: Messages trigger the OHC AI Job Queue (PostgreSQL `SKIP LOCKED`). The Customer Support AI agent reads the thread context and drafts replies for the owner's approval.
  - **Mobile UX Flow (375px)**:
    - **Unified Inbox Screen**: A clean, unified list of active conversations across all channels (WhatsApp, IG, Web). Uses iOS-style translucent glass headers and unread badges.
    - **Conversation View**: Native chat UI. AI-drafted replies appear as suggested chips above the keyboard.
    - **Channel Setup**: A simple "Connect Instagram" button that securely stores Meta Graph API credentials in the `channel_adapters` table via envelope encryption.
  - **Key Design Decisions**:
    - **No Third-Party DB**: All chat data lives inside OHC's primary multi-tenant databases.
    - **Transactional Outbox**: Ensure reliable message delivery to external channels (Meta, Twilio) even during network partitions.

  **Implementation Prompt**
  Implement the core data models and service layer for the native OHC omnichannel chat system in Rust.
  1. Create the database schemas (or proto definitions) for `Conversation`, `Message`, and `ChannelAdapter`, ensuring strict `tenant_id` isolation.
  2. Implement the gRPC/REST service handlers for creating conversations, sending messages, and receiving webhook payloads from external channels (e.g., Meta Graph API).
  3. Integrate with the existing AI Job Queue to trigger AI reply drafting when new inbound customer messages arrive.
  4. Ensure the frontend unified inbox (Flutter/PWA) correctly consumes these new native endpoints instead of the legacy Chatwoot APIs.
  **Acceptance Criteria**: A non-technical owner like Maya can connect her Instagram, receive a DM in the OHC mobile app (375px view), see an AI-suggested reply, and send it back to the customer—all without any external Chatwoot dependency. All unit and Playwright E2E tests must pass.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
