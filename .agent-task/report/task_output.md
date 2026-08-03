issue_title: "Architecture: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol Brief

  ### Problem Statement
  OHC has heavily relied on Chatwoot as a third-party dependency for its omnichannel inbox functionality. However, Chatwoot's architecture adds significant operational overhead, breaks our native multi-tenancy model (PostgreSQL RLS), complicates the Zero-Trust security posture, and introduces unacceptable latency for AI agent real-time intervention. For personas like Maya (the baker fielding Instagram DMs) and Carlos (the handyman replying to service queries), the chat interface needs to be instant, offline-tolerant, and deeply integrated into the OHC AI work assistant. We need a native, high-performance omnichannel engine built in Rust to replace Chatwoot completely.

  ### Research Report
  - **Codebase & Docs Audit:** Chatwoot uses Ruby on Rails with complex database schemas (Accounts, Conversations, Messages, Contacts, Inboxes). It handles WebSocket real-time messaging and channel integrations (WhatsApp, FB, Twitter, Email).
  - **Competitor Systems Audit:** Systems like Shopify Ping, Stripe Customer Portal, and Intercom use edge-optimized real-time messaging with tightly coupled AI copilot features.
  - **Gap Identification:** OHC requires an omnichannel chat architecture built natively in Rust that enforces PostgreSQL RLS (Row-Level Security) natively for multi-tenancy, handles WebSockets directly via OHC's API gateway, and integrates seamlessly with our AI Agent background job queues.
  - **Chatwoot Source Benchmarking:** Analyzed `conversations`, `messages`, `contacts`, and `inboxes` schemas in Chatwoot. We will replicate the unified conversational model but using native Rust structs, Diesel/SQLx, and our established `tenant_id` RLS patterns.

  ### Architecture Design
  - **Architecture Diagram (Mermaid.js)**
    ```mermaid
    erDiagram
        Tenant ||--o{ Inbox : owns
        Inbox ||--o{ Conversation : contains
        Conversation ||--o{ Message : has
        Conversation ||--o{ Contact : involves
        Tenant ||--o{ Contact : owns
        Message }|--|| ChannelAdapter : delivered_via
    ```
    ```mermaid
    sequenceDiagram
        actor Customer
        participant ChannelAdapter (Rust)
        participant API Gateway (Rust/gRPC)
        participant ChatEngine (Rust)
        participant AI Triage Agent
        participant Owner (Mobile PWA)

        Customer->>ChannelAdapter: Sends Instagram DM
        ChannelAdapter->>API Gateway: Webhook Payload
        API Gateway->>ChatEngine: Create Message & Route to Inbox
        ChatEngine->>AI Triage Agent: Trigger Background Job (Postgres SKIP LOCKED)
        AI Triage Agent->>ChatEngine: Generate Draft/Auto-reply
        ChatEngine-->>Owner (Mobile PWA): WebSocket Push (New Message & Draft)
        Owner (Mobile PWA)->>ChatEngine: Approve Draft
        ChatEngine->>ChannelAdapter: Send via Instagram Graph API
    ```
  - **Mobile UX Flow (375px First)**
    - **Unified Inbox:** A clean, bottom-nav tab for "Messages". Shows a unified list of active conversations.
    - **Conversation View:** iMessage-like UI, showing customer context at the top. Below, a persistent AI Copilot bar proposes the "Next Best Action" (e.g., "Draft custom cake quote").
    - **Offline Tolerance:** Messages drafted while offline are queued locally and sync when back online.
    - **Visuals:** MacOS-style Translucent Glass app bar. Clean Ubiquiti UniFi style message bubbles.
  - **AI Agent Integration Points**
    - `AI Operations Agent` listens to `ConversationCreated` events to triage and categorize priority.
    - `AI Customer Assistant` monitors `MessageCreated` events to draft contextual replies based on previous `Contact` history and business `Knowledge` (e.g., policies).

  ### Implementation Prompt
  **User-Facing Outcome:** Owners like Maya and Carlos can view, triage, and reply to all customer messages (Instagram, WhatsApp, Email, Web Chat) from a single, native mobile-first inbox within OHC. AI agents automatically draft replies and suggest quotes based on the conversation context.

  **The CUJ (Critical User Journey):**
  1. Maya opens OHC on her iPhone (375px viewport).
  2. She navigates to the "Inbox" tab and sees a new Instagram DM from a customer asking about a vegan cake.
  3. The AI Customer Assistant has already drafted a reply based on her knowledge base: "Hi! Yes, we can make any of our cakes vegan. Would you like a quote for the 8-inch strawberry?"
  4. Maya taps "Send" on the AI draft.
  5. The message is dispatched natively through the OHC Rust Chat Engine to Instagram, and the conversation is marked "Waiting for Customer".

  **Acceptance Criteria:**
  - Build the core Chat Engine Rust module `ohc-chat-engine` inside `src/server/ohc/`.
  - Implement models for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` RLS.
  - Expose internal gRPC APIs for creating conversations and messages.
  - Create a WebSocket service adapter for real-time delivery to the Flutter frontend.
  - Write unit tests for all models and API endpoints achieving 100% coverage.
  - Provide one E2E Playwright test simulating an owner opening the inbox, viewing a mock injected conversation, and sending a reply.
  - Replace all remaining Chatwoot API calls with the new native Rust API.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
