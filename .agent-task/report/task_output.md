issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**:
  OneHumanCorp (OHC) currently lacks a native, high-performance omnichannel chat system. The original plan to rely on Chatwoot as an external dependency has been explicitly retired. OHC requires a custom, native Rust implementation that matches Chatwoot's core capabilities (omnichannel data models, controllers, channels, WebSocket real-time messaging, and inbox architecture) while integrating deeply into our multi-tenant SaaS architecture. This ensures full ownership, better performance, seamless integration with our AI assistants, and strict adherence to our mobile-first, zero-trust design principles without relying on a bulky third-party service.

  **Research Report**:
  - **Chatwoot Architecture Audit**: Analyzed the Chatwoot source code (`https://github.com/chatwoot/chatwoot`). Key components identified:
    - Models: `Account`, `User`, `Inbox`, `Conversation`, `Message`, `Contact`, `AgentBot`.
      - `Conversation` contains `status`, `priority`, `last_activity_at`.
      - `Message` contains `content`, `content_type`, `message_type`, `private`.
      - `Contact` contains `identifier`, `email`, `phone_number`.
      - `Inbox` contains `channel_type`, `enable_auto_assignment`.
    - Channels: Adapters for web widgets, email, and social integrations.
    - Real-time: WebSocket implementation for immediate message delivery.
    - AI Integration: AgentBots that can hook into the conversation flow.
  - **OHC Architecture Fit**: A native Rust port will fit beautifully into our `src/server` ecosystem, utilizing PostgreSQL (with row-level security for tenant isolation via `tenant_id`), Redis (for Pub/Sub and fast state like presence), and gRPC/REST APIs.
  - **Existing Schema Audit**: The OHC database already has migrations like `unified_threads` and `unified_messages` (in `150_unified_inbox_triage.sql`), `customers` (`120_customers_table.sql`), `omni_inbox_messages` (`031_c_omni_inbox_messages.sql`), etc. The new chat system must coalesce these fragmented tables or introduce a definitive `inbox`, `conversation`, `message`, and `contact` structure mimicking Chatwoot while mapping correctly to OHC's `tenant_id` and existing `customer` tables.
  - **Competitive Landscape**: Systems like Zendesk, Intercom, and Chatwoot separate the "Inbox" (where agents work) from the "Channels" (where messages originate). OHC's version must blend this with the "Owner Work Assistant" paradigm—messages shouldn't just be tickets; they should be actionable items in the owner's triage feed.

  **Design Doc**:
  - **Architecture Diagram (Mental/Descriptive for now)**:
    - **Client**: Flutter UI connecting via WebSockets (for real-time chat) and REST (for historical data and actions).
    - **Gateway/API Layer**: Rust Actix/Axum handlers terminating WebSockets and HTTP requests.
    - **Service Layer**: Omnichannel router. Takes incoming messages from various channels (Web Widget, Email, API), standardizes them into `Message` entities tied to `Conversations` and `Contacts`.
    - **AI Integration**: The `Work Triage` and `Customer & Relationship Assistant` agents hook into the conversation lifecycle (via Redis queues or internal events) to auto-draft replies or create tasks.
    - **Data Store**: PostgreSQL (Conversations, Messages, Inboxes, Contacts) with strict `tenant_id` RLS. Redis for real-time presence and pub/sub broadcasting to connected WebSocket clients.
  - **UI/UX Flow (375px First)**:
    - **Owner Inbox View**: A unified list of conversations, distinct from generic tasks but integrated into the daily feed. Translucent glass style headers, clear unread indicators.
    - **Chat View**: Standard messaging interface. Bottom input area with native keyboard support. AI-suggested replies appear above the input area.
    - **Customer Web Widget**: A lightweight, embeddable React/vanilla JS widget (or just a clean Flutter web view) for customers to initiate chats.
  - **Data Models (High Level)**:
    - `Inbox`: Configuration for a channel (e.g., "Website Chat", "Support Email").
    - `Conversation`: Links a `Contact` (using OHC's existing `customers` table or a new one), an `Inbox`, and an `Assignee` (User or AI).
    - `Message`: The individual chat bubble. Supports text, attachments, and structured templates. Matches `unified_messages` or similar.
    - `Contact`: The external user interacting with the business.
  - **Key Design Decisions**:
    - **Rust Native**: Maximum performance, safe concurrency for WebSockets, unified codebase.
    - **Event-Driven**: Every new message fires events that AI agents can listen to (for auto-reply, triage, summarization).
    - **Strict Isolation**: Everything is scoped by `tenant_id`.

  **Implementation Prompt**:
  *Objective*: Implement the core data models and service layer for the new native Rust Omnichannel Chat System in OHC, replacing any conceptual reliance on Chatwoot, mapping it to existing OHC structures or introducing a cohesive set.
  *CUJ (Critical User Journey)*: As an owner (like Maya), I want to see a new message from a customer in my unified inbox, and I want my AI assistant to draft a suggested reply based on context, so I can respond instantly without typing from scratch.
  *Acceptance Criteria*:
  1. Define Rust structs and Diesel/SQLx schemas for `Inbox`, `Conversation`, `Message`, and `Contact` (or reconcile with `unified_threads`/`unified_messages`). Ensure `tenant_id` is present on all for RLS.
  2. Implement basic CRUD operations for these models.
  3. Create a WebSocket handler stub in `src/server` that can accept connections, associate them with a user/tenant, and broadcast mock messages.
  4. Write comprehensive unit tests for the data models and service logic (100% coverage requirement).
  5. Note: Do not build the full UI yet; focus on the backend foundation.

  **Priority**: P0 (Critical - foundational architecture for core capabilities)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
