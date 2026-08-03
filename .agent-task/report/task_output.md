issue_title: "Architecture: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native, fully-integrated omnichannel messaging system, relying on third-party dependencies like Chatwoot which introduces external data silos, latency, and hinders tight integration with OHC's core AI Agent Work Triage. Our owners—like Maya the baker and Carlos the handyman—need a unified inbox where Instagram DMs, SMS, and WhatsApp messages instantly flow into their OHC Assistant without manual synchronization, external dashboards, or complex configuration.

  ## Research Report
  - **Codebase & Docs Audit:** OHC's goal is to keep owners focused on action. External support dashboards break this by forcing the owner out of the AI work command center.
  - **Chatwoot Source Code Audit & Feature Benchmarking:** We audited the open-source Chatwoot repository (models and controllers). Chatwoot's core abstractions include `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `ChannelAdapters`. It relies heavily on WebSockets for real-time delivery, background workers for SLA policies, and a unified controller pattern for ingesting webhooks from different platforms (e.g. Instagram, Twitter, Twilio).
  - **Competitor Systems Audit:** Shopify Inbox and Wix Inbox provide integrated native chat tightly coupled to store inventory and customer profiles. They avoid third-party routing.
  - **Gap Identified:** OHC must replicate Chatwoot’s robust multi-tenant data model and webhook normalization in a highly concurrent native Rust microservice, tightly coupled with our KAIROS AI agents to allow automated triage and response drafting.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      MESSAGE }o--|| AI_DRAFT : has_draft
  ```

  ```mermaid
  sequenceDiagram
      participant ExternalPlatform as Instagram/WhatsApp
      participant WebhookGateway as Rust Webhook API
      participant ChatEngine as Rust Chat Engine
      participant DB as PostgreSQL
      participant AIAgent as Customer Assistant Agent
      participant MobileClient as OHC Mobile UI

      ExternalPlatform->>WebhookGateway: Inbound Message Webhook
      WebhookGateway->>ChatEngine: Normalize & Route
      ChatEngine->>DB: Persist Message & Conversation
      ChatEngine->>AIAgent: Trigger Triage & Draft Reply
      AIAgent->>DB: Save Draft Proposal
      ChatEngine->>MobileClient: WebSocket Event (New Message + Draft)
      MobileClient-->>ChatEngine: Owner Approves Draft
      ChatEngine->>ExternalPlatform: Send Reply
  ```

  ### UI Wireframes / Screen Flow Description (375px First)
  - **Home Dashboard:** Unread indicator on the "Messages" card.
  - **Unified Inbox List:** A vertically scrollable list of conversations. Badges for channel origin (e.g., Insta icon). Translucent glass unread highlighting.
  - **Conversation Thread:** Clean chat UI with bubbles. At the bottom, instead of just an empty text box, an "AI Draft" card is presented if the AI has proposed a reply. The owner can tap "Approve & Send", "Edit", or type manually.

  ### Mobile UX Flow
  1. Owner receives push notification of new Instagram DM.
  2. Taps notification, opening the OHC app to the conversation.
  3. Sees the customer message: "Do you have vegan cakes available for tomorrow?"
  4. Sees AI proposed reply drafted based on inventory and previous customer context.
  5. Owner taps "Approve & Send" with a single 44x44px touch target.
  6. Success state is instantly shown locally, with a subtle "Sent" receipt indicating external platform confirmation.

  ### AI Agent Integration Points
  - **Work Triage:** A background AI agent subscribes to the `message.created` event queue. It analyzes inbound intent.
  - **Customer Assistant:** Automatically fetches past order history from OHC's database, synthesizes a reply, and writes it as an `ai_draft` record attached to the `Message` model.
  - **Distributed Locks:** Redis Redlock (`ohc:lock:{tenant_id}:conversation:{id}`) prevents multiple agents or background jobs from drafting replies concurrently.

  ### Key Design Decisions
  - **Native Rust Implementation:** Replacing Ruby/Rails (Chatwoot) with Rust allows predictable memory footprints, better concurrency for WebSocket connections, and high-performance parsing of massive webhook volumes.
  - **Row-Level Security (RLS):** Every table (`inboxes`, `conversations`, `messages`) uses strictly enforced PostgreSQL RLS keyed on `tenant_id` to guarantee tenant isolation, crucial for a SaaS multi-tenant platform.
  - **Unified Event Bus:** Using PostgreSQL `SKIP LOCKED` or Redis streams to decouple message ingestion from AI agent processing, ensuring webhook endpoints respond within milliseconds regardless of LLM generation times.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner can receive and reply to customer messages from Instagram, SMS, and web chat directly within the OHC app, with AI agents automatically drafting context-aware replies.
  **CUJ:** Owner opens the app, sees a new unified message from a customer, reviews the AI-generated draft, and taps "Approve & Send" to instantly deliver the reply.
  **Acceptance Criteria:**
  - Rust API implemented for webhook ingestion, normalizing to a standard `Conversation` and `Message` data model.
  - PostgreSQL schema created with strict `tenant_id` RLS policies.
  - AI Customer Assistant successfully triggers on new messages and generates drafts.
  - Mobile-first UI implemented matching the translucent glass design system, with full touch targets and offline-tolerant reads.
  - Full end-to-end E2E Playwright test simulating an inbound webhook and owner approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
