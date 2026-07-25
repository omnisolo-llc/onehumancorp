issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a fully integrated, high-performance, and native omnichannel customer support system. Relying on an external third-party service like Chatwoot introduces multi-tenancy risks, network latency, Zero Trust/SPIFFE integration challenges, and breaks the core promise of a unified, native "Owner Work Assistant." Business owners like Maya (the baker responding to Instagram DMs) or Carlos (the handyman fielding SMS quote requests) need instant, unified triage of messages across channels (WhatsApp, Instagram, SMS, Email, Web Widget) directly within the OHC platform. We must retire Chatwoot completely and build a native, scalable omnichannel engine in Rust, deeply embedded into OHC's multi-tenant architecture and AI agent workflows.

  ## Research Report
  - **Competitor Analysis:**
    - *Chatwoot*: Provides robust inbox management, macro support, and multiple channel integrations (WhatsApp, FB, Twitter, Line, SMS). Its architecture relies on a traditional Rails/Postgres stack.
    - *Shopify Inbox / Inbox by Zendesk*: High emphasis on commerce integration—showing order history natively alongside the conversation.
  - **Codebase Insights:**
    - After cloning and analyzing the `chatwoot/chatwoot` source repository, key entities to replicate include: `Conversation`, `Message`, `Contact`, `Inbox`, and `ChannelAdapter` (e.g., `Channel::Whatsapp`, `Channel::Sms`).
    - Chatwoot's real-time messaging heavily relies on ActionCable (WebSockets). In OHC, this will be replaced with Tokio-based WebSockets or gRPC streams within the Rust `ohc-mono` backend.
  - **Strategic Value:**
    - A native Rust system ensures row-level multi-tenant isolation out-of-the-box using our existing Postgres RLS schema.
    - Zero data egress to third-party chat platforms.
    - Enables our internal AI departments (Operations, CS, Sales) to seamlessly read chat context from memory and inject drafted responses directly into the unified data model.

  ## Design Doc

  ### 1. High-Level Architecture (Mermaid.js)
  ```mermaid
  graph TD
    Client[Mobile/Web App 375px UI] -->|WSS / gRPC| API[Rust API Gateway - axum/tonic]
    API --> ChatEngine[Native Rust Chat Engine]

    subgraph "Omnichannel Chat Engine"
      ChatEngine --> InboxService[Inbox Service]
      ChatEngine --> MsgService[Message Service]
      ChatEngine --> ChannelAdapters[Channel Adapters]

      ChannelAdapters --> |Meta API| WhatsApp[WhatsApp WABA]
      ChannelAdapters --> |Twilio/Plivo| SMS[SMS]
      ChannelAdapters --> |IMAP/SMTP| Email[Email]
      ChannelAdapters --> |WebSockets| WebWidget[Web Widget]
    end

    MsgService --> |PG RLS| DB[(PostgreSQL)]
    MsgService --> |Redis PubSub| Realtime[Real-time Events]
    MsgService --> |Queue| AIWorker[AI Draft Worker]

    AIWorker --> |Context| LlmAgent[Gemini/GPT Agent]
    LlmAgent --> |Drafts Reply| MsgService
  ```

  ### 2. Mobile UX Flow (375px Viewport)
  - **Screen 1: Unified Work Feed (Triage):**
    - A single, thumb-friendly list view showing new messages, tasks, and system alerts.
    - Translucent Glass materials on iOS.
    - Badges indicate channel source (e.g., green WhatsApp icon, purple Instagram icon).
  - **Screen 2: Conversation View:**
    - Standard chat bubble interface with at least 44x44px touch targets for attachments/sending.
    - **Crucial Addition:** An "AI Draft" translucent overlay that pops up suggesting a response based on business context (e.g., "Draft: Yes, we do vegan cakes. Deposit link: [Link]"). Owner can single-tap to approve and send.
  - **Screen 3: Customer Context Drawer:**
    - Swiping left reveals the customer's profile, lifetime value, active orders, and custom notes.

  ### 3. AI Agent Integration Points
  - **Customer & Relationship Assistant:** Subscribes to the `MessageCreated` event via NATS/Redis. When a new message arrives in an inbox, the AI agent pulls context from the `Contact` and `Order` tables, generates a draft reply, and saves it to the `Message` table with status `draft`.
  - **Operations Assistant:** Parses incoming text (e.g., "I need a quote for fixing a pipe on Tuesday") to automatically propose an appointment slot and a draft quote in the UI for owner approval.

  ## Implementation Prompt
  **User-Facing Outcome:** Business owners can open the OHC app and see all customer inquiries (SMS, Web, IG) in a single unified inbox. They receive AI-drafted replies instantly and can approve/send them with one tap, without ever leaving the app or managing a separate tool.
  **Critical User Journey (CUJ):**
  1. Customer sends an SMS.
  2. Owner opens OHC and sees a unified notification.
  3. Owner opens the conversation.
  4. Owner sees an AI-suggested draft (e.g., "Hi, here is our pricing for plumbing...").
  5. Owner taps "Approve & Send".
  6. Customer receives the SMS.
  **Acceptance Criteria:**
  - Build a native Rust module in `src/server/ohc/chat` replacing Chatwoot dependencies.
  - Implement core models: `Inbox`, `Conversation`, `Message`, `Contact`.
  - Ensure strict tenant isolation via PostgreSQL RLS (`tenant_id`).
  - Create the Flutter/Web UI matching the Translucent Glass 375px mobile-first design.
  - Wire up the AI Draft worker to automatically suggest responses.
  - E2E Playwright test must cover the full loop: Incoming mock message -> AI Draft generation -> Owner Approval -> Outbound message sent.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
