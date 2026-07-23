issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Implement Custom Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as a third-party omnichannel customer support and chat engine. OHC needs a native Rust implementation inside `onehumancorp/mono` that handles multi-tenant, omnichannel communication seamlessly. This replacement must ensure 100% feature parity with Chatwoot's core functionalities while providing enhanced performance, security, and tight integration with OHC’s architecture. Non-technical owners need an assistant-first unified inbox where they can seamlessly triage and reply to customers across channels (web widget, email, Facebook, WhatsApp, etc.).

  ## Research Report
  - **Context**: The `AGENTS.md` and project requirements explicitly mandate the retirement of the Chatwoot external dependency. A native Rust chat system must be built inside the monorepo (`onehumancorp/mono`).
  - **Source Code Audit**: Cloned and analyzed `https://github.com/chatwoot/chatwoot`. Key components identified in Chatwoot's Ruby on Rails architecture that need Rust equivalents:
    - `app/models/`: `account`, `inbox`, `channel`, `conversation`, `message`, `contact`, `user`.
    - `app/models/channel/`: API, Email, Facebook Page, Instagram, Line, SMS, Telegram, TikTok, Twilio SMS, Twitter Profile, Web Widget, WhatsApp.
  - **Gap**: OHC currently lacks this high-performance, multi-tenant conversational architecture natively in Rust.
  - **Competitive Analysis**: Shopify Inbox and Intercom provide similar unified inbox experiences. Our solution must be simpler for the operator (assistant-first) while supporting robust multi-channel capabilities under the hood.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - **Frontend (Flutter + PWA)**: Assistant-First Unified Inbox UI (375px mobile-first). Connects via WebSockets for real-time updates.
    - **API Layer (Go/Rust gRPC & REST)**: Handles client requests, authentication (SPIFFE/SPIRE), and tenant isolation.
    - **Real-time Engine (Rust)**: Manages WebSocket connections, presence, and event dispatching (mimicking Chatwoot's ActionCable/WebSocket).
    - **Core Services (Rust Microservices/Crates)**:
      - `Inboxes`: Manages multi-channel aggregations per tenant.
      - `Channels`: Adapters for Web Widget, Email, WhatsApp, etc.
      - `Conversations & Messages`: Core data model for threads and chat bubbles.
      - `Contacts`: CRM integration.
    - **Database (PostgreSQL)**: Row-Level Security (RLS) enforced `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, `contacts`).
    - **Cache/PubSub (Redis)**: Cross-node event pub/sub for real-time messaging.

  - **Mobile UX Flow (375px)**:
    1. **Home/Triage**: The owner sees a prioritized feed of incoming messages grouped by urgency, not just a chronological list.
    2. **Conversation View**: Tapping a message opens a clean, macOS Translucent Glass-styled chat interface.
    3. **AI Assistance**: "Customer Assistant" drafts replies automatically based on context (past orders, FAQs). Owner taps "Approve & Send" or edits.
    4. **Context Panel**: Swipe left or tap a tab to see customer details, previous orders, and notes (CRM data).

  - **Key Design Decisions**:
    - **Rust Native**: Build the core messaging engine in Rust for performance and low memory footprint.
    - **Strict Multi-Tenancy**: Every database row and cache key must include `tenant_id`.
    - **Assistant-First**: The UI shouldn't feel like a complex helpdesk (like Zendesk), but rather a smart inbox where the AI suggests actions.
    - **Swappable Channels**: Design the channel architecture using traits/interfaces in Rust so new channels (e.g., TikTok) can be added easily without touching core conversation logic.

  ## Implementation Prompt
  **Goal**: Implement the core Rust data models, database schema (with RLS), and API endpoints for the native OHC Omnichannel Chat System, replacing Chatwoot.

  **Steps for Implementer**:
  1. Define the PostgreSQL schema (using migrations) for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure `tenant_id` is present on all tables with Row-Level Security (RLS) enabled.
  2. Create the corresponding Rust structs and traits in a new `chat` or `inbox` crate/module within the monorepo.
  3. Implement the core CRUD APIs (REST/gRPC) for managing Inboxes and sending/receiving Messages within a Conversation.
  4. Implement a basic Webhook or Event system to handle incoming messages from different channels (starting with a generic API channel adapter).
  5. Ensure 100% unit test coverage for the new Rust modules.
  6. Add a basic Playwright E2E test to verify that a message can be created via API and fetched, proving the data model works.

  **Acceptance Criteria**:
  - The database schema supports conversations, messages, and inboxes with strict `tenant_id` isolation.
  - Rust APIs exist to create a conversation, add a message, and list messages for a tenant.
  - All new code has 100% test coverage.
  - No external Chatwoot dependencies are used.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
