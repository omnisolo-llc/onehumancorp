issue_title: "Build Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot as an external dependency with a native, high-performance, multi-tenant omnichannel customer support & chat engine written in Rust. Non-technical business owners like Maya, Carlos, Priya, Leo, and Fatima need a unified inbox that brings together Instagram DMs, WhatsApp, SMS, Web Chat, and Emails. The current reliance on an external third-party service introduces data silos, increased latency, complex multi-tenant isolation challenges, and a disjointed UX. They need a single, seamless, mobile-first interface that natively coordinates with AI agents to draft replies, track orders, and manage appointments invisibly in the background.

  ## Research Report & Benchmarking
  An exhaustive audit of the `chatwoot/chatwoot` open-source repository was conducted to benchmark its data models, channels, and core features.

  **Key Findings from Chatwoot:**
  - **Data Models:** Highly relational structure centered around `Account` (Tenant), `Inbox` (Channel Gateway), `Conversation` (Thread), `Message` (Individual payload), and `Contact` (Customer).
  - **Channels Adapter Pattern:** Chatwoot uses a robust adapter pattern (`app/models/channel/*`) to ingest from diverse sources (WhatsApp, API, Twitter, Web Widget, Email).
  - **Real-Time Delivery:** Relies heavily on WebSockets (ActionCable in Ruby) to push updates to the web/mobile clients.
  - **Automation & Macros:** Features built-in SLA policies, automation rules, and canned responses.

  **Competitive & Market Context:**
  - Tools like Shopify Inbox and Wix Inbox are heavily integrated into the commerce flow, providing order context alongside chat. OHC needs a similar commerce-aware native chat.
  - Rust provides an immense advantage for WebSocket concurrency (e.g., using `tokio` and `axum` or `actix-web`) over Ruby on Rails, enabling high scale and lower infrastructure costs while maintaining strict memory safety and multi-tenant data isolation.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_ADAPTER : uses
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| AI_AGENT : processed_by
  ```
  ```mermaid
  sequenceDiagram
      participant Customer (WhatsApp)
      participant OHC Webhook Gateway (Rust)
      participant Channel Adapter
      participant Unified Inbox DB
      participant AI Customer Assistant
      participant Owner Mobile App (Flutter)

      Customer (WhatsApp)->>OHC Webhook Gateway (Rust): Sends Message
      OHC Webhook Gateway (Rust)->>Channel Adapter: Normalize Payload
      Channel Adapter->>Unified Inbox DB: Save Message & Update Conversation
      Unified Inbox DB->>AI Customer Assistant: Trigger AI Draft Generation
      AI Customer Assistant->>Unified Inbox DB: Save AI Draft Reply
      Unified Inbox DB->>Owner Mobile App (Flutter): WebSocket Push (New Message + Draft)
      Owner Mobile App (Flutter)-->>Customer (WhatsApp): Owner approves/sends reply
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Feed (Home):** The owner opens the app to a cleanly structured feed (Translucent Glass UI cards). Unread conversations bubble to the top with clear badges indicating the source channel (WhatsApp icon, Instagram icon).
  2. **Conversation View:** Tapping a conversation opens a chat interface optimized for 375px.
     - **Top:** Customer name, avatar, and quick tags (e.g., "VIP", "Returning").
     - **Middle:** Scrollable chat history.
     - **Bottom:** Input field with a floating "AI Draft" button. If the AI has already drafted a response, it appears as a frosted-glass overlay above the input field with "Approve & Send" or "Edit" actions.
  3. **Commerce Context Drawer:** A swipe-left or tap-on-header reveals a sliding drawer containing the customer's order history, active bookings, and loyalty status, natively pulled from OHC's internal commerce engine.

  ### AI Agent Integration Points
  - **Work Triage:** Intercepts incoming messages, categorizes intent (e.g., "Refund Request", "Custom Cake Inquiry"), and prioritizes the conversation in the owner's feed.
  - **Customer & Relationship Assistant:** Subscribes to the unified inbox event stream. Upon new message creation, it reads the conversation history and customer context (from the commerce drawer) to generate and save a draft reply *before* the owner even opens the app.

  ### Key Design Decisions
  - **Multi-Tenant Isolation:** All database tables (`inboxes`, `conversations`, `messages`, `contacts`) will enforce strict PostgreSQL Row-Level Security (RLS) using `tenant_id`.
  - **Rust Native Engine:** The backend will be built in Rust using a high-performance asynchronous runtime (e.g., Tokio) to handle thousands of concurrent WebSocket connections for real-time messaging.
  - **Adapter Pattern for Channels:** Implement an extensible Rust trait for `ChannelAdapter` to easily add new channels (WhatsApp, Instagram, etc.) without altering the core messaging pipeline.
  - **AI-First, Owner-Approved:** The system is designed to have AI generate drafts seamlessly, but the owner retains 1-tap approval power, keeping them in control.

  ## Implementation Prompt
  **Role:** Backend & Frontend Implementer
  **Task:** Implement the foundational Native Rust Omnichannel Chat system for OHC to replace Chatwoot, and wire it up to the Flutter mobile-first frontend.
  **Critical User Journey (CUJ):**
  1. Maya (the baker) connects a mock "Instagram" channel via the OHC UI.
  2. A mock customer sends a message. The Rust backend receives it, normalizes it, and saves it to the unified inbox.
  3. The AI Customer Assistant intercepts the message and drafts a helpful reply based on Maya's inventory.
  4. Maya opens her OHC app on her phone (375px viewport), sees the unread message in her triage feed, taps it, sees the AI draft, and taps "Approve & Send".

  **Acceptance Criteria:**
  - [ ] Implement the core Rust data models and PostgreSQL schemas (with RLS) for Tenant, Inbox, Conversation, Message, and Contact.
  - [ ] Implement a generic Channel Adapter trait in Rust and one mock implementation for E2E testing.
  - [ ] Build the real-time WebSocket infrastructure in Rust to push updates to the frontend.
  - [ ] Develop the mobile-first (375px) Flutter UI for the unified inbox feed and conversation view, applying the Translucent Glass design tokens.
  - [ ] Integrate the AI Customer Assistant to generate draft replies upon receiving new messages.
  - [ ] Write 100% unit test coverage for the new Rust modules.
  - [ ] Create at least 5 Playwright E2E tests covering the complete CUJ (message reception, AI drafting, owner approval, and sending). ZERO mock data in the UI; everything must flow through the real stack.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
