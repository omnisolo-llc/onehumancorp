issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Title**: Implement Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement**:
  OneHumanCorp (OHC) currently lacks a native, fully integrated omnichannel chat and customer support engine. To serve small business owners like Maya (baker) and Carlos (handyman) seamlessly, we must retire our reliance on third-party external services like Chatwoot. The external dependency breaks our strict multi-tenant Zero Trust architecture, creates fragmented data silos, and prevents our AI agents (like the Customer & Relationship Assistant) from deeply embedding into real-time conversation flows. We need a native Rust implementation that brings Chatwoot-level capabilities inside OHC's mono-repo, providing real-time WebSocket messaging, omnichannel routing, and native AI integration without technical complexity for the user.

  **Research Report**:
  - **Chatwoot Architecture Audit**: Analyzed the Chatwoot Ruby on Rails codebase (`https://github.com/chatwoot/chatwoot`).
    - Key data models identified: `conversations`, `messages`, `inboxes`, `contacts`, `channel_*` (e.g., web widget, API, email, social).
    - Real-time event architecture relies heavily on ActionCable (WebSockets) for message broadcasting.
    - Routing and SLA policies use background workers (Sidekiq) to manage agent assignment and capacity.
  - **OHC Migration Audit**: OHC has initial schema primitives in `1009_native_omnichannel_chat.sql` (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`), but lacks the Rust application layer (controllers, WebSocket engine, AI department coordination).
  - **Competitive Landscape**: Shopify Inbox and Wix Chat provide deeply integrated, native chat capabilities that reduce friction. OHC's native chat must follow this pattern, ensuring a seamless 375px mobile experience where an owner can view an order and reply to the customer in the same screen.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        TENANT ||--o{ CHAT_INBOXES : "owns"
        CHAT_INBOXES ||--o{ CHAT_CHANNELS : "contains"
        CHAT_INBOXES ||--o{ CHAT_CONVERSATIONS : "has"
        CHAT_CONTACTS ||--o{ CHAT_CONVERSATIONS : "participates_in"
        CHAT_CONVERSATIONS ||--o{ CHAT_MESSAGES : "contains"
        AGENT ||--o{ CHAT_MESSAGES : "sends"
        AI_DEPARTMENT ||--o{ CHAT_MESSAGES : "drafts/replies"
    ```
    - The core backend will be a native Rust service integrated via gRPC/REST.
    - A WebSocket (Axum/Tokio) real-time streaming service will handle live chat updates, replicating Chatwoot's ActionCable functionality.
    - AI agents will listen to the event stream via PostgreSQL `SKIP LOCKED` job queues to auto-draft replies for incoming messages based on tenant context.

  - **Mobile UX Flow (375px first)**:
    - **Triage Feed**: Owner opens OHC app on phone and sees a unified "Triage Feed". Unread messages appear as premium, Translucent Glass cards.
    - **Conversation View**: Tapping a card opens a chat view natively. Native mobile keyboards are fully supported without horizontal scrolling.
    - **AI Drafts**: If the Customer Assistant AI has prepared a response, it appears as a floating, frosted-glass "Draft" bubble above the input field. The owner taps "Approve & Send" or edits the draft.
    - **Customer Context**: Tapping the customer's avatar slides up a bottom sheet showing their history, lifetime value, and past appointments/orders.

  - **AI Agent Integration Points**:
    - **Work Triage**: Unifies new conversations with system alerts.
    - **Customer Assistant**: Subscribes to new `chat_messages` events. Uses Gemini Pro to draft replies, saving them as `draft` messages pending owner approval.
    - **Operations/Sales**: AI can parse messages (e.g., "I want 3 vegan cakes for Tuesday") and automatically suggest creating a quote or booking.

  **Implementation Prompt**:
  *Objective*: Implement the backend Rust services and Flutter frontend components for the Native Omnichannel Chat System based on the `chat_*` database schemas.
  *CUJ (Critical User Journey)*: Maya receives an Instagram DM from a customer. It appears in her OHC mobile app. She views the message, sees an AI-drafted reply, approves it, and sends it back seamlessly.
  *Acceptance Criteria*:
  1. Develop native Rust API endpoints for CRUD operations on inboxes, conversations, and messages.
  2. Implement a robust WebSocket streaming engine in Rust for real-time message delivery to the Flutter frontend.
  3. Create the Flutter mobile UI (375px optimized) with a Triage Feed and Conversation View using OHC Premium Token designs (Translucent Glass).
  4. Ensure all database access enforces row-level security (RLS) for tenant isolation.
  5. Include full Playwright E2E tests verifying the end-to-end chat flow without mock data.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []