issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) has officially retired Chatwoot as an external third-party service/dependency for handling omnichannel customer support. Currently, the platform lacks a high-performance, multi-tenant omnichannel messaging engine built natively in Rust. This creates a critical architectural gap for owners like Maya (Instagram DM custom cake orders), Carlos (customer SMS quote requests), and Nora (client project communications) who rely heavily on seamless, unified inbox communications managed by AI agents and operators.

  ## Research Report
  Our competitive and architectural audit of the legacy Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals several core data models and capabilities required for full feature parity:
  - **Conversations & Messages:** The core entities managing the lifecycle of customer interactions (`conversation.rb`, `message.rb`).
  - **Inboxes & Channels:** Multi-channel adapters routing messages from Email, Web Widget, SMS, and WhatsApp into unified tenant inboxes.
  - **Agents & Routing:** Assignment policies, working hours, and intelligent routing between AI agents (like the OHC Customer & Relationship Assistant) and human operators.
  - **Real-time Layer:** WebSocket connections and pub-sub mechanisms to instantly synchronize state across mobile and desktop clients without polling.
  - **Multi-tenant Data Security:** Strict tenant isolation leveraging PostgreSQL Row Level Security (RLS).

  Unlike Chatwoot, OHC's implementation will tightly integrate with our AI department coordination (e.g., the Work Triage AI automatically categorizing incoming leads) and use a distributed locks pattern (Redis Redlock) for scaling.

  ## Design Doc
  ### High-Level Architecture (Native Rust Implementation in `onehumancorp/mono`)
  - **Database Models (PostgreSQL + RLS):**
    - `conversations` (tenant_id, id, status, channel_id, assigned_agent_id)
    - `messages` (tenant_id, id, conversation_id, sender_type, content, created_at)
    - `inboxes` (tenant_id, id, name, channel_type)
    - `contacts` (tenant_id, id, name, identifier)
  - **Rust Microservices/Modules:**
    - `chat_engine::inbox`: Handles routing, assignment logic, and channel aggregation.
    - `chat_engine::websocket`: Built using Axum WebSockets and Redis pub/sub to push events (new messages, typing indicators) directly to clients.
    - `chat_engine::ai_bridge`: Hooks into the AI Job Queue (PostgreSQL `SKIP LOCKED`) to let OHC's AI draft responses or triage new conversations.
  - **Architecture Diagram (Mermaid):**
    ```mermaid
    graph TD;
        Client[Mobile/Web Client] <-->|WebSocket/REST| API[Rust API Gateway];
        API --> ChatEngine[Native Rust Chat Engine];
        ChatEngine --> Redis[Redis Pub/Sub];
        ChatEngine --> DB[(PostgreSQL RLS)];
        ChatEngine --> AIQueue[AI Job Queue];
        AIQueue --> CustomerAgent[OHC Customer Assistant];
        CustomerAgent --> DB;
    ```
  - **AI Agent Integration Points:**
    - *Work Triage Agent:* Listens for new conversation events, scores priority, and groups related tickets.
    - *Customer Assistant Agent:* Automatically drafts replies for recurring queries and surfaces contextual knowledge (past orders, CRM data).

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean, UniFi-style modular dashboard card layout listing active conversations. Unread badges are prominent. Tabular filters (All, Mine, Unassigned, AI Drafts) are swipeable.
  - **Conversation Thread:** Tapping a conversation opens a standard chat UI. AI-drafted messages appear with a distinct translucent glass styling awaiting single-tap approval.
  - **Offline/Low-Data Resilience:** Messages are queued in a local SQLite/IndexedDB cache and synchronized via CRDT/Sync mechanics when the network recovers.

  ## Implementation Prompt
  **Role:** Backend & Frontend Implementer Agents
  **Task:** Implement the core CRUD APIs, WebSocket endpoints, and database migrations for the native Rust omnichannel chat system. Build a responsive, 375px mobile-first React/Flutter interface for the unified inbox.
  **CUJ (Critical User Journey):**
  1. Owner (Maya) opens the OHC mobile app.
  2. She navigates to the Inbox and sees a new Instagram DM inquiry (synced via a mock channel adapter for testing).
  3. The OHC AI Assistant has already drafted a reply ("Yes, we do vegan cakes!").
  4. Maya taps "Approve and Send", triggering a WebSocket broadcast to instantly update the UI.
  **Acceptance Criteria:**
  - 100% unit test coverage for the new Rust `chat_engine` module.
  - Playwright E2E test covering the full flow from receiving a message to approving an AI draft.
  - No external Chatwoot dependencies or API calls used.
  - All database queries enforce tenant isolation (`tenant_id`).
  - Mobile UI works flawlessly at 375px viewport with no horizontal scrolling.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
