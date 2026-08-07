issue_title: "Implement Custom Rust Omnichannel Chat System based on Chatwoot"
issue_description: |
  # Research Report: Custom Rust Omnichannel Chat System

  ## Problem Statement
  OHC requires a robust, scalable, and fully featured omnichannel customer support & chat engine natively integrated. It must allow owners to unify messages from various channels into a prioritized feed, interact with customers, and utilize AI for automated responses without relying on third-party services like Chatwoot.

  ## Research Findings
  After analyzing Chatwoot's source code and architecture, we identified the key components required to build a native Rust chat engine:
  - **Core Channels**: WhatsApp, API, Web Widget, Email, Facebook, Instagram, Twitter, SMS, Telegram, TikTok, Line.
  - **Key Features**: Omnichannel inbox, private notes, labels, canned responses, auto-assignment, multi-lingual support, custom views, business hours, team collaboration, and AI automation.
  - **Architecture Needs**: Multi-tenant database model (row-level security in PostgreSQL), robust webhook processing, secure API handling (HMAC), and distributed queue processing for message delivery and sync.

  ## Design Doc
  - **Data Models**: Design native Rust structs and PostgreSQL schemas for unified `Inbox`, `Conversation`, `Message`, and `Contact` tables. Ensure strict `tenant_id` isolation.
  - **Channel Connectors**: Implement modular Rust traits for channel providers. Start with the most critical channels: Web Widget (WebSocket-based), WhatsApp (Meta Cloud API), and generic API channels.
  - **Webhook Engine**: Create a secure webhook ingestion endpoint to handle incoming messages from external providers, verifying signatures and managing idempotency.
  - **AI Integration**: Connect the new chat engine to OHC's existing AI Job Queue to enable automated responses and conversation summaries via the `Customer & Relationship Assistant`.

  ## Implementation Prompt
  1. Define the core multi-tenant database schema for conversations and messages.
  2. Implement the `Web Widget` channel connector, including WebSocket support for real-time messaging.
  3. Implement the `WhatsApp` channel connector, supporting the Meta Cloud API for sending and receiving messages.
  4. Develop the unified `Work Triage` inbox UI within the Flutter frontend, allowing owners to view, reply, and manage conversations.
  5. Ensure all data flows through the real application stack (no mocks) and write comprehensive E2E tests verifying the complete message lifecycle from widget/webhook to owner UI and back.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
