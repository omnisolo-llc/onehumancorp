issue_title: "[Research] Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  The current platform lacks a cohesive, mobile-first, deeply integrated omnichannel customer support and chat system built natively in Rust. Relying on an external Chatwoot deployment (now 100% RETIRED) created systemic friction, data silos, multi-tenant isolation risks, and a disjointed user experience for our core owner/operator personas (Maya, Carlos, Priya, Leo, Fatima). They need a unified inbox that brings all customer interactions (Instagram DMs, website chat, WhatsApp, email) into one assistant-led flow directly within OneHumanCorp without juggling external tools.

  ## Research Report
  - **Goal**: Architect a native, high-performance omnichannel chat system replicating the core strengths of Chatwoot but tightly integrated into OHCs multi-tenant architecture and AI assistant flow.
  - **Competitor Analysis**: Evaluated Chatwoot (source code audited), Shopify Inbox, and Wix Inbox. The key differentiator for OHC must be the seamless AI triage and "assistant-first" approach where agents draft replies and orchestrate operations.
  - **Source Code Audit (Chatwoot)**: Analyzed Chatwoot`s PostgreSQL schema (Conversations, Messages, Contacts, Inboxes, Channel Adapters), ActionCable WebSocket patterns for real-time updates, and Redis-backed background jobs.

  ## Design Doc
  - **Architecture**:
    - **Language**: Rust natively inside `onehumancorp/mono`.
    - **Data Model**: PostgreSQL with row-level security (`tenant_id`). Key entities: `ohc_inboxes`, `ohc_conversations`, `ohc_messages`, `ohc_contacts`, `ohc_channel_configurations`.
    - **Real-time**: Axum WebSockets integrated with Redis Pub/Sub for horizontal scalability and fast client updates.
    - **Agent Integration**: Hook into the existing AI Job Queue (PostgreSQL `SKIP LOCKED`). When a new message arrives, the `Work Triage` agent evaluates it, and the `Customer & Relationship Assistant` agent drafts a reply.
  - **Mobile UX Flow (375px)**:
    - Unified Inbox view showing pending messages with unread indicators.
    - Conversation view with clear distinction between customer messages, AI drafts (highlighted for approval), and owner replies.
    - Bottom sheet for quick actions (create quote, book appointment) directly from the chat context.
  - **Key Decisions**:
    - Build native channel adapters (starting with Web Widget and Email) to ensure deep integration rather than relying on third-party aggregators initially.
    - Store conversation context directly alongside operational data (bookings, orders) to enable rich AI context.

  ## Implementation Prompt
  Implement the foundational data model, API endpoints (REST/gRPC), and a basic Axum WebSocket handler for the new native Rust omnichannel chat system. Ensure strict multi-tenant isolation via `tenant_id` on all tables. Create the `ohc_inboxes`, `ohc_conversations`, and `ohc_messages` tables. Implement the backend logic to receive a message, persist it, and broadcast it via WebSocket. Do not implement external provider adapters (e.g., WhatsApp) in this first pass; focus on the core internal engine and a simple internal API for testing. Ensure 100% unit test coverage.

  ## Priority: P0 (Critical - Unblocks core communication features)
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
