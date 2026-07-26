issue_title: Implement Native Rust Omnichannel Chat System to Replace Chatwoot
issue_description: "\n## Problem Statement\nOHC currently lacks a unified, multi-tenant\
  \ omnichannel chat engine to coordinate messages (DMs, SMS, WhatsApp, Web, Email).\
  \ Relying on external services like Chatwoot breaks the goal of having a fully integrated,\
  \ transparent, and multi-tenant AI work assistant for business owners. Owners (like\
  \ Maya the Baker or Carlos the Handyman) need a 375px mobile-first inbox that groups\
  \ all incoming customer communication into actionable workflows.\n\n## Research\
  \ Report\nThe Chatwoot architecture relies on heavy Ruby on Rails and PostgreSQL,\
  \ using ActionCable for WebSockets.\nOur research into Chatwoot (source cloned and\
  \ audited locally) highlights the core entities needed:\n- **Inboxes / Channels:**\
  \ Adapters for Web Widget, WhatsApp Cloud, Email, FB Messenger.\n- **Conversations\
  \ & Messages:** Unified thread models with attachments and agent-handoff metadata.\n\
  - **Contacts:** Omni-channel customer profiles.\n- **WebSocket PubSub:** Real-time\
  \ push for new messages, typing indicators, and presence.\n- **Macros/SLA:** Automation\
  \ rules.\n\nTo build this natively within OHC in a highly scalable way, we should\
  \ implement a Rust-based system leveraging `axum` (for APIs and WebSockets), `tokio`\
  \ (for concurrency), and PostgreSQL (with Row-Level Security for multi-tenancy).\n\
  \n## Design Doc\n\n### Architecture Diagram\n```mermaid\ngraph TD;\n    Client[Mobile/Web\
  \ Client] --> API[Rust API Server - axum]\n    Client --> WS[Rust WebSocket Server\
  \ - axum/tokio-tungstenite]\n    API --> DB[(PostgreSQL with RLS)]\n    API -->\
  \ Auth[Identity & Auth - SPIFFE/SPIRE]\n    API --> ChannelAdapters[WhatsApp, Email,\
  \ Web, SMS]\n    WS --> PubSub[Redis Pub/Sub]\n    API --> PubSub\n    DB --> Agents[AI\
  \ Assistants - Operations, CS, Sales]\n    Agents --> API\n```\n\n### Mobile UX\
  \ Flow (375px First)\n1. **Inbox View**: Unified list of active conversations with\
  \ clear channel icons (e.g., WhatsApp, Web). Unread badges prominently displayed.\n\
  2. **Conversation View**: Sticky header with customer name and tags. Scrollable\
  \ message history. Native mobile keyboard support.\n3. **AI Drafts**: Suggestion\
  \ chips above the text input showing AI-generated replies.\n4. **Action Menu**:\
  \ Bottom sheet for creating orders, booking appointments, or requesting payments\
  \ directly from the chat.\n\n### AI Agent Integration Points\n- **Customer & Relationship\
  \ Assistant**: Listens to the PubSub channel for new incoming messages. Uses LLM\
  \ to draft context-aware replies and saves them as `draft` messages in the database.\n\
  - **Work Triage**: Analyzes conversation intent and tags conversations (e.g., \"\
  Urgent\", \"Lead\", \"Support\") or creates tasks in the Operations queue.\n\n###\
  \ Key Design Decisions\n- **Rust for Performance**: Use Rust for the chat backend\
  \ to handle high concurrent WebSocket connections with low latency and low memory\
  \ footprint.\n- **PostgreSQL RLS**: Enforce tenant isolation at the database level\
  \ using Row-Level Security (`tenant_id`).\n- **Unified Message Schema**: Store all\
  \ messages in a single table with a `channel_type` enum to simplify queries and\
  \ AI context retrieval.\n- **Event-Driven Architecture**: Use Redis Pub/Sub to decouple\
  \ the API from the WebSocket server and AI workers.\n\n## Implementation Prompt\n\
  **Goal**: Implement the core Rust backend for the native OHC omnichannel chat system,\
  \ replacing Chatwoot.\n**Persona**: Maya (Baker) needs to see Instagram DMs, WhatsApp\
  \ messages, and Web Chat inquiries in one unified inbox on her iPhone.\n\n**Acceptance\
  \ Criteria**:\n1. Implement the Rust API endpoints (using `axum`) for managing Inboxes,\
  \ Contacts, Conversations, and Messages.\n2. Implement WebSocket support for real-time\
  \ message delivery to clients.\n3. Define the PostgreSQL database schema (with RLS\
  \ enabled) for the chat entities.\n4. Ensure 100% unit test coverage for the new\
  \ Rust modules.\n5. Provide a Playwright E2E test that simulates a user sending\
  \ a message via the Web Widget and the owner receiving it in the unified inbox UI.\n\
  6. Hide all technical complexity behind a clean, 375px-optimized mobile UI (using\
  \ OHC Premium Tokens).\n\n**Priority**: P0 (critical)\n**Estimated Scope**: Large\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
