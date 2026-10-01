outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: OHC Custom Rust Omnichannel Chat Engine

  ## Problem Statement
  OHC requires a native, high-performance omnichannel chat system. The previously used external dependency, Chatwoot, has been 100% RETIRED per engineering standards. Small-business owners and operators (like Maya, Carlos, and Priya) need a unified inbox that brings together Instagram DMs, web chat, email, and WhatsApp without relying on an external SaaS. Relying on an external service creates latency, complicates the auth/tenant boundary, and prevents tight integration with OHC's AI agents.

  ## Research Findings & Competitor Audit
  - **Market Landscape**: Tools like Chatwoot, Intercom, and HubSpot provide omnichannel inboxes. They normalize messages from multiple channels (WhatsApp, Instagram, Email, Web Widget) into a single agent view.
  - **Chatwoot Source Code Audit**: Investigating the Chatwoot repository (`https://github.com/chatwoot/chatwoot`) reveals its core architecture:
      - **Channel Adapters**: Independent modules that listen to webhooks from providers (Meta for WhatsApp/IG, Twilio/Vonage for SMS, Postmark/Sendgrid for Email) and transform them into a standard internal `Message` model.
      - **Conversations & Contacts**: A unified data model linking an external `contact_inbox` identity to a central `Conversation`.
      - **Real-time WebSockets**: Action Cable (Ruby) pushes new messages instantly to the frontend.
      - **Agent Routing & SLA**: Rules engines that auto-assign conversations and flag SLA breaches.
      - **Web Widget**: An embeddable JS snippet that provides the live chat interface for website visitors.
  - **The OHC Gap**: OHC currently lacks a native Rust implementation of these features. To achieve the "Assistant-First Shell" where the AI Assistant triages work, the messaging engine must be embedded natively within OHC's backend so the AI can act on messages synchronously.

  ## Recommended Agentic Solution
  We must build a native Rust microservice (or module within `onehumancorp/server`) that provides a 100% feature-parity replacement for Chatwoot.
  - **Unified Inbox API**: Rust endpoints and a Postgres schema (with row-level security for tenant isolation) to store Contacts, Inboxes, Conversations, and Messages.
  - **Channel Webhook Ingestion**: Fast Rust Axum handlers to receive webhooks from Meta (WhatsApp/Instagram) and Email providers, normalizing them.
  - **Real-time Sync**: Using WebSocket (or Server-Sent Events) via Rust Axum to push new messages to the Tauri desktop app and Next.js legacy app.
  - **AI Agent Integration**: Before a message hits the human inbox, it routes through the OHC AI Job Queue. The Customer Assistant agent drafts a reply or extracts context (e.g., parsing a cake order from Maya's Instagram DM).

  ## Current Implementation Status
  After auditing the codebase, it is clear that **a custom Rust omnichannel chat system is already implemented**.
  - `src/server/api/omnichannel_webhook.rs` exists and has `resolve_identity` checking `customer_identities` and `customers` to insert into `omni_inbox_messages` and `inbox_messages`.
  - `src/server/services/inbox/service.rs` manages `UnifiedThread`, `UnifiedMessage`, and `UnifiedTriageAction` mapping.
  - WebSockets (via axum ws) are available in `src/server/api/sync.rs` and `src/server/api/ws_compression.rs`.
  - `src/e2e/playwright/omnichannel_approval.mock-contract.ts` and `src/e2e/playwright/omnichannel_unified_inbox.mock-contract.ts` demonstrate that the "owner 1-tap approve an omnichannel draft" CUJ is working natively without Chatwoot.
  - 100% replacement for Chatwoot's API seems to be in place with native Axum Rust Webhooks, `customers`, `inbox_messages`, and `UnifiedThread` storage.

  Therefore, no new implementation is needed. The feature described in the issue is already complete.
issue_priority: "P0"
issue_category: "backend"
issue_type: "feature"
issue_label: ""
assignees: []
