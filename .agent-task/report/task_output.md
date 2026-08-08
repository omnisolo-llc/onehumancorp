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

  ## Design Doc
  ### High-Level Architecture
  1.  **Database Entities (PostgreSQL with RLS)**
      -   `tenant_inboxes`: Represents a channel (e.g., "Maya's Instagram", "Support Email").
      -   `contacts`: The external customer.
      -   `conversations`: A thread linking a contact and an inbox.
      -   `messages`: Individual messages within a conversation.
  2.  **API Layer (Rust Axum)**
      -   `POST /api/v1/webhooks/meta`: Ingests incoming Instagram/WhatsApp messages.
      -   `GET /api/v1/conversations`: Fetches the unified inbox for the owner.
      -   `POST /api/v1/conversations/{id}/messages`: Owner or AI sending a reply.
      -   `GET /api/v1/ws`: WebSocket endpoint for real-time updates.
  3.  **UI/UX (Tauri Desktop App / Mobile First)**
      -   A 375px-optimized "Unified Inbox" view.
      -   Conversations list with unread indicators and "AI Draft Ready" badges.
      -   Chat detail view showing message history.
      -   Input area with "Send" and "Approve AI Draft" actions.
      -   Visual style: OHC Premium Token library, clean Apple/Ubiquiti style, translucent materials.

  ## Implementation Prompt
  **User Facing Outcome**: When an owner (e.g., Maya) opens OHC on her phone, she sees a "Messages" feed. This feed contains DMs from Instagram and inquiries from her website widget. She can tap a message, see a reply already drafted by the OHC AI, and tap "Approve & Send".

  **Critical User Journey (CUJ)**:
  1. Customer sends a message via Instagram DM.
  2. OHC Rust backend receives the Meta webhook, normalizes the message, and saves it to Postgres.
  3. OHC AI Agent triggers, reads the message, and drafts a reply based on Maya's business context.
  4. The Tauri desktop/mobile app receives a WebSocket event and updates the Inbox badge.
  5. Maya opens the app, taps the conversation, reads the AI draft, and taps "Approve & Send".
  6. The Rust backend dispatches the approved message back to Meta via their Send API.

  **Acceptance Criteria**:
  - A native Rust backend implementation handles the core Inbox/Conversation/Message schema and API endpoints.
  - Zero reliance on the Chatwoot external service.
  - Unit tests provide 100% coverage for the new Rust modules.
  - A Playwright E2E test validates the CUJ from message ingestion to owner approval in the UI.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
