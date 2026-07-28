issue_title: "[Research] Architect Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  OHC previously explored using Chatwoot as an external third-party service. However, per the new engineering standards, **Chatwoot as an external dependency is 100% RETIRED**. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust to achieve tight integration with the Customer Identity Resolution Engine, The Ambassador Agent, and our unified workspace.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot (Legacy/Reference):** Chatwoot's architecture relies on Rails, PostgreSQL, Redis, and ActionCable for WebSockets. Its core models include `Account` (Tenant), `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact`. It uses an event-driven architecture for webhooks and agent routing. We must replicate this capability natively in Rust.
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed. By building the chat engine natively in Rust, we eliminate the operational overhead of a separate Ruby application, reduce latency, and ensure strict multi-tenant data isolation within our existing PostgreSQL schema.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway / Webhook Ingress)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      E[Web Widget] -->|WebSocket| B
      B --> F{Message Router / Identity Resolution}
      F -->|Lookup/Create| G[(OHC PostgreSQL DB: Inboxes, Contacts, Conversations, Messages)]
      F --> H[NATS Event Bus: message.created]
      H --> I[The Ambassador Agent]
      I -->|Query Context| G
      I -->|Draft Reply| J[Action Required Queue]
      J --> K[Mobile App Feed 375px]
      K -->|1-Tap Approve| L[Rust Channel Adapters (Outgress)]
      L --> A/C/D/E
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. Clean Apple/Ubiquiti-style hierarchy.

  ### Native Rust Chat Engine Components (Chatwoot Parity)
  1.  **Data Models (PostgreSQL + sqlx):**
      -   `tenant_id` (Row-Level Security enforced on all tables).
      -   `inboxes`: Represents a channel endpoint (e.g., a specific WhatsApp number, an Instagram account, a website widget).
      -   `channels`: Defines the type of inbox (e.g., `channel_whatsapp`, `channel_instagram`, `channel_web_widget`).
      -   `contacts`: The unified customer profile.
      -   `contact_inboxes`: Links a contact to specific channel identities (e.g., their IG handle, their phone number).
      -   `conversations`: A thread of messages between a contact and the business, tied to a specific inbox. Statuses: `open`, `resolved`, `snoozed`.
      -   `messages`: The actual content, sender type (contact, agent, bot), and attachments.
  2.  **API & WebSocket Layer (axum + tokio-tungstenite):**
      -   REST APIs for CRUD operations on Inboxes, Contacts, Conversations, and Messages.
      -   WebSocket endpoints for real-time updates to the OHC Frontend (Mobile/PWA) and Web Widget.
  3.  **Channel Adapters:**
      -   Implementations for verifying and handling webhooks from Meta (Instagram/WhatsApp), Twilio (SMS), etc.
      -   Logic to send outgoing messages back to the respective platform APIs.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Subscribes to `message.created` events. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies. If confidence is high, it can auto-reply. If low or requiring owner approval (configurable), it creates a draft in the `ActionRequiredQueue`.

  ### Key Design Decisions
  - **Single Binary:** Integrating the chat engine directly into the `server` binary simplifies deployment and resource utilization compared to running a separate Chatwoot cluster.
  - **NATS for Eventing:** Use NATS to decouple the webhook ingress from the AI processing and routing logic, ensuring high throughput and resilience.
  - **Strict Multi-Tenancy:** Leverage existing OHC infrastructure for auth and row-level security. Every query must include `tenant_id`.

  # Implementation Prompt
  **User-Facing Outcome:** As an OHC user, I want a unified inbox where I can receive and reply to customer messages from Instagram, WhatsApp, and my website, all within the OHC app, with AI helping me draft responses based on customer history.

  **CUJ & Acceptance Criteria:**
  1.  **Database Migrations:** Create sqlx migrations for the core chat entities: `inboxes`, `channels`, `contacts`, `contact_inboxes`, `conversations`, and `messages`. Ensure all tables have a `tenant_id` column and RLS policies are applied.
  2.  **Rust Data Models & Repositories:** Implement the corresponding Rust structs and sqlx data access layer methods for CRUD operations.
  3.  **Core Services:** Implement a `ConversationService` and `MessageService` to handle the business logic of creating conversations, appending messages, and publishing NATS events (e.g., `ohc.chat.message.created`).
  4.  **API Endpoints (axum):** Expose REST endpoints under `/api/v1/chat/...` for the frontend to fetch conversations and send messages.
  5.  **Dummy Webhook Ingress:** Create a simple unauthenticated `/webhooks/chat/dummy` endpoint that simulates an incoming message from an external channel to test the end-to-end flow.
  6.  **Tests:** Provide comprehensive unit tests for the repositories and services. Ensure 100% test coverage for new code. Provide a basic Playwright E2E test verifying a user can load the chat interface (if UI is implemented, otherwise test the API flow). All tests must pass via `bazel test //...`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
