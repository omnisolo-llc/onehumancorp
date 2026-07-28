issue_title: "Native Rust Omnichannel Chat System: Architecting OHC's Inbox"
issue_description: |
  # Problem Statement
  OHC previously relied on Chatwoot as an external service for omnichannel messaging. This violates the architectural mandate for a unified, self-contained, high-performance platform. We need a native Rust omnichannel customer support and chat engine built directly into OHC. It must support multiple channels (web widget, email, SMS, social), real-time WebSocket messaging, unified inbox data models, and deep AI agent integration (The Ambassador) to draft replies and manage context autonomously, completely replacing Chatwoot.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot (Baseline):** Offers a strong open-source model with explicit concepts for Accounts (Tenants), Inboxes (Channels), Conversations, Messages, and Contacts. It uses Rails/Postgres/Redis and ActionCable for WebSockets.
  - **Shopify/Wix:** Provide basic unified inboxes but lack deep, programmatic AI integration at the core model level; AI is often bolted on as a feature rather than an acting agent.
  - **OHC Requirement:** The new system must match Chatwoot's core data models (multi-tenant isolated) and real-time capabilities but implemented in Rust (using Axum/Tonic) with PostgreSQL and Valkey. Crucially, it must integrate seamlessly with our AI job queue so that the Customer Success Agent can instantly process incoming messages, query the Customer Graph, and draft replies before the human owner even opens the app.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Customer (Web/Mobile)] -->|WebSocket/HTTP| Gateway(Axum API Gateway)
      Webhook[External Channels (IG/WhatsApp)] -->|HTTP POST| WebhookHandler

      Gateway --> Auth[Identity & Session]
      WebhookHandler --> Auth

      Auth --> Controller(Chat Controller)

      Controller --> DB[(PostgreSQL: Conversations, Messages, Contacts)]
      Controller --> Cache[(Valkey: Pub/Sub, Presence)]

      Controller --> JobQueue(AI Job Queue)
      JobQueue --> Agent[The Ambassador Agent]

      Agent -->|Query| DB
      Agent -->|Draft Reply| Controller

      Controller -->|WebSocket Broadcast| OwnerApp[Owner Mobile/Web App]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Owner Inbox View:** Clean, translucent glass list of active conversations. Unread badges. Indicators showing if an AI draft is pending.
  - **Conversation View:** Standard chat interface. Messages clearly styled to differentiate between Customer, Owner, and AI Drafts. An AI draft appears as a distinct card with "Send" and "Edit" buttons.
  - **Customer Web Widget (Future Phase):** A lightweight, embeddable chat widget for the owner's storefront, connecting directly via WebSocket to the new Rust backend.

  ### AI Agent Integration Points
  - **Ingestion:** Every new message event triggers a lightweight job evaluating if an AI response or draft is appropriate (based on tenant settings).
  - **Contextual Drafting:** The Ambassador agent retrieves the conversation history, customer profile, and relevant business context (inventory, policies) to generate a high-quality draft.
  - **Action Approval:** The draft is saved to the database (status: `draft`) and pushed to the owner's UI. The owner approves, which transitions the message to `sent` and broadcasts it to the channel.

  ### Key Design Decisions
  - **Data Model:** Adopt a robust schema inspired by Chatwoot: `tenants`, `channels` (inboxes), `contacts` (customers), `conversations` (tied to a contact and inbox), and `messages`. All strictly tenant-isolated via RLS.
  - **Real-time:** Use Axum WebSockets and Valkey Pub/Sub for low-latency delivery of messages and typing indicators.
  - **AI-First:** The system isn't just for human-to-human chat; the AI agent is a first-class participant capable of reading, drafting, and (if configured) auto-replying.

  # Implementation Prompt
  **User-Facing Outcome:** An owner receives messages from multiple sources into a single, real-time inbox in the OHC app. They see AI-drafted replies ready for approval, eliminating the need to manually type responses for common inquiries.
  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL schema (using migrations) for `contacts`, `channels`, `conversations`, and `messages`, ensuring strong `tenant_id` isolation.
  2. Implement the core Rust service layer (models, repositories, services) for CRUD operations on these entities.
  3. Implement an Axum WebSocket handler that allows clients to connect, authenticate (tenant context), and receive real-time message events.
  4. Create an internal API/integration point where an incoming message triggers the AI Job Queue for drafting a reply.
  5. Provide Playwright E2E tests demonstrating a simulated conversation flow: creating a contact, starting a conversation, sending a message, and verifying the real-time WebSocket broadcast (or polling fallback) in the UI.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
