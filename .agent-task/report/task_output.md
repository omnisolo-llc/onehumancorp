issue_title: "[Architect] Design Native Rust Chatwoot Replacement (Omnichannel unified Inbox)"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is transitioning away from external dependencies for its omnichannel unified inbox. Currently, the architecture conceptualizes an integration with Chatwoot, but as a core requirement, Chatwoot must be **100% RETIRED**. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. Relying on external services breaks the core mandate of OHC being a unified, highly reliable platform. Small business owners (like Carlos or Maya) need real-time, consolidated messaging (Instagram DMs, WhatsApp, Email, Web Chat) without OHC relying on a brittle external system.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Architecture Assessment:** Based on a theoretical audit of `https://github.com/chatwoot/chatwoot`, the core components needed are:
    - Multi-tenant data model (Accounts, Inboxes, Contacts, Conversations, Messages).
    - Channel Adapters (Web Widget, API, Email, WhatsApp, Facebook/Instagram).
    - Real-time communication layer (WebSockets).
    - Background job processing for webhooks and email parsing.
  - **Shopify Inbox & Wix Inbox:** While integrated, these systems often lack the deep agentic AI hook-ins we envision. They are passive aggregation points.
  - **OHC Native Rust Approach:** By building this natively in Rust (using Axum, Tokio, and Postgres), OHC can achieve significantly higher performance, lower memory footprint, and, most importantly, tight integration with our "Ambassador" and "Manager" AI agents. State changes in the inbox can directly trigger state machine transitions in the OHC KAIROS orchestrator.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client_WebWidget[Web Chat Widget] -->|WebSocket / REST| OHC_Gateway[Rust API Gateway - Axum]
      Ext_WhatsApp[WhatsApp Cloud API] -->|Webhook| OHC_Gateway
      Ext_Insta[Instagram Graph API] -->|Webhook| OHC_Gateway

      OHC_Gateway -->|Route| Channel_Adapter[Native Channel Adapters]
      Channel_Adapter --> DB[(PostgreSQL Ledger)]
      Channel_Adapter --> Cache[(Valkey/Redis)]

      DB -->|Notify on Insert| Event_Bus[Rust Event Bus / Tokio MPSC]
      Event_Bus --> RealTime_Dispatcher[WebSocket Dispatcher]
      RealTime_Dispatcher --> Client_MobileApp[OHC Mobile App - 375px]

      Event_Bus --> AI_Orchestrator[KAIROS Agent Orchestrator]
      AI_Orchestrator --> The_Ambassador[The Ambassador Agent]
      The_Ambassador -->|Draft Reply| DB
  ```

  ### Mobile UX Flow (375px First)
  - **The Unified Feed:** The primary interface is not a traditional "Inbox" with folders, but a prioritized feed.
  - **Card Design:** A new message appears as an actionable card.
    - *Header:* Customer Name & Channel Icon (e.g., WhatsApp).
    - *Body:* The message snippet.
    - *Context Expansion:* Tapping reveals customer history (pulled natively from the same DB, no external API call needed).
  - **AI Drafting:** If the Ambassador agent has drafted a reply, it appears inline with a large "Approve & Send" button (min 44x44px).
  - **Offline Resilience:** The UI must cache recent conversations locally (e.g., SQLite in Tauri/Flutter) to allow viewing and drafting replies while offline, syncing when connectivity returns.

  ### Key Design Decisions
  - **Zero Chatwoot Dependency:** We will not use Chatwoot APIs, webhooks, or its data model directly. We are building the `ohc-chat` module natively in Rust.
  - **Strict Multi-Tenancy:** Every table (`contacts`, `conversations`, `messages`, `inboxes`) MUST include `tenant_id` and utilize PostgreSQL Row-Level Security (RLS).
  - **Agent-First Eventing:** The database insertion of a message must immediately broadcast to the internal event bus, waking up the relevant AI agents *before* pushing to the human owner's UI, allowing the AI to prep a draft.
  - **Idempotency:** Webhook receivers for external channels must implement strict idempotency to handle retries from Meta/WhatsApp without duplicating messages.

  # Implementation Prompt
  **User-Facing Outcome:** As an OHC owner, I receive a unified feed of customer messages from my website and Instagram natively within the OHC app. The system feels instantaneous and offline-tolerant, and my AI assistant frequently has a drafted reply waiting for my approval.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A webhook payload simulating an incoming Instagram DM hits the native OHC Rust API endpoint (`/api/v1/webhooks/instagram`).
  2. The Rust webhook handler parses the payload, verifies idempotency, and creates a `Message` and `Conversation` record in Postgres, strictly scoped to the `tenant_id`.
  3. The database insert triggers a real-time WebSocket event.
  4. A connected client (Playwright test representing the mobile app) receives the WebSocket event and updates the UI to show the new message instantly.
  5. The Ambassador agent is triggered in the background, reads the new message, and creates a "Draft" message associated with the conversation.
  6. **Acceptance:** Playwright E2E tests must demonstrate receiving the mock webhook, updating the UI via WebSocket, and displaying the AI-drafted reply card, all without relying on any external Chatwoot service. All Rust code must have 100% unit test coverage.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
