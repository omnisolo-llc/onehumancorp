issue_title: "[RESEARCH] Architect Native Rust Omnichannel Inbox & Customer Identity Resolution"
issue_description: |
  # Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) receive customer inquiries across fragmented channels (Instagram DMs, WhatsApp, SMS, Email). Managing these manually causes missed leads, delayed responses, and lost revenue. Previously, OHC evaluated third-party solutions like The Previously Evaluated Third-Party Solution, but the **mandate is now 100% complete The Previously Evaluated Third-Party Solution retirement**. We must build a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust. This native system will not just aggregate messages but deeply integrate with OHC's AI agents (The Ambassador) to proactively draft contextual, identity-aware responses, shifting the owner's workload from manual typing to one-tap approval.

  # Research Report
  **Findings & Competitive Analysis:**
  - **The Previously Evaluated Third-Party Solution Source Audit:** The Previously Evaluated Third-Party Solution's architecture relies heavily on separate channel adapters (Facebook, Twitter, Web Widget, API), a central Inbox model, and Conversation/Message models. It uses WebSockets for real-time updates and ActionCable (Rails). For OHC's native Rust implementation, we need an equivalent but more performant architecture:
      - **Channel Adapters:** Rust traits/services to handle incoming webhooks (WhatsApp Cloud API, Meta Graph API, SendGrid/Postmark).
      - **Data Model:** `Tenant` -> `Inbox` -> `Conversation` -> `Message`. Crucially, we need a strong `Contact` (Customer Identity) model that links to previous OHC orders/bookings.
      - **Real-time:** Rust async WebSockets (e.g., using `tokio` and `axum` or `actix-web`) for instant UI updates.
  - **Shopify Inbox & Wix Inbox:** Aggregates well but lacks proactive AI drafting based on full multi-channel history.
  - **OHC Native Advantage:** By building in Rust within the `onehumancorp/mono` repo, we achieve strict tenant data isolation (Row Level Security equivalent in our Rust data layer), zero network hops to our core CRM/Inventory data, and deep integration with our AI Job Queue for autonomous drafting.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp/IG/SMS Webhooks] -->|Ingress| B(Rust API Gateway / axum)
      B --> C{Channel Adapters & Parsers}
      C --> D[Customer Identity Resolution Engine]
      D -->|Lookup/Create| E[(Native CRM Database - PostgreSQL)]
      D --> F[Rust Inbox Controller]
      F -->|Save Message| E
      F --> G[Real-time WebSocket Hub]
      G -->|Broadcast| H[Flutter Web/Mobile App]
      F --> I[AI Agent Job Queue - PostgreSQL SKIP LOCKED]
      I --> J[The Ambassador Agent Worker]
      J -->|Draft Reply| F
      J -->|Push Notification| H
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Command Center (375px):** The primary view is NOT a traditional complex ticketing inbox. It is an "Action Required" feed.
  - **Card View:** A translucent glass card displays: "New Inquiry: Maya (Instagram)".
  - **Tap to Expand:** Shows the customer's identity context (e.g., "Past Customer: 3 Orders, Last order: Vegan Cake"). Below this, the full thread history is displayed.
  - **AI Proactive Draft:** A visually distinct section shows the AI-drafted reply (e.g., "Hi Maya! Yes, we can do a vegan cake for Saturday. Shall I send a deposit link?").
  - **Interaction:** A prominent primary button "Approve & Send". A secondary button "Edit Draft".
  - **Real-time:** The UI updates instantly via WebSockets when new messages arrive or drafts are ready.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Listens to the AI Job Queue for new `ConversationMessage` events. It uses tenant-scoped memory and RAG against the business's data (orders, FAQs) to draft highly accurate replies and updates the database, triggering a WebSocket event to the frontend.

  ### Key Design Decisions
  - **Native Rust Implementation:** Complete removal of any external The Previously Evaluated Third-Party Solution dependency. High-performance, zero-trust isolated multi-tenant design.
  - **Identity-First:** Every incoming message MUST run through an Identity Resolution step to tie it to a unified `Contact/Customer` record before hitting the inbox, enabling rich context for the AI.
  - **Draft-First UX:** The UI optimizes for the owner approving AI work, not manually typing replies.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner (e.g., Carlos), when a lead messages my WhatsApp asking for a quote, I get a notification on my 375px Android phone. I open the OHC app, see the message, and instantly see a drafted reply from my AI assistant with a link to book a consultation. I tap "Approve" and it sends instantly.

  **CUJ & Acceptance Criteria:**
  1.  **Backend Data Model:** Implement Rust structs and database migrations for `Contact`, `Inbox`, `Conversation`, and `Message` entities, ensuring strict multi-tenant isolation.
  2.  **API & Webhook Ingestion:** Implement a generic webhook ingestion endpoint in Rust (`axum`/`actix`) that can parse standard message payloads and route them to the correct Tenant Inbox.
  3.  **Identity Resolution:** Implement a service that attempts to match incoming sender identifiers (phone, email, social ID) to existing `Contact` records, or creates a new one.
  4.  **WebSocket Real-time:** Implement a WebSocket server in Rust that broadcasts new messages and AI drafts to authenticated frontend clients for a specific tenant.
  5.  **AI Drafting Job:** Implement a worker that picks up new messages, queries the LLM with customer context, and saves a "draft" message to the conversation.
  6.  **E2E Playwright Verification:**
      - A script simulates a webhook POST with a new customer message.
      - Playwright logs into the OHC Flutter/PWA UI as the business owner.
      - Verifies the message appears in the feed (via WebSocket or polling).
      - Verifies the AI draft appears.
      - Clicks "Approve & Send".
      - Verifies the backend marks the message as sent.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, native-chat]
assignees: []
