issue_title: "Architecture: Native Rust Omnichannel Chat System (Legacy Chat System Replacement)"
issue_description: |
  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Legacy Chat System as an external third-party service is being 100% RETIRED from the OneHumanCorp (OHC) stack to ensure strict multi-tenant isolation, zero-trust security, and seamless integration with our native AI Work Assistant capabilities. OHC requires a high-performance, native Rust-based omnichannel chat engine built directly into `onehumancorp/mono`.

  ## Research Report
  - **Competitor Analysis (Legacy Chat System)**: Legacy Chat System utilizes a heavy Ruby on Rails architecture with Postgres, Redis, and ActionCable for WebSockets. Its core data model revolves around `Accounts` (Tenants), `Inboxes`, `Conversations`, `Messages`, and `Contacts`. A `Conversation` tracks state like `status`, `agent_last_seen_at`, and `contact_last_seen_at`.
  - **Scaling Opportunities**: A Rust-based implementation leveraging `tokio` and `axum` (with `axum-tungstenite` for WebSockets) will significantly reduce memory overhead and latency compared to Rails ActionCable.
  - **Integration Gaps**: By bringing this natively into OHC, our AI Assistant (e.g., The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph, and proactively drafts complete, accurate responses directly in the owner's unified feed without crossing external webhook boundaries.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    INBOX ||--o{ CONVERSATION : contains
    INBOX }|--|| CHANNEL_ADAPTER : uses
    CONVERSATION ||--o{ MESSAGE : contains
    CONVERSATION }|--|| CONTACT : involves
    TENANT ||--o{ CONTACT : owns
  ```
  ### Multi-Tenant Data Model & Invariants
  - **Data Entities**: `Tenant`, `Inbox`, `ChannelAdapter` (Web, IG, WA, Email), `Contact`, `Conversation`, `Message`.
  - **Invariants**: Every database query MUST filter by `tenant_id`. Row-Level Security (RLS) policies must enforce `tenant_id` validation. Redis lock keys for concurrent message handling must follow `ohc:lock:{tenant_id}:conversation:{conversation_id}`.

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: The AI reads unassigned `Conversation` streams. Upon a new incoming `Message`, a background task is enqueued to evaluate context and auto-draft a reply (marked as `ai_draft` for owner approval).

  ### Mobile UX Flow (375px First)
  - **Inbox View**: Clean, Apple/Ubiquiti-style list view. Unread indicators are bold. Translucent glass sticky header showing current filter (e.g., "All Unread").
  - **Conversation View**: Native keyboard support. Touch targets for "Approve AI Draft" or "Send Payment Link" at least 44x44px.
  - **Offline Capability**: Conversations cache locally via Flutter/PWA local storage. Outbound messages are queued locally and synchronized with the Rust backend upon reconnection.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational data models, Rust `axum` APIs, and Flutter frontend views for the Native OHC Chat System.
  **Acceptance Criteria**:
  1. Define Rust structs and Postgres schema (with RLS) for `Inbox`, `Conversation`, `Message`, and `Contact` reflecting Legacy Chat System's core capabilities.
  2. Implement an `axum` WebSocket route `/ws/chat` that authenticates via SPIFFE/SPIRE and streams incoming messages.
  3. Create a Flutter 375px-optimized "Inbox" screen showcasing the translucent glass UI and unified message feed.
  4. Integrate the UI with the real API (ZERO mock data allowed in the UI codebase). Ensure empty states are truthful.
  5. E2E Playwright tests must cover: navigating to the inbox, viewing a conversation, and sending a message (asserting WebSocket broadcast).

  **Scope**: Large
  **Priority**: P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
